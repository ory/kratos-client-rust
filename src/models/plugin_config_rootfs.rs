/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.3-alpha.8
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// PluginConfigRootfs : PluginConfigRootfs plugin config rootfs



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigRootfs {
    /// diff ids
    #[serde(rename = "diff_ids", skip_serializing_if = "Option::is_none")]
    pub diff_ids: Option<Vec<String>>,
    /// type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl PluginConfigRootfs {
    /// PluginConfigRootfs plugin config rootfs
    pub fn new() -> PluginConfigRootfs {
        PluginConfigRootfs {
            diff_ids: None,
            _type: None,
        }
    }
}


