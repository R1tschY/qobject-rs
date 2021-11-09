from __future__ import annotations

import subprocess
import sys
from enum import Enum
from pathlib import Path
from typing import List, Dict, Optional, Union, Tuple

import yaml
from jinja2 import Environment, FileSystemLoader
from pydantic import BaseModel, Field


class MethodKind(str, Enum):
    Trivial = "trivial"


class QList(BaseModel):
    cpp: str
    rs: str


class Method(BaseModel):
    params: Dict[str, str] = Field(default_factory=dict)
    return_: Optional[str] = Field(default="void", alias="return")
    body: str
    const: bool = False
    static: bool = False


class Class(BaseModel):
    dtor: Union[bool, MethodKind] = True
    default_ctor: Union[bool, MethodKind] = Field(default=False, alias="default-ctor")
    copy_ctor: Union[bool, MethodKind] = Field(default=False, alias="copy-ctor")
    copy_assign: Union[bool, MethodKind] = Field(default=False, alias="copy-assign")
    movable: bool = False
    eq: bool = False
    ord: bool = False

    layout: Dict[str, str] = Field(default_factory=dict)
    methods: Dict[str, Method] = Field(default_factory=dict)
    qobject: bool = False
    qobject_default_ctor: bool = Field(default=False, alias="qobject-default-ctor")
    overwrite_include: Optional[str] = Field(default=None, alias="overwrite-include")
    overwrite_name: Optional[str] = Field(default=None, alias="overwrite-name")

    @property
    def generate_dtor(self) -> bool:
        if self.qobject:
            # Use QObject dtor
            return False

        return self.dtor is not True


class BindgenConfig(BaseModel):
    prelude: Optional[str]
    classes: Dict[str, Class]
    includes: List[str] = Field(default_factory=list)
    qlists: Dict[str, QList] = Field(default_factory=dict)


class Check:
    _config: BindgenConfig
    _cls_name: Optional[str]
    _cls: Optional[Class]
    _errors: List[Tuple[str, Class, str, Method, str]]

    def __init__(self):
        self._cls = None
        self._cls_name = None
        self._errors = []

    def add_method_error(self, name: str, method: Method, message: str):
        self._errors.append((self._cls_name, self._cls, name, method, message))

    def check_method(self, name: str, method: Method):
        pass

    def check_class(self, name: str, cls: Class):
        self._cls = cls
        self._cls_name = name

        for name, method in cls.methods.items():
            self.check_method(name, method)

    def check_config(self, config: BindgenConfig):
        self._config = config
        for name, cls in config.classes.items():
            self.check_class(name, cls)

        return self._errors


def check_config(checks: List[Check], config: BindgenConfig):
    errors = []
    for check in checks:
        errors.extend(check.check_config(config))

    if errors:
        for error in errors:
            print(f"error: {error[0]}.{error[2]}: {error[4]}")
        sys.exit(1)


class ReturnNonTrivialClass(Check):
    def check_method(self, name: str, method: Method):
        if method.return_ is not None:
            rty = self._config.classes.get(method.return_)
            if rty is not None and rty.dtor != MethodKind.Trivial:
                self.add_method_error(
                    name, method, f"C API does not allow to return non-trivially C++ type {method.return_}")


def handle_qlist(name: str, ty: QList):
    return {
        "overwrite-include": "QList",
        "default-ctor": "true",
        "copy-ctor": "true",
        "copy-assign": "true",
        "movable": "true",
        "eq": "true",

        "layout": dict(__d="void*"),

        "methods": {
            "size": {
                "const": True,
                "return": "int",
                "body": "return self->size();"
            },
            "asSlice": {
                "const": True,
                "params": {
                    "size": "int*"
                },
                "return": f"{ty.cpp} const*",
                "body": "*size = self->size(); if (size == 0) { return nullptr; } else { return & self->front(); }"
            },
            "append": {
                "params": {
                    "item": f"{ty.cpp} const*"
                },
                "body": "self->append(*item);"
            },
            "appendList": {
                "params": {
                    "item": f"QList<{ty.cpp}> const*"
                },
                "body": "self->append(*item);"
            },
            "appendSlice": {
                "params": {
                    "items": f"{ty.cpp} const*",
                    "size": "int"
                },
                "body": "self->reserve(self->size() + size); "
                        "for (int i = 0; i < size; ++i) { self->push_back(items[i]); }"
            },
            "reserveAdditional": {
                "params": {
                    "additional": "int"
                },
                "body": "self->reserve(self->size() + additional);"
            },
        }
    }


def generate():
    root = Path(__file__).parent
    core_root = Path(__file__).parent.parent / "core"

    env = Environment(
        loader=FileSystemLoader(str(root)),
    )

    qffi_template_hpp = env.get_template("qffi_template.hpp.j2")
    qffi_template_cpp = env.get_template("qffi_template.cpp.j2")
    qffi_template_rs = env.get_template("qffi_impl_template.rs.j2")
    qlists_template_rs = env.get_template("list_template.rs.j2")
    bindgen_template = env.get_template("bindgen.yml")

    bindgen = bindgen_template.render({})
    config = BindgenConfig(**yaml.safe_load(bindgen))

    for name, ty in config.qlists.items():
        config.classes[name] = Class(**handle_qlist(name, ty))

    check_config([ReturnNonTrivialClass()], config)

    print(config.json(indent=2))

    (root / "qffi.hpp").write_text(qffi_template_hpp.render(config))
    (root / "qffi.cpp").write_text(qffi_template_cpp.render(config))
    (root / "qffi_impl.rs").write_text(qffi_template_rs.render(config))
    (core_root / "list.rs").write_text(qlists_template_rs.render(config))

    with (root / "qffi.rs").open("w") as fp:
        subprocess.run(
            args=[
                "bindgen",
                str(root / "qffi.hpp"),
                "--no-layout-tests",
                "--no-derive-debug",
                "--no-derive-copy",
                "--size_t-is-usize",
                "--rust-target=1.20",
                "--",
                "-DBINDGEN"],
            stdout=fp)


if __name__ == '__main__':
    generate()
