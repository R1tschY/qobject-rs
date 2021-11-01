import subprocess
from pathlib import Path
from typing import List, Dict, Optional

from jinja2 import Environment, FileSystemLoader
import yaml
from pydantic import BaseModel, Field

root = Path(__file__).parent

env = Environment(
    loader=FileSystemLoader(str(root)),
)


class Method(BaseModel):
    params: Dict[str, str] = Field(default_factory=dict)
    return_: Optional[str] = Field(default="void", alias="return")
    body: str
    const: bool = False
    static: bool = False


class Class(BaseModel):
    dtor: Optional[bool] = None
    default_ctor: bool = Field(default=False, alias="default-ctor")
    copy_ctor: bool = Field(default=False, alias="copy-ctor")
    trivially: bool = False
    movable: bool = False
    eq: bool = False
    ord: bool = False
    methods: Dict[str, Method] = Field(default_factory=dict)
    qobject: bool = False
    qobject_default_ctor: bool = Field(default=False, alias="qobject-default-ctor")
    overwrite_include: Optional[str] = Field(default=None, alias="overwrite-include")
    overwrite_name: Optional[str] = Field(default=None, alias="overwrite-name")

    @property
    def generate_dtor(self) -> bool:
        if self.dtor is not None:
            return self.dtor

        if self.qobject:
            # Use QObject dtor
            return False

        return True


class BindgenConfig(BaseModel):
    classes: Dict[str, Class]
    includes: List[str]


qffi_template_hpp = env.get_template("qffi_template.hpp.j2")
qffi_template_cpp = env.get_template("qffi_template.cpp.j2")
qffi_template_rs = env.get_template("qffi_impl_template.rs.j2")

with (root / "bindgen.yml").open("r") as fp:
    config = BindgenConfig(**yaml.safe_load(fp))

print(config.json(indent=2))

(root / "qffi.hpp").write_text(qffi_template_hpp.render(config))
(root / "qffi.cpp").write_text(qffi_template_cpp.render(config))
(root / "qffi_impl.rs").write_text(qffi_template_rs.render(config))

with (root / "qffi.rs").open("w") as fp:
    subprocess.run(
        args=[
            "bindgen",
            str(root / "qffi.hpp"),
            "--no-layout-tests",
            "--no-derive-debug",
            "--no-derive-copy",
            "--",
            "-DBINDGEN"],
        stdout=fp)
