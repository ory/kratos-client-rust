/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v0.11.1
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityCredentialsOidcProvider {
    #[serde(rename = "initial_access_token", skip_serializing_if = "Option::is_none")]
    pub initial_access_token: Option<String>,
    #[serde(rename = "initial_id_token", skip_serializing_if = "Option::is_none")]
    pub initial_id_token: Option<String>,
    #[serde(rename = "initial_refresh_token", skip_serializing_if = "Option::is_none")]
    pub initial_refresh_token: Option<String>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

impl Default for IdentityCredentialsOidcProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityCredentialsOidcProvider {
    pub fn new() -> IdentityCredentialsOidcProvider {
        IdentityCredentialsOidcProvider {
                initial_access_token: None,
                initial_id_token: None,
                initial_refresh_token: None,
                provider: None,
                subject: None,
        }
    }
}


