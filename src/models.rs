use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
    #[serde(rename = "displayname")]
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInVerifyRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInVerifyResponse {
    pub success: bool,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub timestamp: String,
    pub rpid: String,
    pub origin: String,
    pub device: String,
    pub country: String,
    pub nickname: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    pub token_id: String,
    #[serde(rename = "type")]
    pub stype: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCredentialRequest {
    #[serde(rename = "credentialId")]
    pub credential_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDescriptor {
    #[serde(rename = "type")]
    pub typec: String,
    pub id: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub descriptor: CredentialDescriptor,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "userHandle")]
    pub user_handle: String,
    #[serde(rename = "signatureCounter")]
    pub signature_counter: u64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "aaGuid")]
    pub aa_guid: String,
    #[serde(rename = "lastUsedAt")]
    pub last_used_at: String,
    pub rpid: String,
    pub origin: String,
    pub country: String,
    pub device: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AliasRequest{
    #[serde(rename = "userId")]
    pub user_id: String,
    pub hashing: bool,
    pub aliases: Vec<String>

}