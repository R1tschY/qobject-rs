#[macro_use]
extern crate syn;

use crate::arguments::MetaArgs;
use crate::diagnostic::DiagnosticExt;
use proc_macro2::Ident;
use qobject_compiler::qobject::QObjectConfig;
use qobject_compiler::typeref::TypeRefTrait;
use qobject_compiler::{Include, QObjectMethod, TypeRef};
use quote::quote;
use std::collections::HashMap;
use std::mem;
use std::ops::Deref;
use syn::export::{Span, TokenStream, TokenStream2};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, FnArg, ImplItem, PathArguments, PathSegment, ReturnType, Type,
};

mod arguments;
mod diagnostic;

#[derive(Debug)]
enum QObjectDeriveError {
    InternalError(String),
    IllegalSyntax(syn::Error),
    IllegalInput(String, Span),
}

impl From<syn::Error> for QObjectDeriveError {
    fn from(err: syn::Error) -> Self {
        Self::IllegalSyntax(err)
    }
}

type QObjectDeriveResult<T> = Result<T, QObjectDeriveError>;

fn codegen_errors(err: QObjectDeriveError) -> TokenStream2 {
    match err {
        QObjectDeriveError::InternalError(message) => Span::call_site().error(message).emit(),
        QObjectDeriveError::IllegalInput(message, span) => span.error(message).emit(),
        QObjectDeriveError::IllegalSyntax(err) => err.to_compile_error(),
    }
}

#[proc_macro_attribute]
pub fn qobject(args: TokenStream, input: TokenStream) -> TokenStream {
    let impl_block: syn::ItemImpl = parse_macro_input!(input);
    match parse(args, impl_block) {
        Err(err) => codegen_errors(err),
        Ok(tokens) => tokens,
    }
    .into()
}

enum QObjectItem {
    Slot {
        sig: syn::Signature,
    },
    Signal {
        sig: syn::Signature,
    },
    Property {
        span: Span,
        ty: Box<syn::Type>,
        write: Option<String>,
        notify: Option<String>,
        const_: bool,
    },
    Method {
        sig: syn::Signature,
        invokable: bool,
        override_: bool,
    },
}

fn illegal_input(message: &str, span: impl Spanned) -> QObjectDeriveError {
    QObjectDeriveError::IllegalInput(message.to_string(), span.span())
}

fn get_single_path_segment(path: &syn::Path) -> Option<&PathSegment> {
    let segments = &path.segments;
    if segments.len() == 1 {
        Some(&segments[0])
    } else {
        None
    }
}

fn parse_attributes(method: &mut syn::ImplItemMethod) -> QObjectDeriveResult<Vec<QObjectItem>> {
    let (attrs, qattrs): (Vec<Attribute>, Vec<Attribute>) = mem::replace(&mut method.attrs, vec![])
        .into_iter()
        .partition(|attr| {
            if let Some(segment) = get_single_path_segment(&attr.path) {
                let ident = &segment.ident;
                ident == "slot" || ident == "signal" || ident == "property" || ident == "method"
            } else {
                false
            }
        });

    let qattrs: QObjectDeriveResult<Vec<QObjectItem>> = qattrs
        .into_iter()
        .map(|attr| {
            let span = attr.span();
            let segments = &attr.path.segments;
            if segments[0].arguments != PathArguments::None {
                return Err(illegal_input("path arguments on attribute", span));
            }

            let args: MetaArgs = parse_macro_input::parse(attr.tokens.into())?;

            let ident = &segments[0].ident;
            if ident == "slot" {
                let mut const_ = false;

                for arg in args.args {
                    if arg.name == "const" {
                        if arg.value.is_some() {
                            return Err(illegal_input("no argument expected", span));
                        }
                        const_ = true;
                    } else {
                        return Err(illegal_input("unknown argument", span));
                    }
                }

                Ok(QObjectItem::Slot {
                    sig: method.sig.clone(),
                })
            } else if ident == "signal" {
                if !method.block.stmts.is_empty() {
                    return Err(illegal_input(
                        "block of signal must be empty",
                        &method.block,
                    ));
                }

                Ok(QObjectItem::Signal {
                    sig: method.sig.clone(),
                })
            } else if ident == "property" {
                let ty = match &method.sig.output {
                    ReturnType::Default => {
                        return Err(illegal_input(
                            "missing return type for property",
                            &method.sig.output,
                        ))
                    }
                    ReturnType::Type(_, ty) => ty.clone(),
                };

                Ok(QObjectItem::Property {
                    span: ident.span(),
                    ty,
                    write: None,
                    notify: None,
                    const_: false,
                })
            } else if ident == "method" {
                Ok(QObjectItem::Method {
                    sig: method.sig.clone(),
                    invokable: false,
                    override_: false,
                })
            } else {
                unreachable!()
            }
        })
        .collect();

    method.attrs = attrs;
    qattrs
}

fn get_ident_ty_from_fnarg(arg: &syn::PatType) -> QObjectDeriveResult<(&Ident, &Type)> {
    let ident = match arg.pat.deref() {
        syn::Pat::Ident(identpat) => &identpat.ident,
        _ => {
            return Err(illegal_input(
                "unsupported pattern in function argument",
                &arg.pat,
            ))
        }
    };
    Ok((ident, &arg.ty))
}

fn get_type_path(ty: &Type) -> QObjectDeriveResult<&syn::Path> {
    match ty {
        syn::Type::Path(syn::TypePath { qself, path }) if qself.is_none() => Ok(path),
        _ => Err(illegal_input("expected a path", ty)),
    }
}

enum Wrapper {
    No,
    Ref(bool),
    Ptr(bool),
}

fn resolve_type_ref(ty: &Type) -> QObjectDeriveResult<TypeRef> {
    let (wrapper, path) = match ty {
        syn::Type::Reference(syn::TypeReference {
            lifetime,
            mutability,
            elem,
            ..
        }) => {
            if lifetime.is_some() {
                return Err(illegal_input("explicit lifetimes not supported", lifetime));
            }

            (Wrapper::Ref(mutability.is_some()), get_type_path(&elem)?)
        }
        syn::Type::Ptr(syn::TypePtr {
            mutability, elem, ..
        }) => (Wrapper::Ptr(mutability.is_some()), get_type_path(&elem)?),
        ty => (Wrapper::No, get_type_path(ty)?),
    };
    let segs: QObjectDeriveResult<Vec<String>> = path
        .segments
        .iter()
        .map(|seg| {
            if seg.arguments == PathArguments::None {
                Err(illegal_input("unexpected path arguments", &seg.arguments))
            } else {
                Ok(seg.ident.to_string())
            }
        })
        .collect();
    let path_ref = segs?.join("::");

    let mut types = HashMap::<&'static str, TypeRef>::new();
    types.insert("QObject", TypeRef::qobject());
    types.insert("QString", TypeRef::qstring());
    types.insert("QByteArray", TypeRef::qt_core_object("QByteArray"));
    types.insert("QModelIndex", TypeRef::qt_core_object("QModelIndex"));
    types.insert("QVariant", TypeRef::qt_core_object("QVariant"));
    types.insert(
        "QHashIntQByteArray",
        TypeRef::new(
            "QHash<int, QByteArray>",
            "qt5qml::core::QHashIntQByteArray",
            false,
            Some(Include::System("QHash".to_string())),
        ),
    );
    types.insert("i8", i8::type_ref());
    types.insert("u8", u8::type_ref());
    types.insert("i16", i16::type_ref());
    types.insert("u16", u16::type_ref());
    types.insert("i32", i32::type_ref());
    types.insert("u32", u32::type_ref());
    types.insert("i64", i64::type_ref());
    types.insert("u64", u64::type_ref());
    types.insert("f32", f32::type_ref());
    types.insert("f64", f64::type_ref());
    types.insert("bool", bool::type_ref());

    let type_ref = if let Some(type_ref) = types.get(&(&path_ref as &str)) {
        type_ref.clone()
    } else {
        return Err(illegal_input("unknown type", &ty));
    };

    Ok(match wrapper {
        Wrapper::No => type_ref,
        Wrapper::Ref(false) => type_ref.with_const_ref(),
        Wrapper::Ref(true) => type_ref.with_mut_ref(),
        Wrapper::Ptr(false) => type_ref.with_const_ptr(),
        Wrapper::Ptr(true) => type_ref.with_mut_ptr(),
    })
}

fn parse(_args: TokenStream, mut impl_block: syn::ItemImpl) -> QObjectDeriveResult<TokenStream2> {
    if let Some((_, path, _)) = impl_block.trait_ {
        return Err(illegal_input("No trait impl allowed", path.span()));
    }

    let mut qitems: Vec<QObjectItem> = vec![];
    for mut item in &mut impl_block.items {
        if let ImplItem::Method(ref mut method) = &mut item {
            qitems.extend(parse_attributes(method)?);
        }
    }
    let class_name = match impl_block.self_ty.deref() {
        Type::Path(syn::TypePath { qself, path })
            if qself.is_none()
                && path.segments.len() == 1
                && path.segments[0].arguments == PathArguments::None =>
        {
            path.segments[0].ident.to_string()
        }
        _ => {
            return Err(illegal_input(
                "self type must only be a identifier",
                impl_block.self_ty,
            ))
        }
    };

    let mut qobject: QObjectConfig = QObjectConfig::new(&class_name);
    for item in qitems {
        match item {
            QObjectItem::Slot { sig } => {
                let mut slot = QObjectMethod::new(&sig.ident.to_string());
                let mut mutability: Option<bool> = None;
                for arg in &sig.inputs {
                    match arg {
                        FnArg::Receiver(rec) => {
                            if let Some((_, lifetime)) = &rec.reference {
                                if lifetime.is_some() {
                                    return Err(illegal_input(
                                        "explicit lifetime not supported",
                                        lifetime,
                                    ));
                                }
                            } else if rec.reference.is_none() {
                                return Err(illegal_input("self type must be a reference", rec));
                            }

                            mutability = Some(rec.mutability.is_some());
                        }
                        FnArg::Typed(arg) => {
                            let (ident, ty) = get_ident_ty_from_fnarg(&arg)?;
                            let type_ref = resolve_type_ref(ty)?;
                            slot = slot.arg_with_type(&ident.to_string(), type_ref);
                        }
                    }
                }

                if let Some(mutability) = mutability {
                    if !mutability {
                        slot = slot.const_();
                    }
                } else {
                    return Err(illegal_input("static methods not supported", sig));
                }

                qobject.slot(slot);
            }
            _ => {}
        }
    }

    Ok(quote! {
        #impl_block
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
