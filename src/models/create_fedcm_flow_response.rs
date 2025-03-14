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

/// CreateFedcmFlowResponse : Contains a list of all available FedCM providers.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFedcmFlowResponse {
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    #[serde(rename = "providers", skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<models::Provider>>,
}

impl CreateFedcmFlowResponse {
    /// Contains a list of all available FedCM providers.
    pub fn new() -> CreateFedcmFlowResponse {
        CreateFedcmFlowResponse {
            csrf_token: None,
            providers: None,
        }
    }
}

