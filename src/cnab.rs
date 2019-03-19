use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Bundle implements a CNAB bundle descriptor
///
/// Bundle descriptors describe the properties of a bundle, including which images
/// are associated, what parameters and credentials are configurable, and whether there
/// are any additional traget actions that can be executed on this bundle.
///
/// The fields here are in canonical order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle<'a> {
    /// The list of additional actions that this bundle can perform.
    ///
    /// 'install', 'upgrade', and 'uninstall' are default actions, but additional actions
    /// may be defined here.
    pub actions: Option<HashMap<&'a str, Action<'a>>>,
    /// The list of configurable credentials.
    ///
    /// Credentials are injected into the bundle's invocation image at startup time.
    pub credentials: Option<Vec<Credential<'a>>>,
    /// This field allows for additional data to described in the bundle.
    ///
    /// This data should be stored in key/value pairs, where the value is undefined by
    /// the specification (but must be representable as JSON).
    pub custom: Option<HashMap<&'a str, serde_json::Value>>,
    /// description is a short description of this bundle
    pub description: Option<&'a str>,
    /// The list of images that comprise this bundle.
    ///
    /// Each image here is considered a constituent of the application described by this
    /// bundle.
    pub images: Option<HashMap<&'a str, Image<'a>>>,
    /// inovcation_images is the list of available bootstrapping images for this bundle
    ///
    /// Only one ought to be executed.
    pub invocation_images: Vec<Image<'a>>,
    /// keywords is a list of keywords describing this bundle
    pub keywords: Option<Vec<&'a str>>,
    /// license is the license of this bundle
    pub license: Option<&'a str>,
    /// maintainers is a list of maintainers responsible for this bundle
    pub maintainers: Option<Vec<Maintainer<'a>>>,
    /// name is the name of the bundle
    pub name: &'a str,
    /// The collection of parameters that can be passed into this bundle.
    ///
    /// Parameters can be injected into a bundle during startup time.
    pub parameters: Option<HashMap<&'a str, Parameter<'a>>>,
    /// schema_version is the version of the CNAB specification used to describe this
    pub schema_version: &'a str,
    /// version is the version of the bundle
    pub version: &'a str,
}

/// Represents a bundle.
impl<'a> Bundle<'a> {
    ///fn new(name: String, version: String) -> Bundle {}
    pub fn from_string(json_data: &'a str) -> Result<Bundle<'a>, serde_json::Error> {
        let res: Bundle = serde_json::from_str(json_data)?;
        Ok(res)
    }

    pub fn from_file(file_path: &str) -> Result<Bundle, serde_json::Error> {
        let file = File::open(Path::new(&file_path)).expect("file not found");
        let res: Bundle = serde_json::from_reader(file)?;
        Ok(res)
    }
}

/// Maintainer describes a bundle mainainer.
///
/// The name field is required, though the format of its value is unspecified.
#[derive(Debug, Serialize, Deserialize)]
pub struct Maintainer<'a> {
    /// The email address of the maintainer
    pub email: Option<&'a str>,
    /// The name of the maintainer
    pub name: &'a str,
    /// A URL with more information about the maintainer
    pub url: Option<&'a str>,
}

/// Image describes a CNAB image.
///
/// Both invocation images and regular images can be described using this object.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image<'a> {
    /// A digest to be used to verify the integrity of the image
    pub digest: Option<&'a str>,
    /// The image, as a string of the form REPO/NAME:TAG@SHA
    pub image: &'a str,
    /// The type of image. Typically, this is treated as an OCI Image
    pub image_type: Option<&'a str>,
    /// The media type of the image
    pub media_type: Option<&'a str>,
    /// The platform this image may be deployed on
    pub platform: Option<Platform<'a>>,
    /// The size in bytes of the image
    pub size: Option<i64>,
}

/// Platform defines a platform as a machine architecture plus and operating system
#[derive(Debug, Serialize, Deserialize)]
pub struct Platform<'a> {
    /// The architecture
    ///
    /// Typical values are amd64, i386, and arm64
    pub arch: Option<&'a str>,
    /// The operating system.
    ///
    /// Typical values are darwin, windows, and linux
    pub os: Option<&'a str>,
}

/// Credential describes a particular credential that may be injected into a bundle
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential<'a> {
    /// The description of this credential
    pub description: Option<&'a str>,
    /// The name of the environment variable into which the value will be placed
    pub env: Option<&'a str>,
    /// The fully qualified path into which the value will be placed
    pub path: Option<&'a str>,
}

/// Parameter describes a parameter that will be put into the invocation image
///
/// Paramters are injected into the invocation image at startup time
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter<'a> {
    /// The actions to which this parameter applies.
    ///
    /// If unset, this parameter will be applied to all actions.
    pub apply_to: Option<Vec<&'a str>>,
    /// The location where this parameter will be injected in the invocation image
    pub destination: Destination<'a>,
    /// This parameter's default value
    pub default_value: Option<serde_json::Value>,

    /// An enumeration of allowed values
    #[serde(rename = "enum")]
    pub allowed_values: Option<Vec<serde_json::Value>>,
    /// alphabetically, this is 'enum'
    /// The exclusive maximum.
    ///
    /// If unspecified, no exclusive max is applied
    pub exclusive_maximum: Option<i64>,
    /// The exclusive minimum.
    ///
    /// If unspecified, no exclusive min is applied
    pub exclusive_minimum: Option<i64>,
    /// The maximum
    ///
    /// If unspecieid, the maximum 64-bit integer value is applied
    pub maximum: Option<i64>,
    /// The maximum length of a string value
    ///
    /// If unspecified, no max is applied.
    pub max_length: Option<i64>,
    /// Additional parameter information
    pub metadata: Option<Metadata<'a>>,
    /// The minimum integer value
    ///
    /// If unspecified, the minimum 64-bit integer value is applied
    pub minimum: Option<i64>,
    /// The minimum string length
    pub min_length: Option<i64>,
    /// A regular expression (as defined in ECMAScript)
    ///
    /// If it is not matched, a string parameter value will be rejected
    pub pattern: Option<&'a str>,
    /// Indicate whether this parameter is required
    ///
    /// Default is false.
    #[serde(default)]
    pub required: bool,
    /// This describes the underlying type of the parameter (string, int...)
    #[serde(rename = "type")]
    pub parameter_type: &'a str, // Should be Enum; alphabetically, this is 'type'
}

/// An Action is a custom action in an invocation image.
///
/// For example, an invocation image may provide help text by creating a 'help'
/// action that, when triggered, prints help text to STDOUT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Action<'a> {
    /// Describes what this action does
    pub description: Option<&'a str>,
    /// If true, this action modifies the deployment, and should be tracked as a release.
    #[serde(default)]
    pub modifies: bool,
    /// If true, this action does not require any state information to be injected
    ///
    /// For example, printing help text does not require an installation, credentials,
    /// or paramters.
    #[serde(default)]
    pub stateless: bool,
}

/// Describe a parameter
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata<'a> {
    /// A description of a parameter
    pub description: Option<&'a str>,
}

/// Destination describes where, in the invocation image, a particular paramter value should be
/// placed.
///
/// A parameter value can be placed into an environment variable (`env`) or a file at
/// a particular location on the filesystem (`path`). This is a non-exclusive or, meaining
/// that the same paramter can be written to both an env var and a path.
#[derive(Debug, Serialize, Deserialize)]
pub struct Destination<'a> {
    /// The name of the destination environment variable
    pub env: Option<&'a str>,
    /// The fully qualified path to the destination file
    pub path: Option<&'a str>,
}
