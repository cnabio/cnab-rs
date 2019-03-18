extern crate spectral;

use crate::cnab::Bundle;
use serde_json::*;
use spectral::prelude::*;

#[test]
fn test_bundle_simple() {
    // Testing that we can build one with only the minimal fields.
    let res = Bundle::from_string(
        r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0-WD",
        "version": "1.0.0"
    }"#,
    );

    let bun = res.unwrap();

    assert_that(&bun.name).is_equal_to("aristotle");
    assert_that(&bun.schema_version).is_equal_to("1.0-WD");
    assert_that(&bun.version).is_equal_to("1.0.0");
    assert_that(&bun.invocation_images.len()).is_equal_to(&0);
}

#[test]
fn test_bundle_keywords() {
    // Testing that we can build one with only the minimal fields.
    let res = Bundle::from_string(
        r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0-WD",
        "version": "1.0.0",
        "keywords": ["a", "b", "c"]
    }"#,
    );

    let bun = res.unwrap();

    assert_that(&bun.name).is_equal_to("aristotle");
    assert_that(&bun.schema_version).is_equal_to("1.0-WD");
    assert_that(&bun.version).is_equal_to("1.0.0");
    assert_that(&bun.invocation_images.len()).is_equal_to(&0);

    let kw = &bun.keywords.unwrap();
    assert_that(&kw.len()).is_equal_to(&3);
    assert_that(&kw[0]).is_equal_to("a");
    assert_that(&kw[1]).is_equal_to("b");
    assert_that(&kw[2]).is_equal_to("c");
}

#[test]
fn test_bundle_parameters() {
    // Testing that we can build one with only the minimal fields.
    let res = Bundle::from_string(
        r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0-WD",
        "version": "1.0.0",
        "parameters": {
            "arg1": {
                "destination": {
                    "env": "FIRST"
                },
                "defaultValue": 1234,
                "exclsiveMinimum": 123,
                "exclusiveMaximum": 567789,
                "metadata": {
                    "description": "a parameter"
                },
                "type": "int"
            },
            "arg2": {
                "destination": {
                    "path": "/path/to/num"
                },
                "defaultValue": 1234,
                "minimum": 123,
                "maximum": 567789,
                "metadata": {
                    "description": "a parameter"
                },
                "required": true,
                "type": "int"
            },
            "arg3": {
                "applyTo": ["uninstall"],
                "destination": {
                    "env": "LETTERS",
                    "path": "/path/to/abc"
                },
                "defaultValue": "abc",
                "enum": ["a", "ab", "abc"],
                "minLength": 1,
                "maxLength": 5,
                "metadata": {
                    "description": "a parameter"
                },
                "pattern": "[a-z]+",
                "required": true,
                "type": "string"
            }
        }
    }"#,
    );

    let bun = res.unwrap();

    assert_that(&bun.name).is_equal_to("aristotle");
    assert_that(&bun.schema_version).is_equal_to("1.0-WD");
    assert_that(&bun.version).is_equal_to("1.0.0");

    let params = bun.parameters.unwrap();
    assert_that(&params.len()).is_equal_to(&3);

    let arg3 = params.get(&"arg3");

    assert!(arg3.is_some());
    assert_that(&arg3.unwrap().parameter_type).is_equal_to("string");

    let apply = &arg3.unwrap().apply_to;
    assert!(apply.is_some());

    let dest = &arg3.unwrap().destination;
    let env = &dest.env;
    assert_that(&env).is_equal_to(&Some("LETTERS"));

    let path = &dest.path;
    assert_that(path).is_equal_to(&Some("/path/to/abc"));

    let abc = json!("abc");
    let dv = &arg3.unwrap().default_value;
    assert_that(dv).is_equal_to(&Some(abc));

    let allowed = &arg3.unwrap().allowed_values;
    assert_that(allowed).is_equal_to(&Some(vec![json!("a"), json!("ab"), json!("abc")]));

    assert_that(&arg3.as_ref().unwrap().min_length.unwrap()).is_equal_to(1);
    assert_that(&arg3.as_ref().unwrap().max_length.unwrap()).is_equal_to(5);
    assert_that(&arg3.as_ref().unwrap().pattern).is_equal_to(&Some("[a-z]+"));
    assert_that(&arg3.unwrap().required).is_equal_to(true);

    let meta = &arg3.unwrap().metadata;
    assert_that(&meta.is_some()).is_equal_to(true);

    assert_that(&meta.as_ref().unwrap().description.as_ref().unwrap()).is_equal_to(&"a parameter");

    let apply_to = &arg3.unwrap().apply_to;
    assert_that(apply_to).is_equal_to(&Some(vec!["uninstall"]));
    assert_that(&arg3.unwrap().parameter_type).is_equal_to("string");
}

#[test]
fn test_bundle_images() {
    // Testing that we can build one with only the minimal fields.
    let res = Bundle::from_string(
        r#"{
        "name": "aristotle",
        "images": {
            "web": {
                "image": "nginx:latest",
                "imageType": "oci",
                "mediaType": "application/x-image-thinger",
                "platform": {
                    "os": "linux",
                    "arch": "amd64"
                },
                "size": 1234567890
            }
        },
        "invocationImages": [
            {
                "image": "nginx:latest",
                "imageType": "oci",
                "mediaType": "application/x-image-thinger",
                "platform": {
                    "os": "linux",
                    "arch": "amd64"
                },
                "size": 1234567890
            }
        ],
        "schemaVersion": "1.0-WD",
        "version": "1.0.0",
        "labels": ["hello", "world"]
    }"#,
    );

    let bun = res.unwrap();

    assert_that(&bun.name).is_equal_to("aristotle");
    assert_that(&bun.schema_version).is_equal_to("1.0-WD");
    assert_that(&bun.version).is_equal_to("1.0.0");
    assert_that(&bun.invocation_images.len()).is_equal_to(&1);
}
