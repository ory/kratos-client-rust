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

/// ContinueWithRecoveryUi : Indicates, that the UI flow could be continued by showing a recovery ui
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinueWithRecoveryUi {
    /// Action will always be `show_recovery_ui` show_recovery_ui ContinueWithActionShowRecoveryUIString
    #[serde(rename = "action")]
    pub action: ActionEnum,
    #[serde(rename = "flow")]
    pub flow: Box<models::ContinueWithRecoveryUiFlow>,
}

impl ContinueWithRecoveryUi {
    /// Indicates, that the UI flow could be continued by showing a recovery ui
    pub fn new(action: ActionEnum, flow: models::ContinueWithRecoveryUiFlow) -> ContinueWithRecoveryUi {
        ContinueWithRecoveryUi {
            action,
            flow: Box::new(flow),
        }
    }
}
/// Action will always be `show_recovery_ui` show_recovery_ui ContinueWithActionShowRecoveryUIString
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionEnum {
    #[serde(rename = "show_recovery_ui")]
    ShowRecoveryUi,
}

impl Default for ActionEnum {
    fn default() -> ActionEnum {
        Self::ShowRecoveryUi
    }
}

