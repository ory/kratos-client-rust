/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v1.3.4
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OAuth2LoginRequest : OAuth2LoginRequest struct for OAuth2LoginRequest
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2LoginRequest {
    /// ID is the identifier (\\\"login challenge\\\") of the login request. It is used to identify the session.
    #[serde(rename = "challenge", skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client: Option<Box<models::OAuth2Client>>,
    #[serde(rename = "oidc_context", skip_serializing_if = "Option::is_none")]
    pub oidc_context: Option<Box<models::OAuth2ConsentRequestOpenIdConnectContext>>,
    /// RequestURL is the original OAuth 2.0 Authorization URL requested by the OAuth 2.0 client. It is the URL which initiates the OAuth 2.0 Authorization Code or OAuth 2.0 Implicit flow. This URL is typically not needed, but might come in handy if you want to deal with additional request parameters.
    #[serde(rename = "request_url", skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    #[serde(rename = "requested_access_token_audience", skip_serializing_if = "Option::is_none")]
    pub requested_access_token_audience: Option<Vec<String>>,
    #[serde(rename = "requested_scope", skip_serializing_if = "Option::is_none")]
    pub requested_scope: Option<Vec<String>>,
    /// SessionID is the login session ID. If the user-agent reuses a login session (via cookie / remember flag) this ID will remain the same. If the user-agent did not have an existing authentication session (e.g. remember is false) this will be a new random value. This value is used as the \\\"sid\\\" parameter in the ID Token and in OIDC Front-/Back- channel logout. It's value can generally be used to associate consecutive login requests by a certain user.
    #[serde(rename = "session_id", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Skip, if true, implies that the client has requested the same scopes from the same user previously. If true, you can skip asking the user to grant the requested scopes, and simply forward the user to the redirect URL.  This feature allows you to update / set session information.
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<bool>,
    /// Subject is the user ID of the end-user that authenticated. Now, that end user needs to grant or deny the scope requested by the OAuth 2.0 client. If this value is set and `skip` is true, you MUST include this subject type when accepting the login request, or the request will fail.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

impl OAuth2LoginRequest {
    /// OAuth2LoginRequest struct for OAuth2LoginRequest
    pub fn new() -> OAuth2LoginRequest {
        OAuth2LoginRequest {
            challenge: None,
            client: None,
            oidc_context: None,
            request_url: None,
            requested_access_token_audience: None,
            requested_scope: None,
            session_id: None,
            skip: None,
            subject: None,
        }
    }
}

