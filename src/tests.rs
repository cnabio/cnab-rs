extern crate spectral;

use crate::cnab::Bundle;
use serde_json::*;
use spectral::prelude::*;

#[test]
// Testing that we can build one with only the minimal fields.
fn test_bundle_simple() {
    let res = Bundle::from_string(
        r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0-WD",
        "version": "1.0.0"
    }"#,
    );

    let bun = res.unwrap();

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0-WD".to_string());
    assert_that(&bun.version).is_equal_to("1.0.0".to_string());
    assert_that(&bun.invocation_images.len()).is_equal_to(&0);
}

// Test labels
#[test]
fn test_bundle_keywords() {
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

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0-WD".to_string());
    assert_that(&bun.version).is_equal_to("1.0.0".to_string());
    assert_that(&bun.invocation_images.len()).is_equal_to(&0);

    let kw = &bun.keywords.unwrap();
    assert_that(&kw.len()).is_equal_to(&3);
    assert_that(&kw[0]).is_equal_to("a".to_string());
    assert_that(&kw[1]).is_equal_to("b".to_string());
    assert_that(&kw[2]).is_equal_to("c".to_string());
}

// Test parameters
#[test]
fn test_bundle_parameters() {
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
                "exclusiveMinimum": 123,
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

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0-WD".to_string());
    assert_that(&bun.version).is_equal_to("1.0.0".to_string());

    let params = bun.parameters.unwrap();
    assert_that(&params.len()).is_equal_to(&3);

    // Arg 1 tests
    {
        let arg1 = params.get(&"arg1".to_string());
        assert_that(&arg1).is_some();

        // required should be set to false by default
        assert_that(&arg1.unwrap().required).is_false();

        // Destination should have just env
        assert_that(&arg1.unwrap().destination.env.as_ref()).is_some().is_equal_to(&"FIRST".to_string());
        assert_that(&arg1.unwrap().destination.path).is_none();

        // Test exclusive_min/max
        assert_that(&arg1.unwrap().exclusive_minimum).is_some().is_equal_to(&123);
        assert_that(&arg1.unwrap().exclusive_maximum).is_some().is_equal_to(&567789);

        // Sanity check that min and max are none.
        assert_that(&arg1.unwrap().minimum).is_none();
        assert_that(&arg1.unwrap().maximum).is_none();
    }

    // Arg 2 tests
    {
        let arg2 = params.get(&"arg2".to_string());
        assert_that(&arg2).is_some();

        // required should be set to true
        assert_that(&arg2.unwrap().required).is_true();

        // Destination should have just path
        assert_that(&arg2.unwrap().destination.env.as_ref()).is_none();
        assert_that(&arg2.unwrap().destination.path.as_ref()).is_some().is_equal_to(&"/path/to/num".to_string());

        // Test min/max
        assert_that(&arg2.unwrap().minimum).is_some().is_equal_to(&123);
        assert_that(&arg2.unwrap().maximum).is_some().is_equal_to(&567789);

        // Sanity check that exclusive min and max are none.
        assert_that(&arg2.unwrap().exclusive_minimum).is_none();
        assert_that(&arg2.unwrap().exclusive_maximum).is_none();
    }

    // Arg 3 tests
    {
        let arg3 = params.get(&"arg3".to_string());

        assert!(arg3.is_some());
        assert_that(&arg3.unwrap().parameter_type).is_equal_to("string".to_string());

        let apply = &arg3.unwrap().apply_to;
        assert!(apply.is_some());

        let dest = &arg3.unwrap().destination;
        let env = &dest.env;
        assert_that(&env).is_equal_to(&Some("LETTERS".to_string()));

        let path = &dest.path;
        assert_that(path).is_equal_to(&Some("/path/to/abc".to_string()));

        let abc = json!("abc");
        let dv = &arg3.unwrap().default_value;
        assert_that(dv).is_equal_to(&Some(abc));

        let allowed = &arg3.unwrap().allowed_values;
        assert_that(allowed).is_equal_to(&Some(vec![json!("a"), json!("ab"), json!("abc")]));

        assert_that(&arg3.as_ref().unwrap().min_length).is_some().is_equal_to(1);
        assert_that(&arg3.as_ref().unwrap().max_length).is_some().is_equal_to(5);
        assert_that(&arg3.as_ref().unwrap().pattern).is_equal_to(&Some("[a-z]+".to_string()));
        assert_that(&arg3.unwrap().required).is_true();

        let meta = &arg3.as_ref().unwrap().metadata;
        assert_that(&meta.as_ref()).is_some();

        assert_that(&meta.as_ref().unwrap().description.as_ref()).is_some().is_equal_to(&"a parameter".to_string());

        let apply_to = &arg3.unwrap().apply_to;
        assert_that(apply_to).is_equal_to(&Some(vec!["uninstall".to_string()]));
        assert_that(&arg3.unwrap().parameter_type).is_equal_to("string".to_string());
    }
}

// Test custom data
#[test]
fn test_bundle_custom() {
    let res = Bundle::from_string(
        r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0.0",
        "version": "1.0.0",
        "custom": {
          "com.example.praxis": {
            "techne": true
          }
        }
    }"#,
    );

    assert_that(&res).is_ok();

    let bun = res.unwrap();
    assert_that(&bun.custom).is_some();
    let val: Option<&serde_json::Value> = bun.custom.as_ref().unwrap().get(&"com.example.praxis".to_string());
    // Lookup docs on Value when I'm online again.
    assert_that(&val).is_some(); // .map(|v| v.get("foo").is_some() );

}

// Test credentials
#[test]
fn test_bundle_credentials() {
    let res = Bundle::from_string(
        r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0-WD",
        "version": "1.0.0",
        "credentials": {
            "mytoken": {
                "description": "token",
                "env": "TOKEN"
            },
            "myconfig": {
                "description": "config",
                "path": "/etc/config"
            },
            "myboth": {
                "path": "/foo",
                "env": "FOO"
            }
        }
    }"#,
    );

    let bun = res.unwrap();

    assert_that(&bun.credentials.as_ref()).is_some();

    let creds = &bun.credentials.as_ref().unwrap();
    let first = &creds.get(&"mytoken".to_string()).unwrap();
    assert_that(&first.description).is_some().is_equal_to("token".to_string());
    assert_that(&first.env).is_some().is_equal_to("TOKEN".to_string());
    assert_that(&first.path).is_none();

    let second = &creds.get(&"myconfig".to_string()).unwrap();
    assert_that(&second.description).is_some().is_equal_to("config".to_string());
    assert_that(&second.env).is_none();
    assert_that(&second.path).is_some().is_equal_to("/etc/config".to_string());

    let third = &creds.get(&"myboth".to_string()).unwrap();
    assert_that(&third.description).is_none();
    assert_that(&third.env).is_some();
    assert_that(&third.path).is_some();
}

// Test invocation images and regular images
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

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0-WD".to_string());
    assert_that(&bun.version).is_equal_to("1.0.0".to_string());

    // Check that all of the fields unmarshaled correctly.
    let invo_imgs = &bun.invocation_images;
    assert_that(&invo_imgs.len()).is_equal_to(1);
    {
        let ii1 = &invo_imgs[0];
        assert_that(&ii1.image).is_equal_to("nginx:latest".to_string());
        assert_that(&ii1.image_type).is_equal_to(Some("oci".to_string()));
        assert_that(&ii1.media_type).is_equal_to(Some("application/x-image-thinger".to_string()));
        assert_that(&ii1.size).is_equal_to(Some(1234567890));
        assert_that(&ii1.platform.as_ref().unwrap().os).is_equal_to(Some("linux".to_string()));
        assert_that(&ii1.platform.as_ref().unwrap().arch).is_equal_to(Some("amd64".to_string()));
    }



    let imgs = &bun.images.as_ref();
    assert_that(&imgs.unwrap().len()).is_equal_to(1);
    {
        let img = &bun.images.as_ref().unwrap().get(&"web".to_string()).unwrap();
        assert_that(&img.image).is_equal_to("nginx:latest".to_string());
        assert_that(&img.image_type).is_equal_to(Some("oci".to_string()));
        assert_that(&img.media_type).is_equal_to(Some("application/x-image-thinger".to_string()));
        assert_that(&img.size).is_equal_to(Some(1234567890));
        assert_that(&img.platform.as_ref().unwrap().os).is_equal_to(Some("linux".to_string()));
        assert_that(&img.platform.as_ref().unwrap().arch).is_equal_to(Some("amd64".to_string()));
    }
}


// Test that a parsing failure returns an error (not a panic)
#[test]
fn test_bundle_parse_error() {
    let bad_data = "{hello";
    let bun = Bundle::from_string(bad_data);
    assert_that(&bun.is_err()).is_true()
}


// Test loading a bundle from a file
#[test]
fn test_bundle_deserialize() {
    let bun = Bundle::from_file("testdata/bundle.json").unwrap();

    assert_that(&bun.name).is_equal_to("helloworld".to_string());
    assert_that(&bun.schema_version).is_equal_to("v1.0.0-WD".to_string());
    assert_that(&bun.version).is_equal_to("0.1.2".to_string());
    assert_that(&bun.maintainers.unwrap().len()).is_equal_to(&1);
    assert_that(&bun.custom.unwrap().len()).is_equal_to(&2);
}


// Check that a missing file results in an error (not a panic)
#[test]
fn test_bundle_from_file_not_found() {
    let bun = Bundle::from_file("no/such/file.json");
    assert_that(&bun.is_err()).is_true();
}
