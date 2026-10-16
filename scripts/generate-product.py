#!/usr/bin/env python3
"""Generate Rust protocol models from a local official Java SDK snapshot.

Only annotated wire-field names/types and operation identifiers are consumed.
No Java implementation, comments, inheritance or credential behavior is copied.
Generation is offline, opt-in, and never runs during a Cargo build.
"""
import argparse
import json
import pathlib
import re


def snake(name):
    name = name.replace("BlackList", "Blacklist").replace("PassWord", "Password")
    name = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name).lower()


def rust_name(name):
    return "".join(part.capitalize() for part in snake(name).split("_"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("java_root", type=pathlib.Path)
    parser.add_argument("product", choices=["ses", "ssm"])
    parser.add_argument("version")
    args = parser.parse_args()
    root = pathlib.Path(__file__).resolve().parents[1]
    product, version = args.product, args.version
    source = args.java_root / "src/main/java/com/tencentcloudapi" / product / version
    target = root / "crates" / product
    models_dir = target / "src" / version / "models"
    models_dir.mkdir(parents=True, exist_ok=True)
    field_pattern = re.compile(r'@SerializedName\("([^\"]+)"\)\s*@Expose\s*private\s+(\w+)(\s*\[\])?\s+(\w+)\s*;')
    primitives = {"String": "String", "Long": "i64", "Integer": "i32", "Double": "f64", "Float": "f32", "Boolean": "bool"}
    schema = {}
    for path in sorted((source / "models").glob("*.java")):
        text = path.read_text()
        fields = field_pattern.findall(text)
        assert len(fields) == len(re.findall(r"@SerializedName\(", text)), path
        schema[path.stem] = [(wire, kind, bool(array)) for wire, kind, array, _ in fields]
    assert schema, "no models found"
    for name, fields in schema.items():
        imports = sorted({rust_name(kind) for _, kind, _ in fields if kind not in primitives})
        assert all(kind in primitives or kind in schema for _, kind, _ in fields), name
        text = "// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).\n"
        text += "// Copyright (c) 2017-2025 Tencent. All Rights Reserved.\n"
        text += "// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.\n"
        text += "// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.\n"
        if imports:
            text += "use super::{" + ", ".join(imports) + "};\n"
        text += "use serde::{Deserialize, Serialize};\nuse std::fmt;\n"
        text += f"/// The `{name}` wire model. Fields follow the official {product.upper()} API schema.\n"
        text += "#[derive(Clone, Default, Serialize, Deserialize)]\n"
        text += f"pub struct {rust_name(name)} {{\n"
        rust_fields = []
        for wire, kind, array in fields:
            field = snake(wire)
            if field in {"type", "match", "ref", "self", "use", "mod", "in", "where", "loop", "return"}:
                field = "r#" + field
            assert field not in rust_fields, (name, field)
            rust_fields.append(field)
            ty = primitives.get(kind, rust_name(kind))
            if array:
                ty = f"Vec<{ty}>"
            text += f'    /// Official `{wire}` field; omitted when unset and accepts JSON null.\n'
            text += f'    #[serde(rename = "{wire}", default, skip_serializing_if = "Option::is_none")]\n'
            text += f"    pub {field}: Option<{ty}>,\n"
        text += "}\n"
        text += f"impl fmt::Debug for {rust_name(name)} {{\n"
        text += "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n"
        text += f'        f.debug_struct("{rust_name(name)}").finish_non_exhaustive()\n'
        text += "    }\n}\n"
        (models_dir / (snake(name) + ".rs")).write_text(text)
    index = "//! Strongly typed wire models. Debug output deliberately hides all field values.\n"
    for name in schema:
        index += f"mod {snake(name)};\npub use {snake(name)}::{rust_name(name)};\n"
    (models_dir / "mod.rs").write_text(index)
    client_source = (source / (product.capitalize() + "Client.java")).read_text()
    operations = re.findall(r"public\s+(\w+Response)\s+(\w+)\((\w+Request)\s+req\)", client_source)
    assert len(operations) == len([name for name in schema if name.endswith("Request")])
    assert all(request in schema and response in schema for response, _, request in operations)
    registry = "// Generated official action-to-request/response bindings.\noperations! {\n"
    for response, action, request in operations:
        registry += f'    {snake(action)}: {rust_name(request)} => {rust_name(response)} = "{action}";\n'
    registry += "}\n"
    (target / "src" / version / "operations.rs").write_text(registry)
    fixture_dir = target / "tests" / "fixtures"
    fixture_dir.mkdir(parents=True, exist_ok=True)
    # A checked-in schema lets tests and future maintenance inspect the exact snapshot.
    manifest = {"product": product, "version": version, "operations": operations, "models": schema}
    (fixture_dir / "schema.json").write_text(json.dumps(manifest, indent=2) + "\n")
    tests = "// Generated exhaustive model round trips and operation routing checks.\n"
    def sample(name):
        result = {}
        for wire, kind, array in schema[name]:
            if kind == "String": value = "test-marker"
            elif kind in {"Long", "Integer"}: value = 123
            elif kind in {"Double", "Float"}: value = 1.5
            elif kind == "Boolean": value = True
            else: value = sample(kind)
            result[wire] = [value] if array else value
        return result
    for name in schema:
        fixture = json.dumps(sample(name), separators=(",", ":"))
        nulls = json.dumps({wire: None for wire, _, _ in schema[name]}, separators=(",", ":"))
        encoded_fixture = '"{}"' if fixture == "{}" else 'r#"' + fixture + '"#'
        encoded_nulls = '"{}"' if nulls == "{}" else 'r#"' + nulls + '"#'
        tests += f'#[test]\nfn model_{snake(name)}() {{\n'
        tests += f'    round_trip::<models::{rust_name(name)}>({encoded_fixture}, {encoded_nulls});\n}}\n'
    for response, action, request in operations:
        response_value = sample(response)
        response_value["RequestId"] = "test-request-id"
        tests += f'#[tokio::test]\nasync fn route_{snake(action)}() {{\n'
        tests += f'      let (client, calls) = mock_client("{action}", serde_json::json!({{"Response": {json.dumps(response_value, separators=(",", ":"))}}}), http::StatusCode::OK);\n'
        tests += f'      let result = client.{snake(action)}(&models::{rust_name(request)}::default()).await.unwrap();\n'
        tests += '      assert_eq!(result.metadata.request_id.as_deref(), Some("test-request-id"));\n'
        tests += f'      assert_eq!(serde_json::to_value(result.data).unwrap(), serde_json::json!({json.dumps(response_value, separators=(",", ":"))}));\n'
        tests += '      assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1); }\n'
    (target / "tests" / "generated").mkdir(parents=True, exist_ok=True)
    (target / "tests" / "generated" / "checks.rs").write_text(tests)
    print(f"Generated {product} {version}: {len(operations)} operations, {len(schema)} models")


if __name__ == "__main__":
    main()
