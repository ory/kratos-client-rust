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

/// AuthenticatorAssuranceLevel : The authenticator assurance level can be one of \"aal1\", \"aal2\", or \"aal3\". A higher number means that it is harder for an attacker to compromise the account.  Generally, \"aal1\" implies that one authentication factor was used while AAL2 implies that two factors (e.g. password + TOTP) have been used.  To learn more about these levels please head over to: https://www.ory.sh/kratos/docs/concepts/credentials
/// The authenticator assurance level can be one of \"aal1\", \"aal2\", or \"aal3\". A higher number means that it is harder for an attacker to compromise the account.  Generally, \"aal1\" implies that one authentication factor was used while AAL2 implies that two factors (e.g. password + TOTP) have been used.  To learn more about these levels please head over to: https://www.ory.sh/kratos/docs/concepts/credentials
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthenticatorAssuranceLevel {
    #[serde(rename = "aal0")]
    Aal0,
    #[serde(rename = "aal1")]
    Aal1,
    #[serde(rename = "aal2")]
    Aal2,
    #[serde(rename = "aal3")]
    Aal3,

}

impl std::fmt::Display for AuthenticatorAssuranceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Aal0 => write!(f, "aal0"),
            Self::Aal1 => write!(f, "aal1"),
            Self::Aal2 => write!(f, "aal2"),
            Self::Aal3 => write!(f, "aal3"),
        }
    }
}

impl Default for AuthenticatorAssuranceLevel {
    fn default() -> AuthenticatorAssuranceLevel {
        Self::Aal0
    }
}

