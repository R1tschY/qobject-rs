from pathlib import Path

from jinja2 import Environment, FileSystemLoader

root = Path(__file__).parent

env = Environment(
    loader=FileSystemLoader(str(root)),
)

# QList
(root / "list.rs").write_text(env.get_template("list.rs.jinja").render(
    includes=[
        "QList",
        "QObject",
        "QString"
    ],
    types=[
        {
            "name": "QObjectList",
            "cpp": "QObject*",
        },
        {
            "name": "QStringList",
            "cpp": "QString",
        }
    ]
))

# QHash
(root / "hash.rs").write_text(env.get_template("hash.rs.jinja").render(
    includes=[
        "QHash",
        "QByteArray"
    ],
    uses=[
        "crate::core::QByteArray",
        "std::collections::HashMap"
    ],
    types=[
        {
            "name": "QHashIntQByteArray",
            "key_rs": "i32",
            "key": "int",
            "value_rs": "QByteArray",
            "value": "QByteArray",
        },
    ]
))

