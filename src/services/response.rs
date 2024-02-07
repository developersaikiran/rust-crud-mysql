use serde::Serialize;
use crate::model::User;

#[derive(Serialize, Debug)]
// pub struct success_response<T> {
//     pub status: i32, // 200
//     pub success: bool, // true
//     pub message: usize, // success message
//     pub data: T, // array or object return
// }
pub struct ResponseInterface<T> {
    pub status: i32, // 200
    pub success: bool, // true
    pub message: String, // success message
    pub data: T, // array or object return
}


pub fn success_response<T>(data: T, message: &str) -> Result<ResponseInterface<T>, &'static str> {
    Ok(ResponseInterface {
        status: 200,
        success: true,
        message: message.to_string(),
        data,
    })
}

pub fn server_error_response<T>(data: T, message: &str) -> Result<ResponseInterface<T>, &'static str> {
    Ok(ResponseInterface {
        status: 500,
        success: false,
        message: "failed".to_string(),
        data,
    })
}