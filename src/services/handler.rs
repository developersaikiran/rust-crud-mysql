use crate::{
    model::{AppState, QueryOptions, UpdateUserSchema, User},
    response::{GenericResponse, SingleUserResponse, UserData, UserListResponse},
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

use sqlx::mysql::MySqlRow;
use sqlx::query_as;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/users")]
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
    // .map_err(|e| {
    //     eprintln!("Error fetching users: {:?}", e);
    //     HttpResponse::InternalServerError().finish()
    // })?;

    let users = match users_result {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let json_response = UserListResponse {
        status: "success".to_string(),
        results: users.len(),
        users,
    };

    HttpResponse::Ok().json(json_response)
}

// #[post("/users")]
// async fn create_user_handler(
//     mut body: web::Json<User>,
//     data: web::Data<AppState>,
// ) -> impl Responder {
//     let mut vec = data.user_db.lock().unwrap();

//     let user = vec.iter().find(|user| user.title == body.title);

//     if user.is_some() {
//         let error_response = GenericResponse {
//             status: "fail".to_string(),
//             message: format!("User with title: '{}' already exists", body.title),
//         };
//         return HttpResponse::Conflict().json(error_response);
//     }

//     let uuid_id = Uuid::new_v4();
//     let datetime = Utc::now();

//     body.id = Some(uuid_id.to_string());
//     body.completed = Some(false);
//     body.createdAt = Some(datetime);
//     body.updatedAt = Some(datetime);

//     let user = body.to_owned();

//     vec.push(body.into_inner());

//     let json_response = SingleUserResponse {
//         status: "success".to_string(),
//         data: UserData { user },
//     };

//     HttpResponse::Ok().json(json_response)
// }

// #[get("/users/{id}")]
// async fn get_user_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
//     let vec = data.user_db.lock().unwrap();

//     let id = path.into_inner();
//     let user = vec.iter().find(|user| user.id == Some(id.to_owned()));

//     if user.is_none() {
//         let error_response = GenericResponse {
//             status: "fail".to_string(),
//             message: format!("User with ID: {} not found", id),
//         };
//         return HttpResponse::NotFound().json(error_response);
//     }

//     let user = user.unwrap();
//     let json_response = SingleUserResponse {
//         status: "success 456".to_string(),
//         data: UserData { user: user.clone() },
//     };

//     HttpResponse::Ok().json(json_response)
// }

// #[patch("/users/{id}")]
// async fn edit_user_handler(
//     path: web::Path<String>,
//     body: web::Json<UpdateUserSchema>,
//     data: web::Data<AppState>,
//     ) -> impl Responder {
//     let mut vec = data.user_db.lock().unwrap();

//     let id = path.into_inner();
//     let user = vec.iter_mut().find(|user| user.id == Some(id.to_owned()));

//     if user.is_none() {
//         let error_response = GenericResponse {
//             status: "fail".to_string(),
//             message: format!("User with ID: {} not found", id),
//         };
//         return HttpResponse::NotFound().json(error_response);
//     }

//     let user = user.unwrap();
//     let datetime = Utc::now();
//     let title = body.title.to_owned().unwrap_or(user.title.to_owned());
//     let content = body.content.to_owned().unwrap_or(user.content.to_owned());
//     let payload = User {
//         id: user.id.to_owned(),
//         title: if !title.is_empty() {
//             title
//         } else {
//             user.title.to_owned()
//         },
//         content: if !content.is_empty() {
//             content
//         } else {
//             user.content.to_owned()
//         },
//         completed: if body.completed.is_some() {
//             body.completed
//         } else {
//             user.completed
//         },
//         createdAt: user.createdAt,
//         updatedAt: Some(datetime),
//     };
//     *user = payload;

//     let json_response = SingleUserResponse {
//         status: "success".to_string(),
//         data: UserData { user: user.clone() },
//     };

//     HttpResponse::Ok().json(json_response)
// }

// #[delete("/users/{id}")]
// async fn delete_user_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
//     let mut vec = data.user_db.lock().unwrap();

//     let id = path.into_inner();
//     let user = vec.iter_mut().find(|user| user.id == Some(id.to_owned()));

//     if user.is_none() {
//         let error_response = GenericResponse {
//             status: "fail".to_string(),
//             message: format!("User with ID: {} not found", id),
//         };
//         return HttpResponse::NotFound().json(error_response);
//     }

//     vec.retain(|user| user.id != Some(id.to_owned()));
//     HttpResponse::NoContent().finish()
// }

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(users_list_handler);
    // .service(create_user_handler)
    // .service(get_user_handler)
    // .service(edit_user_handler)
    // .service(delete_user_handler);

    conf.service(scope);
}
