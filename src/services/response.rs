use serde::Serialize;

use crate::model::User;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Debug)]
pub struct SingleUserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Debug)]
pub struct SuccessResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<User>,
}