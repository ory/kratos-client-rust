/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v1.3.6-alpha.1
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiNodeImageAttributes {
    /// Height of the image
    #[serde(rename = "height")]
    pub height: i64,
    /// A unique identifier
    #[serde(rename = "id")]
    pub id: String,
    /// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0.  In this struct it technically always is \"img\". text Text input Input img Image a Anchor script Script
    #[serde(rename = "node_type")]
    pub node_type: NodeTypeEnum,
    /// The image's source URL.  format: uri
    #[serde(rename = "src")]
    pub src: String,
    /// Width of the image
    #[serde(rename = "width")]
    pub width: i64,
}

impl UiNodeImageAttributes {
    pub fn new(height: i64, id: String, node_type: NodeTypeEnum, src: String, width: i64) -> UiNodeImageAttributes {
        UiNodeImageAttributes {
            height,
            id,
            node_type,
            src,
            width,
        }
    }
}
/// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0.  In this struct it technically always is \"img\". text Text input Input img Image a Anchor script Script
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

