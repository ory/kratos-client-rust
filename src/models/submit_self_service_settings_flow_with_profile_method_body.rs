/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.3-alpha.2
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// SubmitSelfServiceSettingsFlowWithProfileMethodBody : nolint:deadcode,unused



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitSelfServiceSettingsFlowWithProfileMethodBody {
    /// The Anti-CSRF Token  This token is only required when performing browser flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method  Should be set to profile when trying to update a profile.
    #[serde(rename = "method")]
    pub method: String,
    /// Traits contains all of the identity's traits.
    #[serde(rename = "traits")]
    pub traits: serde_json::Value,
}

impl SubmitSelfServiceSettingsFlowWithProfileMethodBody {
    /// nolint:deadcode,unused
    pub fn new(method: String, traits: serde_json::Value) -> SubmitSelfServiceSettingsFlowWithProfileMethodBody {
        SubmitSelfServiceSettingsFlowWithProfileMethodBody {
            csrf_token: None,
            method,
            traits,
        }
    }
}


