/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v1.3.6-alpha.2
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IsAlive200Response {
    /// Always \"ok\".
    #[serde(rename = "status")]
    pub status: String,
}

impl IsAlive200Response {
    pub fn new(status: String) -> IsAlive200Response {
        IsAlive200Response {
            status,
        }
    }
}

