/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.6-alpha.6
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// Session : A Session



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
    /// Whether or not the session is active.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The Session Authentication Timestamp  When this session was authenticated at.
    #[serde(rename = "authenticated_at", skip_serializing_if = "Option::is_none")]
    pub authenticated_at: Option<String>,
    /// The Session Expiry  When this session expires at.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "identity")]
    pub identity: Box<crate::models::Identity>,
    /// The Session Issuance Timestamp  When this session was authenticated at.
    #[serde(rename = "issued_at", skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
}

impl Session {
    /// A Session
    pub fn new(id: String, identity: crate::models::Identity) -> Session {
        Session {
            active: None,
            authenticated_at: None,
            expires_at: None,
            id,
            identity: Box::new(identity),
            issued_at: None,
        }
    }
}


