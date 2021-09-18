/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.6-alpha.3
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfServiceLogoutUrl {
    /// LogoutURL can be opened in a browser to  format: uri
    #[serde(rename = "logout_url")]
    pub logout_url: String,
}

impl SelfServiceLogoutUrl {
    pub fn new(logout_url: String) -> SelfServiceLogoutUrl {
        SelfServiceLogoutUrl {
            logout_url,
        }
    }
}


