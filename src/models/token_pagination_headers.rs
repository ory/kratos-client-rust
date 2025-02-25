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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenPaginationHeaders {
    /// The link header contains pagination links.  For details on pagination please head over to the [pagination documentation](https://www.ory.sh/docs/ecosystem/api-design#pagination).  in: header
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The total number of clients.  in: header
    #[serde(rename = "x-total-count", skip_serializing_if = "Option::is_none")]
    pub x_total_count: Option<String>,
}

impl TokenPaginationHeaders {
    pub fn new() -> TokenPaginationHeaders {
        TokenPaginationHeaders {
            link: None,
            x_total_count: None,
        }
    }
}

