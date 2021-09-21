/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.6-alpha.7
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// SuccessfulSelfServiceRegistrationWithoutBrowser : The Response for Registration Flows via API



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessfulSelfServiceRegistrationWithoutBrowser {
    #[serde(rename = "identity")]
    pub identity: Box<crate::models::Identity>,
    #[serde(rename = "session", skip_serializing_if = "Option::is_none")]
    pub session: Option<Box<crate::models::Session>>,
    /// The Session Token  This field is only set when the session hook is configured as a post-registration hook.  A session token is equivalent to a session cookie, but it can be sent in the HTTP Authorization Header:  Authorization: bearer ${session-token}  The session token is only issued for API flows, not for Browser flows!
    #[serde(rename = "session_token", skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

impl SuccessfulSelfServiceRegistrationWithoutBrowser {
    /// The Response for Registration Flows via API
    pub fn new(identity: crate::models::Identity) -> SuccessfulSelfServiceRegistrationWithoutBrowser {
        SuccessfulSelfServiceRegistrationWithoutBrowser {
            identity: Box::new(identity),
            session: None,
            session_token: None,
        }
    }
}


