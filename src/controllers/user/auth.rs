use crate::{
    model::{AppState, QueryOptions, UpdateUserSchema, User},
    response::{GenericResponse, SingleUserResponse, UserData, SuccessResponse},
};
use actix_web::{web, HttpResponse };


// pub async fn users_list_handler( opts: web::Query<QueryOptions>, data: web::Data<AppState> ) -> Result<SuccessResponse, HttpResponse> {
pub async fn users_list_handler( opts: web::Query<QueryOptions>, data: web::Data<AppState> ) -> Result<HttpResponse, HttpResponse> {

    let pool = data.db.lock().unwrap();
    let limit = opts.limit.unwrap_or(10);
    let skip = opts.skip.unwrap_or(0);
    
    let users_result = sqlx::query_as!(User, r#"SELECT id, name, email, password, createdAt, updatedAt FROM users LIMIT ? OFFSET ?"#, limit as i64, skip as i64)
    .fetch_all(&*pool)
    .await;



    // let users = users_result.expect("Failed to fetch users from the database");
    // Ok(SuccessResponse {
    //     status: "success".to_string(),
    //     results: users.len(),
    //     users,
    // })


    match users_result {
        Ok(users) => {
            let json_response = SuccessResponse {
                status: "success".to_string(),
                results: users.len(),
                data: users,
            };
            Ok(HttpResponse::Ok().json(json_response))
        }
        Err(err) => {
            eprintln!("Error fetching users: {:?}", err);
            Err(HttpResponse::InternalServerError().finish())
        }
    }
}