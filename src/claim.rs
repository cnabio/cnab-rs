use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// Implementation of CNAB Claims 1.0
///
/// This provides a struct that matches the CNAB Claims 1.0 specification at the
/// time when the CNAB Core 1.0 specification was finalized.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    /// The bundle descriptor
    pub bundle: crate::cnab::Bundle,
    /// Creation date
    pub created: DateTime<Utc>,
    /// Extension space
    pub custom: Option<serde_json::Value>,
    /// Modification date
    pub modified: DateTime<Utc>,
    /// The name of the claim (e.g. the release name)
    pub name: String,
    /// Name/value pairs representing the outputs from the runtime
    pub outputs: Option<BTreeMap<String, String>>,
    /// Name/value paris that represent the parameter values
    pub parameters: Option<BTreeMap<String, String>>,
    /// The results according to the Runtime
    pub result: Response,
    /// A ulid to track the revision
    pub revision: String,
    /// A canonical reference to the bundle used in the last action.
    /// When provided, the bundle reference SHOULD be digested to identify a specific version of
    /// the referenced bundle.
    pub bundle_reference: Option<String>,
}

/// Response represents the result of a CNAB operation, as described in a Claim.
///
/// Since 'result' is a technical term in Rust, this is called Response instead.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    action: String,
    message: Option<String>,
    status: Status,
}

/// Status is one of 'success', 'failure', or 'pending'
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Success,
    Failure,
    Pending,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_claim() {
        let claim: Claim = serde_json::from_str(
            r#"{
                "name": "claimtest",
                "bundle": {
                    "name": "aristotle",
                    "invocationImages": [],
                    "schemaVersion": "1.0.0",
                    "version": "1.0.0",
                    "custom": {
                        "com.example.praxis": {
                            "techne": true
                        }
                    }
                },
                "bundleReference": "hub.example.com/my/bundle@sha256:eeeeeeeee...",
                "created": "2018-08-30T20:39:55.549002887-06:00",
                "modified": "2018-08-30T20:39:55.549002887-06:00",
                "result": {
                    "action": "install",
                    "message": "installed wordpress",
                    "status": "success"
                },
                "outputs": {
                    "one": "output 1",
                    "two": "output 2"
                },
                "parameters": {
                    "one": "param one",
                    "two": "param two"
                },
                "revision": "01CP6XM0KVB9V1BQDZ9NK8VP29"
            }"#,
        )
        .expect("Successfully parsed claim");

        assert_eq!(claim.result.status, Status::Success);
    }
}
