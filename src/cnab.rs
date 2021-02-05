use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Bundle implements a CNAB bundle descriptor
///
/// Bundle descriptors describe the properties of a bundle, including which images
/// are associated, what parameters and credentials are configurable, and whether there
/// are any additional target actions that can be executed on this bundle.
///
/// The fields here are in canonical order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    /// The list of additional actions that this bundle can perform.
    ///
    /// 'install', 'upgrade', and 'uninstall' are default actions, but additional actions
    /// may be defined here.
    pub actions: Option<BTreeMap<String, Action>>,
    /// The list of configurable credentials.
    ///
    /// Credentials are injected into the bundle's invocation image at startup time.
    pub credentials: Option<BTreeMap<String, Credential>>,
    /// This field allows for additional data to described in the bundle.
    ///
    /// This data should be stored in key/value pairs, where the value is undefined by
    /// the specification (but must be representable as JSON).
    pub custom: Option<BTreeMap<String, serde_json::Value>>,

    /// The JSON Schemata describing the parameters
    ///
    /// TODO: Should use a suitable Rust library as the target for this.
    pub definitions: Option<BTreeMap<String, serde_json::Value>>,

    /// description is a short description of this bundle
    pub description: Option<String>,
    /// The list of images that comprise this bundle.
    ///
    /// Each image here is considered a constituent of the application described by this
    /// bundle.
    pub images: Option<BTreeMap<String, Image>>,
    /// The list of available bootstrapping images for this bundle
    ///
    /// Only one ought to be executed.
    pub invocation_images: Vec<InvocationImage>,
    /// A list of keywords describing this bundle
    pub keywords: Option<Vec<String>>,
    /// The SPDX license identifier of this bundle
    pub license: Option<String>,
    /// A list of maintainers responsible for this bundle
    pub maintainers: Option<Vec<Maintainer>>,
    /// The name of the bundle
    pub name: String,
    /// The name/value pairs of outputs that this bundle produces.
    pub outputs: Option<BTreeMap<String, Output>>,
    /// The collection of parameters that can be passed into this bundle.
    ///
    /// Parameters can be injected into a bundle during startup time.
    pub parameters: Option<BTreeMap<String, Parameter>>,
    /// schema_version is the version of the CNAB specification used to describe this
    pub schema_version: String,
    /// version is the version of the bundle
    pub version: Version,
}

/// Represents a bundle.
impl Bundle {
    /// A convenience function to open and deserialize a [`bundle.json`](https://github.com/deislabs/cnab-spec/blob/master/101-bundle-json.md) file.
    ///
    /// ```
    /// use libcnab::Bundle;
    ///
    /// let bundle = Bundle::from_file("testdata/bundle.json").unwrap();
    /// assert_eq!(bundle.name, "helloworld");
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, BundleParseError> {
        let file = File::open(path)?;
        Self::from_json(file)
    }

    /// Deserialize a `Bundle` from any type implementing `Read`.
    pub fn from_json<R: Read>(reader: R) -> Result<Self, BundleParseError> {
        let bundle = serde_json::from_reader(reader)?;
        Ok(bundle)
    }
}

impl FromStr for Bundle {
    type Err = serde_json::Error;

    fn from_str(json_data: &str) -> Result<Self, Self::Err> {
        let bundle = serde_json::from_str(json_data)?;
        Ok(bundle)
    }
}

/// Represents an error parsing a bundle descriptor
///
/// This captures the various errors that may bubble up when a bundle descriptor
/// fails to parse.
#[derive(Debug)]
pub enum BundleParseError {
    SerdeJSONError(serde_json::Error),
    IoError(std::io::Error),
}

impl From<std::io::Error> for BundleParseError {
    fn from(error: std::io::Error) -> Self {
        BundleParseError::IoError(error)
    }
}

impl From<serde_json::Error> for BundleParseError {
    fn from(error: serde_json::Error) -> Self {
        BundleParseError::SerdeJSONError(error)
    }
}

/// Maintainer describes a bundle maintainer.
///
/// The name field is required, though the format of its value is unspecified.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Maintainer {
    /// The email address of the maintainer
    pub email: Option<String>,
    /// The name of the maintainer
    pub name: String,
    /// A URL with more information about the maintainer
    pub url: Option<String>,
}

/// Image describes a CNAB image.
///
/// Both invocation images and regular images can be described using this object.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// A description of the purpose of this image
    pub description: Option<String>,
    /// A digest to be used to verify the integrity of the image
    /// A cryptographic hash digest of the contents of the image that can be used to validate the image. This may be interpreted differently based on imageType
    pub content_digest: Option<String>,
    /// A resolvable reference to the image. This may be interpreted differently based on imageType, but the default is to treat this as an OCI image
    pub image: String,
    /// The type of image. If not specified, this is treated as an OCI Image (`oci`)
    pub image_type: Option<String>,
    /// The media type of the image
    pub media_type: Option<String>,
    /// The platform this image may be deployed on
    pub platform: Option<Platform>,
    /// The size in bytes of the image
    pub size: Option<i64>,
    /// Key/value pairs that used to specify identifying attributes of images
    pub labels: Option<BTreeMap<String, String>>,
}

/// InvocationImage describes a bootstrapping image for a CNAB bundle.
///
/// In the final CNAB Core 1.0 spec, this is subtly different than the regular Image type.
///
/// This conforms to the CNAB Core 1.0 specification
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvocationImage {
    /// A digest to be used to verify the integrity of the image
    /// A cryptographic hash digest of the contents of the image that can be used to validate the image. This may be interpreted differently based on imageType
    ///
    /// The specification requires this field _at installation time_, but not during development. Thus it is optional, and the runtime must validate whether
    /// the circumstances require a value here.
    pub content_digest: Option<String>,
    /// A resolvable reference to the image. This may be interpreted differently based on imageType, but the default is to treat this as an OCI image
    pub image: String,
    /// The type of image. If not specified, this is treated as an OCI Image (`oci`)
    ///
    /// The spec lists this field as required, but with a defined default. We interpret that to mean that if None, then `oci`.
    pub image_type: Option<String>,
    /// The media type of the image
    pub media_type: Option<String>,
    /// The size in bytes of the image
    pub size: Option<i64>,
    /// Key/value pairs that used to specify identifying attributes of images
    pub labels: Option<BTreeMap<String, String>>,
}

/// Platform defines a platform as a machine architecture plus and operating system
#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    /// The architecture
    ///
    /// Typical values are amd64, i386, and arm64
    pub arch: Option<String>,
    /// The operating system.
    ///
    /// Typical values are darwin, windows, and linux
    pub os: Option<String>,
}

/// Credential describes a particular credential that may be injected into a bundle
///
/// Satisfies the CNAB Core 1.0 specification
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    /// The description of this credential
    pub description: Option<String>,
    /// The name of the environment variable into which the value will be placed
    pub env: Option<String>,
    /// The fully qualified path into which the value will be placed
    pub path: Option<PathBuf>,
    /// Indicates whether this credential must be supplied. None is interpreted as "Some(false)".
    pub required: Option<bool>,
}

/// Parameter describes a parameter that will be put into the invocation image
///
/// Paramters are injected into the invocation image at startup time
///
/// Conforms to CNAB Core 1.0
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// The actions to which this parameter applies.
    ///
    /// If unset, this parameter will be applied to all actions.
    pub apply_to: Option<Vec<String>>,
    /// The name of a definition that describes the schema structure of this parameter
    pub definition: Option<String>,
    /// Human readable description of what this parameter does
    pub description: Option<String>,
    /// This describes the underlying type of the parameter (string, int...)
    /// The location where this parameter will be injected in the invocation image
    pub destination: Destination,
    /// Indicate whether this parameter is required
    ///
    /// None is treated as Some<false>
    pub required: Option<bool>,
}

/// An Action is a custom action in an invocation image.
///
/// For example, an invocation image may provide help text by creating a 'help'
/// action that, when triggered, prints help text to STDOUT.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Action {
    /// Describes what this action does
    pub description: Option<String>,
    /// If true, this action modifies the deployment, and should be tracked as a release.
    #[serde(default)]
    pub modifies: bool,
    /// If true, this action does not require any state information to be injected
    ///
    /// For example, printing help text does not require an installation, credentials,
    /// or parameters.
    #[serde(default)]
    pub stateless: bool,
}

/// Describe a parameter
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// A description of a parameter
    pub description: Option<String>,
}

/// Destination describes where, in the invocation image, a particular parameter value should be
/// placed.
///
/// A parameter value can be placed into an environment variable (`env`) or a file at
/// a particular location on the filesystem (`path`). This is a non-exclusive or, meaning
/// that the same paramter can be written to both an env var and a path.
#[derive(Debug, Serialize, Deserialize)]
pub struct Destination {
    /// The name of the destination environment variable
    pub env: Option<String>,
    /// The fully qualified path to the destination file
    pub path: Option<PathBuf>,
}

/// A value that is produced by running an invocation image
///
/// Complies to CNAB Core 1.0
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    /// An optional exhaustive list of actions producing this output
    pub apply_to: Option<Vec<String>>,
    /// The name of a definition that describes the schema structure of this output
    pub definition: String,
    /// Human-readable description of this output
    pub description: Option<String>,
    /// The path inside of the invocation image where output will be written
    pub path: Option<PathBuf>,
}
