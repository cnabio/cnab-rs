#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::collections::HashMap;

// Bundle implementa a CNAB bundle descriptor
//
// Bundle descriptors describe the properties of a bundle, including which images
// are associated, what parameters and credentials are configurable, and whether there
// are any additional traget actions that can be executed on this bundle.
//
// The fields here are in canonical order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Bundle {
    // The list of additional actions that this bundle can perform.
    //
    // 'install', 'upgrade', and 'uninstall' are default actions, but additional actions
    // may be defined here.
    actions: Option<HashMap<String, Action>>,
    // The list of configurable credentials.
    //
    // Credentials are injected into the bundle's invocation image at startup time.
    credentials: Option<Vec<Credential>>,
    // This field allows for additional data to described in the bundle.
    //
    // This data should be stored in key/value pairs, where the value is undefined by
    // the specification (but must be representable as JSON).
    custom: Option<HashMap<String, serde_json::Value>>,
    // description is a short description of this bundle
    description: Option<String>,
    // The list of images that comprise this bundle.
    //
    // Each image here is considered a constituent of the application described by this
    // bundle.
    images: Option<HashMap<String, Image>>,
    // inovcation_images is the list of available bootstrapping images for this bundle
    //
    // Only one ought to be executed.
    invocation_images: Vec<Image>,
    // keywords is a list of keywords describing this bundle
    keywords: Option<Vec<String>>,
    // license is the license of this bundle
    license: Option<String>,
    // maintainers is a list of maintainers responsible for this bundle
    maintainers: Option<Vec<Maintainer>>,
    // name is the name of the bundle
    name: String,
    // The collection of parameters that can be passed into this bundle.
    //
    // Parameters can be injected into a bundle during startup time.
    parameters: Option<HashMap<String, Parameter>>,
    // schema_version is the version of the CNAB specification used to describe this
    schema_version: String,
    // version is the version of the bundle
    version: String,
}

// Represents a bundle.
impl Bundle {
    //fn new(name: String, version: String) -> Bundle {}
    pub fn from_string(json_data: &str) -> Result<Bundle, serde_json::Error> {
        let res: Bundle = serde_json::from_str(json_data)?;
        Ok(res)
    }
}

// Maintainer describes a bundle mainainer.
//
// The name field is required, though the format of its value is unspecified.
#[derive(Debug, Serialize, Deserialize)]
struct Maintainer {
    // The email address of the maintainer
    email: Option<String>,
    // The name of the maintainer
    name: String,
    // A URL with more information about the maintainer
    url: Option<String>,
}

// Image describes a CNAB image.
//
// Both invocation images and regular images can be described using this object.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    // A digest to be used to verify the integrity of the image
    digest: Option<String>,
    // The image, as a string of the form REPO/NAME:TAG@SHA
    image: String,
    // The type of image. Typically, this is treated as an OCI Image
    image_type: Option<String>,
    // The media type of the image
    media_type: Option<String>,
    // The platform this image may be deployed on
    platform: Option<Platform>,
    // The size in bytes of the image
    size: Option<i64>,
}

// Platform defines a platform as a machine architecture plus and operating system
#[derive(Debug, Serialize, Deserialize)]
struct Platform {
    // The architecture
    //
    // Typical values are amd64, i386, and arm64
    arch: Option<String>,
    // The operating system.
    //
    // Typical values are darwin, windows, and linux
    os: Option<String>,
}

// Credential describes a particular credential that may be injected into a bundle
#[derive(Debug, Serialize, Deserialize)]
struct Credential {
    // The description of this credential
    description: Option<String>,
    // The name of the environment variable into which the value will be placed
    env: Option<String>,
    // The fully qualified path into which the value will be placed
    path: Option<String>,
}

// Parameter describes a parameter that will be put into the invocation image
//
// Paramters are injected into the invocation image at startup time
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Parameter {
    // The actions to which this parameter applies.
    //
    // If unset, this parameter will be applied to all actions.
    apply_to: Option<Vec<String>>,
    // The location where this parameter will be injected in the invocation image
    destination: Destination,
    // This parameter's default value
    default_value: Option<serde_json::Value>,

    // An enumeration of allowed values
    #[serde(rename = "enum")]
    allowed_values: Option<Vec<serde_json::Value>>, // alphabetically, this is 'enum'
    // The exclusive maximum.
    //
    // If unspecified, no exclusive max is applied
    exclusive_maximum: Option<i64>,
    // The exclusive minimum.
    //
    // If unspecified, no exclusive min is applied
    exclusive_minimum: Option<i64>,
    // The maximum
    //
    // If unspecieid, the maximum 64-bit integer value is applied
    maximum: Option<i64>,
    // The maximum length of a string value
    //
    // If unspecified, no max is applied.
    max_length: Option<i64>,
    // Additional parameter information
    metadata: Option<Metadata>,
    // The minimum integer value
    //
    // If unspecified, the minimum 64-bit integer value is applied
    minimum: Option<i64>,
    // The minimum string length
    min_length: Option<i64>,
    // A regular expression (as defined in ECMAScript)
    //
    // If it is not matched, a string parameter value will be rejected
    pattern: Option<String>,
    // Indicate whether this parameter is required
    //
    // Default is false.
    #[serde(default)]
    required: bool,
    // This describes the underlying type of the parameter (string, int...)
    #[serde(rename = "type")]
    parameter_type: String, // Should be Enum; alphabetically, this is 'type'
}


// An Action is a custom action in an invocation image.
//
// For example, an invocation image may provide help text by creating a 'help'
// action that, when triggered, prints help text to STDOUT.
#[derive(Debug, Serialize, Deserialize)]
struct Action {
    // Describes what this action does
    description: Option<String>,
    // If true, this action modifies the deployment, and should be tracked as a release.
    #[serde(default)]
    modifies: bool,
    // If true, this action does not require any state information to be injected
    //
    // For example, printing help text does not require an installation, credentials,
    // or paramters.
    #[serde(default)]
    stateless: bool,
}

// Describe a parameter
#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    // A description of a parameter
    description: Option<String>,
}

// Destination describes where, in the invocation image, a particular paramter value should be
// placed.
//
// A parameter value can be placed into an environment variable (`env`) or a file at
// a particular location on the filesystem (`path`). This is a non-exclusive or, meaining
// that the same paramter can be written to both an env var and a path.
#[derive(Debug, Serialize, Deserialize)]
struct Destination {
    // The name of the destination environment variable
    env: Option<String>,
    // The fully qualified path to the destination file
    path: Option<String>,
}

#[cfg(test)]
mod tests;
