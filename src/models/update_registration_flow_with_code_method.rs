/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v1.3.6
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateRegistrationFlowWithCodeMethod : Update Registration Flow with Code Method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRegistrationFlowWithCodeMethod {
    /// The OTP Code sent to the user
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The CSRF Token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method to use  This field must be set to `code` when using the code method.
    #[serde(rename = "method")]
    pub method: String,
    /// Resend restarts the flow with a new code
    #[serde(rename = "resend", skip_serializing_if = "Option::is_none")]
    pub resend: Option<String>,
    /// The identity's traits
    #[serde(rename = "traits")]
    pub traits: serde_json::Value,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}

impl UpdateRegistrationFlowWithCodeMethod {
    /// Update Registration Flow with Code Method
    pub fn new(method: String, traits: serde_json::Value) -> UpdateRegistrationFlowWithCodeMethod {
        UpdateRegistrationFlowWithCodeMethod {
            code: None,
            csrf_token: None,
            method,
            resend: None,
            traits,
            transient_payload: None,
        }
    }
}

