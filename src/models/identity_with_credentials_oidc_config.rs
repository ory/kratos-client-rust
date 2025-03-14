/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v1.3.8
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityWithCredentialsOidcConfig {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::IdentityWithCredentialsPasswordConfig>>,
    /// A list of OpenID Connect Providers
    #[serde(rename = "providers", skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<models::IdentityWithCredentialsOidcConfigProvider>>,
}

impl IdentityWithCredentialsOidcConfig {
    pub fn new() -> IdentityWithCredentialsOidcConfig {
        IdentityWithCredentialsOidcConfig {
            config: None,
            providers: None,
        }
    }
}

