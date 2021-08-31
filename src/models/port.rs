/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.3-alpha.3
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// Port : Port An open port on a container



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Port {
    /// IP
    #[serde(rename = "IP", skip_serializing_if = "Option::is_none")]
    pub IP: Option<String>,
    /// Port on the container
    #[serde(rename = "PrivatePort")]
    pub private_port: i32,
    /// Port exposed on the host
    #[serde(rename = "PublicPort", skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    /// type
    #[serde(rename = "Type")]
    pub _type: String,
}

impl Port {
    /// Port An open port on a container
    pub fn new(private_port: i32, _type: String) -> Port {
        Port {
            IP: None,
            private_port,
            public_port: None,
            _type,
        }
    }
}


