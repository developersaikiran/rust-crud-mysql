use crate::{
    model::{AppState, QueryOptions, UpdateUserSchema, User},
    response::{GenericResponse, SingleUserResponse, UserData, UserListResponse},
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

use sqlx::mysql::MySqlRow;
use sqlx::query_as;

#[get("/lists")]
pub async fn users_list_handler(
    opts: web::Query<QueryOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    
    let pool = data.db.lock().unwrap();
    let limit = opts.limit.unwrap_or(10);
    let skip = opts.skip.unwrap_or(0);
    
    let users_result = sqlx::query_as!(User, r#"SELECT id, name, email, password, createdAt, updatedAt FROM users LIMIT ? OFFSET ?"#, limit as i64, skip as i64)
    .fetch_all(&*pool)
    .await;
    let users = users_result.expect("Failed to fetch users from the database");

    let json_response = UserListResponse {
        status: "success".to_string(),
        results: users.len(),
        users,
    };

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/order")
        .service(users_list_handler);

    conf.service(scope);
}
