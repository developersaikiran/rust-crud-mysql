use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestBody_RTypes {
    pub email: String,
    pub password: String,
    pub deviceToken: String,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FindUser_RType {
    pub email: String,
    pub password: String,
    // pub deviceToken: String,
}