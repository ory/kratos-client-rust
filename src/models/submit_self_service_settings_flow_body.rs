/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.3-alpha.6
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum SubmitSelfServiceSettingsFlowBody {
    #[serde(rename="oidc")]
    SubmitSelfServiceSettingsFlowWithOidcMethodBody {
        /// Flow ID is the flow's ID.  in: query
        #[serde(rename = "flow", skip_serializing_if = "Option::is_none")]
        flow: Option<String>,
        /// Link this provider  Either this or `unlink` must be set.  type: string in: body
        #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
        link: Option<String>,
        /// Unlink this provider  Either this or `link` must be set.  type: string in: body
        #[serde(rename = "unlink", skip_serializing_if = "Option::is_none")]
        unlink: Option<String>,
    },
    #[serde(rename="password")]
    SubmitSelfServiceSettingsFlowWithPasswordMethodBody {
        /// CSRFToken is the anti-CSRF token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        csrf_token: Option<String>,
        /// Password is the updated password
        #[serde(rename = "password")]
        password: String,
    },
    #[serde(rename="profile")]
    SubmitSelfServiceSettingsFlowWithProfileMethodBody {
        /// The Anti-CSRF Token  This token is only required when performing browser flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        csrf_token: Option<String>,
        /// Traits contains all of the identity's traits.
        #[serde(rename = "traits")]
        traits: serde_json::Value,
    },
}




