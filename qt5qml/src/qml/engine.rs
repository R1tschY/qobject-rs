#[repr(C)]
pub struct QQmlEngine(pub(crate) crate::ffi::QQmlEngine);
impl_ffi_trait!(QQmlEngine);
impl_qobject_ref!(QQmlEngine);
