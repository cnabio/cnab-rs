extern crate spectral;

use spectral::prelude::*;

use super::*;

#[test]
fn test_bundle_simple(){
    // Testing that we can build one with only the minimal fields.
    let res = Bundle::from_string(r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0-WD",
        "version": "1.0.0"
    }"#);

    let bun = res.unwrap();

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0-WD".to_string());
    assert_that(&bun.version).is_equal_to("1.0.0".to_string());
    assert_that(&bun.invocation_images.len()).is_equal_to(&0);
}

#[test]
fn test_bundle_keywords(){
    // Testing that we can build one with only the minimal fields.
    let res = Bundle::from_string(r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0-WD",
        "version": "1.0.0",
        "keywords": ["a", "b", "c"]
    }"#);

    let bun = res.unwrap();

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0-WD".to_string());
    assert_that(&bun.version).is_equal_to("1.0.0".to_string());
    assert_that(&bun.invocation_images.len()).is_equal_to(&0);
    assert_that(&bun.keywords.unwrap().len()).is_equal_to(&3);
}

#[test]
fn test_bundle_parameters(){
    // Testing that we can build one with only the minimal fields.
    let res = Bundle::from_string(r#"{
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
                "apply_to": ["delete"],
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
    }"#);

    let bun = res.unwrap();

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0-WD".to_string());
    assert_that(&bun.version).is_equal_to("1.0.0".to_string());

    let params = bun.parameters.unwrap();
    assert_that(&params.len()).is_equal_to(&3);

    let arg3 = params.get(&"arg3".to_string()).unwrap();
    //assert_that(&arg3.apply_to.unwrap().len()).is_equal_to(&1);
    assert_that(&arg3.parameter_type).is_equal_to("string".to_string());
}

#[test]
fn test_bundle_images(){
    // Testing that we can build one with only the minimal fields.
    let res = Bundle::from_string(r#"{
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
    }"#);

    let bun = res.unwrap();

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0-WD".to_string());
    assert_that(&bun.version).is_equal_to("1.0.0".to_string());
    assert_that(&bun.invocation_images.len()).is_equal_to(&1);
}
