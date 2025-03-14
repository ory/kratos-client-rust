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

/// RegistrationFlowState : The experimental state represents the state of a registration flow. This field is EXPERIMENTAL and subject to change!
/// The experimental state represents the state of a registration flow. This field is EXPERIMENTAL and subject to change!
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegistrationFlowState {
    #[serde(rename = "choose_method")]
    ChooseMethod,
    #[serde(rename = "sent_email")]
    SentEmail,
    #[serde(rename = "passed_challenge")]
    PassedChallenge,

}

impl std::fmt::Display for RegistrationFlowState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChooseMethod => write!(f, "choose_method"),
            Self::SentEmail => write!(f, "sent_email"),
            Self::PassedChallenge => write!(f, "passed_challenge"),
        }
    }
}

impl Default for RegistrationFlowState {
    fn default() -> RegistrationFlowState {
        Self::ChooseMethod
    }
}

