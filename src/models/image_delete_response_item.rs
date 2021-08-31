/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.3-alpha.3
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// ImageDeleteResponseItem : ImageDeleteResponseItem image delete response item



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDeleteResponseItem {
    /// The image ID of an image that was deleted
    #[serde(rename = "Deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    /// The image ID of an image that was untagged
    #[serde(rename = "Untagged", skip_serializing_if = "Option::is_none")]
    pub untagged: Option<String>,
}

impl ImageDeleteResponseItem {
    /// ImageDeleteResponseItem image delete response item
    pub fn new() -> ImageDeleteResponseItem {
        ImageDeleteResponseItem {
            deleted: None,
            untagged: None,
        }
    }
}


