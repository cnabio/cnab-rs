use crate::cnab::*;
use semver::Version;
use spectral::prelude::*;
use std::path::PathBuf;

#[test]
// Testing that we can build one with only the minimal fields.
fn test_bundle_simple() {
    let bun: Bundle = r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0",
        "version": "1.0.0"
    }"#
    .parse()
    .unwrap();

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0".to_string());
    assert_that(&bun.version).is_equal_to(Version::new(1, 0, 0));
    assert_that(&bun.invocation_images.len()).is_equal_to(&0);
}

// Test labels
#[test]
fn test_bundle_keywords() {
    let bun: Bundle = r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0",
        "version": "1.0.0",
        "keywords": ["a", "b", "c"]
    }"#
    .parse()
    .unwrap();

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0".to_string());
    assert_that(&bun.version).is_equal_to(Version::new(1, 0, 0));
    assert_that(&bun.invocation_images.len()).is_equal_to(&0);

    let kw = &bun.keywords.unwrap();
    assert_that(&kw.len()).is_equal_to(&3);
    assert_that(&kw[0]).is_equal_to("a".to_string());
    assert_that(&kw[1]).is_equal_to("b".to_string());
    assert_that(&kw[2]).is_equal_to("c".to_string());
}

#[test]
fn test_bundle_actions() {
    let bun: Bundle = r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0",
        "version": "1.0.0",
        "actions": {
            "my_action": {
                "modifies": true,
                "stateless": true,
                "description": "a custom action"
            }
        }
    }"#
    .parse()
    .unwrap();

    let actions = bun.actions;
    assert_that(&actions).is_some();
    let action_map = actions.unwrap();
    let my_action = &action_map.get(&"my_action".to_string());
    assert_that(&my_action.is_some());
    assert_that(&my_action.unwrap()).is_equal_to(&Action {
        description: Option::from("a custom action".to_string()),
        modifies: true,
        stateless: true,
    });
}

// Test parameters
#[test]
fn test_bundle_parameters() {
    let bun: Bundle = r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0",
        "version": "1.0.0",
        "parameters": {
            "arg1": {
                "description": "this is a description",
                "destination": {
                    "env": "FIRST"
                },
                "definition": "somedef",
                "metadata": {
                    "description": "a parameter"
                }
            },
            "arg2": {
                "destination": {
                    "path": "/path/to/num"
                },
                "required": true
            },
            "arg3": {
                "applyTo": ["uninstall"],
                "destination": {
                    "env": "LETTERS",
                    "path": "/path/to/abc"
                },
                "required": true
            }
        },
        "definitions": {
            "somedef": {
                "type": "string"
            }
        }
    }"#
    .parse()
    .expect("parsed bundle");

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0".to_string());
    assert_that(&bun.version).is_equal_to(Version::new(1, 0, 0));
    assert_that(
        &bun.definitions
            .expect("definitions")
            .get(&"somedef".to_string()),
    )
    .is_some();

    let params = bun.parameters.expect("params");
    assert_that(&params.len()).is_equal_to(&3);

    // Arg 1 tests
    {
        let arg1 = params.get(&"arg1".to_string()).expect("arg1 exists");

        // required should be set to false by default
        assert!(&arg1.required.is_none());

        // Destination should have just env
        assert_that(&arg1.destination.env.as_ref())
            .is_some()
            .is_equal_to(&"FIRST".to_string());
        assert_that(&arg1.destination.path).is_none();
        assert_that(&arg1.description).is_equal_to(Some("this is a description".into()));
        assert_that(&arg1.definition).is_equal_to(Some("somedef".into()));
    }

    // Arg 2 tests
    {
        let arg2 = params.get(&"arg2".to_string());
        assert_that(&arg2).is_some();

        // required should be set to true
        assert!(&arg2.expect("arg2").required.expect("required"));

        // Destination should have just path
        let destination = &arg2.unwrap().destination;
        assert_that(&destination.env.as_ref()).is_none();
        assert_that(&destination.path)
            .is_some()
            .is_equal_to("/path/to/num".parse::<std::path::PathBuf>().unwrap());
    }
    // Arg 3 tests
    {
        let arg3 = params.get(&"arg3".to_string());

        assert!(arg3.is_some());

        let apply = &arg3.unwrap().apply_to;
        assert!(apply.is_some());

        let dest = &arg3.unwrap().destination;
        let env = &dest.env;
        assert_that(&env).is_equal_to(&Some("LETTERS".to_string()));

        let path = &dest.path;
        assert_that(path)
            .is_some()
            .is_equal_to("/path/to/abc".parse::<std::path::PathBuf>().unwrap());

        let apply_to = &arg3.unwrap().apply_to;
        assert_that(apply_to).is_equal_to(&Some(vec!["uninstall".to_string()]));
    }
}

// Test custom data
#[test]
fn test_bundle_outputs() {
    let bun: Bundle = r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0.0",
        "version": "1.0.0",
        "outputs": {
          "first": {
            "applyTo": ["example"],
            "definition": "somedef",
            "description": "does stuff",
            "path": "/var/run/hello"
          },
          "second": {
              "path": "/var/run/empty",
              "definition": "somedef"
          }
        },
        "definitions": {
            "somedef": {
                "type": "string"
            }
        }
    }"#
    .parse()
    .expect("bundle parsed");

    assert_that(&bun.outputs.as_ref().expect("outputs").len()).is_equal_to(2);
    let first = bun
        .outputs
        .as_ref()
        .expect("outputs")
        .get("first")
        .expect("first");
    assert_that(&first.apply_to.as_ref().expect("applyTo")[0]).is_equal_to(&"example".to_string());
    assert_that(&first.definition).is_equal_to(&"somedef".to_string());
    assert_that(&first.path.as_ref().expect("path buffer")).is_equal_to(&PathBuf::from("/var/run/hello"));
    assert_that(&first.description.as_ref().expect("description"))
        .is_equal_to(&"does stuff".to_string());
}

// Test custom data
#[test]
fn test_bundle_custom() {
    let bun: Bundle = r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0.0",
        "version": "1.0.0",
        "custom": {
          "com.example.praxis": {
            "techne": true
          }
        }
    }"#
    .parse()
    .unwrap();

    assert_that(&bun.custom).is_some();
    let val: Option<&serde_json::Value> = bun
        .custom
        .as_ref()
        .unwrap()
        .get(&"com.example.praxis".to_string());
    // Lookup docs on Value when I'm online again.
    assert_that(&val).is_some(); // .map(|v| v.get("foo").is_some() );
}

// Test credentials
#[test]
fn test_bundle_credentials() {
    let bun: Bundle = r#"{
        "name": "aristotle",
        "invocationImages": [],
        "schemaVersion": "1.0",
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
    }"#
    .parse()
    .unwrap();

    assert_that(&bun.credentials.as_ref()).is_some();

    let creds = &bun.credentials.as_ref().unwrap();
    let first = &creds.get(&"mytoken".to_string()).unwrap();
    assert_that(&first.description)
        .is_some()
        .is_equal_to("token".to_string());
    assert_that(&first.env)
        .is_some()
        .is_equal_to("TOKEN".to_string());
    assert_that(&first.path).is_none();

    let second = &creds.get(&"myconfig".to_string()).unwrap();
    assert_that(&second.description)
        .is_some()
        .is_equal_to("config".to_string());
    assert_that(&second.env).is_none();
    assert_that(&second.path)
        .is_some()
        .is_equal_to("/etc/config".parse::<std::path::PathBuf>().unwrap());

    let third = &creds.get(&"myboth".to_string()).unwrap();
    assert_that(&third.description).is_none();
    assert_that(&third.env).is_some();
    assert_that(&third.path).is_some();
}

// Test invocation images and regular images
#[test]
fn test_bundle_images() {
    // Testing that we can build one with only the minimal fields.
    let bun: Bundle = r#"{
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
                "size": 1234567890
            }
        ],
        "schemaVersion": "1.0",
        "version": "1.0.0",
        "labels": ["hello", "world"]
    }"#
    .parse()
    .expect("bundle is unwrapped");

    assert_that(&bun.name).is_equal_to("aristotle".to_string());
    assert_that(&bun.schema_version).is_equal_to("1.0".to_string());
    assert_that(&bun.version).is_equal_to(Version::new(1, 0, 0));

    // Check that all of the fields unmarshaled correctly.
    let invo_imgs = &bun.invocation_images;
    assert_that(&invo_imgs.len()).is_equal_to(1);
    {
        let ii1 = &invo_imgs[0];
        assert_that(&ii1.image).is_equal_to("nginx:latest".to_string());
        assert_that(&ii1.image_type).is_equal_to(Some("oci".to_string()));
        assert_that(&ii1.media_type).is_equal_to(Some("application/x-image-thinger".to_string()));
        assert_that(&ii1.size).is_equal_to(Some(1_234_567_890));
    }

    let imgs = &bun.images.as_ref();
    assert_that(&imgs.expect("image").len()).is_equal_to(1);
    {
        let img = &bun
            .images
            .as_ref()
            .expect("at least one image")
            .get(&"web".to_string())
            .expect("web");
        assert_that(&img.image).is_equal_to("nginx:latest".to_string());
        assert_that(&img.image_type).is_equal_to(Some("oci".to_string()));
        assert_that(&img.media_type).is_equal_to(Some("application/x-image-thinger".to_string()));
        assert_that(&img.size).is_equal_to(Some(1_234_567_890));
        assert_that(&img.platform.as_ref().unwrap().os).is_equal_to(Some("linux".to_string()));
        assert_that(&img.platform.as_ref().unwrap().arch).is_equal_to(Some("amd64".to_string()));
    }
}

// Test that a parsing failure returns an error (not a panic)
#[test]
fn test_bundle_parse_error() {
    let bad_data = "{hello";
    let bun = bad_data.parse::<Bundle>();
    assert_that(&bun.is_err()).is_true()
}

// Test loading a bundle from a file
#[test]
fn test_bundle_deserialize() {
    let bun = Bundle::from_file("testdata/bundle.json").expect("parse testdata/bundle.json");

    assert_that(&bun.name).is_equal_to("helloworld".to_string());
    assert_that(&bun.schema_version).is_equal_to("v1.0.0".to_string());
    assert_that(&bun.version).is_equal_to(Version::new(0, 1, 2));
    assert_that(&bun.maintainers.unwrap().len()).is_equal_to(&1);
    assert_that(&bun.custom.unwrap().len()).is_equal_to(&2);
}

// Check that a missing file results in an error (not a panic)
#[test]
fn test_bundle_from_file_not_found() {
    let bun = Bundle::from_file("no/such/file.json");
    assert_that(&bun.is_err()).is_true();
}
