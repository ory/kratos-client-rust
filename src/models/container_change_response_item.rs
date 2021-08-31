/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.3-alpha.3
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// ContainerChangeResponseItem : ContainerChangeResponseItem change item in response to ContainerChanges operation



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerChangeResponseItem {
    /// Kind of change
    #[serde(rename = "Kind")]
    pub kind: i32,
    /// Path to file that has changed
    #[serde(rename = "Path")]
    pub path: String,
}

impl ContainerChangeResponseItem {
    /// ContainerChangeResponseItem change item in response to ContainerChanges operation
    pub fn new(kind: i32, path: String) -> ContainerChangeResponseItem {
        ContainerChangeResponseItem {
            kind,
            path,
        }
    }
}


