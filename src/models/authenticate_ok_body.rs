/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.7.3-alpha.6
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// AuthenticateOkBody : AuthenticateOKBody authenticate o k body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticateOkBody {
    /// An opaque token used to authenticate a user after a successful login
    #[serde(rename = "IdentityToken")]
    pub identity_token: String,
    /// The status of the authentication
    #[serde(rename = "Status")]
    pub status: String,
}

impl AuthenticateOkBody {
    /// AuthenticateOKBody authenticate o k body
    pub fn new(identity_token: String, status: String) -> AuthenticateOkBody {
        AuthenticateOkBody {
            identity_token,
            status,
        }
    }
}


