use crate::{
    model::{AppState, QueryOptions },
    response::{server_error_response, success_response, bad_request_response},
};

use actix_web::{
    web,
    HttpResponse,
};

use super::rType::{
    RequestBody_RTypes,
    FindUser_RType,
};

pub async fn users_registration( opts: web::Json<RequestBody_RTypes>, data: web::Data<AppState> ) -> Result<HttpResponse, HttpResponse> {
    let pool = data.db.lock().unwrap();
    let email = &opts.email;
    let password = &opts.password;
    println!("email: {}", email);

    let users_result = sqlx::query_as!(
        FindUser_RType,
        r#"
        SELECT email, password FROM users 
        WHERE email = ?"#, email
    )
    .fetch_one(&*pool)
    .await;

    match users_result {
        Ok(users) => {

            // return Ok(HttpResponse::InternalServerError().json(server_error_response({}, "User is already exits")));

            // if users.name == String::from("saikiran") {
            //     println!("This is my data: {:?}", users);
            //     return Ok(HttpResponse::Ok().json(server_error_response({}, "User is already exits")));
            // } 

            // let json_response = match success_response(users, "success") {
            //     Ok(response) => response,
            //     Err(err) => {
            //         eprintln!("Error creating success response: {}", err);
            //         return Err(HttpResponse::InternalServerError().finish());
            //     }
            // };

            if users.password == password {
                Ok(HttpResponse::Ok().json(success_response(users, "success")))
            } else{
                Ok(HttpResponse::BadRequest().json(bad_request_response({}, "Invalid password", {})))
            }

        }
        Err(err) => {
            eprintln!("Error fetching users: {:?}", err);
            // Err(HttpResponse::InternalServerError().finish())

            Err(HttpResponse::BadRequest().json(bad_request_response({}, "User is not exists with this email.", err.to_string())))
        }
    }
}
