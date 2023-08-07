/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v0.13.1
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateRegistrationFlowWithOidcMethod : Update Registration Flow with OpenID Connect Method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRegistrationFlowWithOidcMethod {
    /// The CSRF Token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method to use  This field must be set to `oidc` when using the oidc method.
    #[serde(rename = "method")]
    pub method: String,
    /// The provider to register with
    #[serde(rename = "provider")]
    pub provider: String,
    /// The identity traits
    #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
    pub traits: Option<serde_json::Value>,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
    /// UpstreamParameters are the parameters that are passed to the upstream identity provider.  These parameters are optional and depend on what the upstream identity provider supports. Supported parameters are: `login_hint` (string): The `login_hint` parameter suppresses the account chooser and either pre-fills the email box on the sign-in form, or selects the proper session. `hd` (string): The `hd` parameter limits the login/registration process to a Google Organization, e.g. `mycollege.edu`.
    #[serde(rename = "upstream_parameters", skip_serializing_if = "Option::is_none")]
    pub upstream_parameters: Option<serde_json::Value>,
}


impl UpdateRegistrationFlowWithOidcMethod {
    /// Update Registration Flow with OpenID Connect Method
    pub fn new(method: String, provider: String) -> UpdateRegistrationFlowWithOidcMethod {
        UpdateRegistrationFlowWithOidcMethod {
                csrf_token: None,
                method,
                provider,
                traits: None,
                transient_payload: None,
                upstream_parameters: None,
        }
    }
}

