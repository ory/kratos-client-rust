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
pub struct UiNodeAnchorAttributes {
    /// The link's href (destination) URL.  format: uri
    #[serde(rename = "href")]
    pub href: String,
    /// A unique identifier
    #[serde(rename = "id")]
    pub id: String,
    /// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0.  In this struct it technically always is \"a\". text Text input Input img Image a Anchor script Script
    #[serde(rename = "node_type")]
    pub node_type: NodeTypeEnum,
    #[serde(rename = "title")]
    pub title: Box<models::UiText>,
}

impl UiNodeAnchorAttributes {
    pub fn new(href: String, id: String, node_type: NodeTypeEnum, title: models::UiText) -> UiNodeAnchorAttributes {
        UiNodeAnchorAttributes {
            href,
            id,
            node_type,
            title: Box::new(title),
        }
    }
}
/// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0.  In this struct it technically always is \"a\". text Text input Input img Image a Anchor script Script
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NodeTypeEnum {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "img")]
    Img,
    #[serde(rename = "a")]
    A,
    #[serde(rename = "script")]
    Script,
}

impl Default for NodeTypeEnum {
    fn default() -> NodeTypeEnum {
        Self::Text
    }
}

