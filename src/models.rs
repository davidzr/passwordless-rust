use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest{
    
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
    #[serde(rename = "displayname")]
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse{
    pub token: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct  SignInVerifyRequest{
    pub token: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct  SignInVerifyResponse{
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

