/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.3-alpha.8
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitSelfServiceSettingsFlowWithPasswordMethodBody {
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method  Should be set to password when trying to update a password.
    #[serde(rename = "method")]
    pub method: Method,
    /// Password is the updated password
    #[serde(rename = "password")]
    pub password: String,
}

impl SubmitSelfServiceSettingsFlowWithPasswordMethodBody {
    pub fn new(method: Method, password: String) -> SubmitSelfServiceSettingsFlowWithPasswordMethodBody {
        SubmitSelfServiceSettingsFlowWithPasswordMethodBody {
            csrf_token: None,
            method,
            password,
        }
    }
}

/// Method  Should be set to password when trying to update a password.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "oidc")]
    Oidc,
}

