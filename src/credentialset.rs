use serde::{Serialize, Deserialize};
/// CredentialSet implements section 802 of the CNAB specification at the time CNAB Core 1.0 was finalized.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSet {
    pub name: String,
    pub credentials: Vec<Credential>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    name: String,
    source: CredentialSource,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSource {
    value: Option<String>,
    env: Option<String>,
    path: Option<std::path::PathBuf>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_credentialset() {
        let _: CredentialSet = serde_json::from_str(
            r#"{
                "name": "test_credentials",
                "credentials": [
                    {
                        "name": "kubeconfig",
                        "source": {
                            "path": "$HOME/.kube/config"
                        }
                    },
                    {
                        "name": "image_token",
                        "source": {
                            "value": "1234aaaaaaaaaaaa"
                        }
                    },
                    {
                        "name": "hostkey",
                        "source": {
                            "env": "HOSTKEY",
                            "path": "$HOME/.thing/hostkey"
                        }
                    }
                ]
            }"#
        ).expect("credential set parsed");
    }
}