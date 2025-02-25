/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v1.3.7
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateSettingsFlowWithPasskeyMethod : Update Settings Flow with Passkey Method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsFlowWithPasskeyMethod {
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method  Should be set to \"passkey\" when trying to add, update, or remove a webAuthn pairing.
    #[serde(rename = "method")]
    pub method: String,
    /// Remove a WebAuthn Security Key  This must contain the ID of the WebAuthN connection.
    #[serde(rename = "passkey_remove", skip_serializing_if = "Option::is_none")]
    pub passkey_remove: Option<String>,
    /// Register a WebAuthn Security Key  It is expected that the JSON returned by the WebAuthn registration process is included here.
    #[serde(rename = "passkey_settings_register", skip_serializing_if = "Option::is_none")]
    pub passkey_settings_register: Option<String>,
}

impl UpdateSettingsFlowWithPasskeyMethod {
    /// Update Settings Flow with Passkey Method
    pub fn new(method: String) -> UpdateSettingsFlowWithPasskeyMethod {
        UpdateSettingsFlowWithPasskeyMethod {
            csrf_token: None,
            method,
            passkey_remove: None,
            passkey_settings_register: None,
        }
    }
}

