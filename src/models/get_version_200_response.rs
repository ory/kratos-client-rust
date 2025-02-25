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
pub struct GetVersion200Response {
    /// The version of Ory Kratos.
    #[serde(rename = "version")]
    pub version: String,
}

impl GetVersion200Response {
    pub fn new(version: String) -> GetVersion200Response {
        GetVersion200Response {
            version,
        }
    }
}

