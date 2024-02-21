use crate::{
    database::{ AppState },
    response::{server_error_response, success_response, bad_request_response},
};

use actix_web::{
    web,
    HttpResponse,
};

use super::rType::{
    RequestBody_RTypes,
    FindUser_RType,
    CreateUser_RType,
};

// use solana_sdk::signature::Keypair;

pub async fn users_registration( opts: web::Json<RequestBody_RTypes>, data: web::Data<AppState> ) -> Result<HttpResponse, HttpResponse> {
    let pool = data.db.lock().unwrap();
    let name = opts.name.clone();
    let email = opts.email.clone();
    let password = opts.password.clone();
    let deviceToken = opts.deviceToken.clone();
    println!("email: {}", email);

    let users_result = sqlx::query_as!(
        FindUser_RType,
        r#"
        SELECT email FROM users 
        WHERE email = ?"#, email
    )
    .fetch_one(&*pool)
    .await;

    match users_result {
        Ok(_) => Ok(HttpResponse::BadRequest().json(
            bad_request_response({}, "Email is already exists", {}),
        )),
        Err(err) => {
            let create_user = sqlx::query!(
                r#"
                    INSERT INTO users (name, email, password, deviceToken) VALUES (?, ?, ?, ?)
                "#,
                name,
                email,
                password,
                deviceToken
            )
            .execute(&*pool)
            .await;


            match create_user {
                Ok(_) => {
                    // Fetch the user after successful insertion
                    let inserted_user = sqlx::query_as!(
                        CreateUser_RType,
                        r#"
                            SELECT id, name, email FROM users WHERE email = ?
                        "#,
                        email
                    )
                    .fetch_one(&*pool)
                    .await;

                    // // Generate a new keypair
                    // let keypair = Keypair::new();

                    // // Print the public key and private key
                    // println!("Public Key: {:?}", keypair.pubkey());
                    // println!("Private Key: {:?}", keypair.to_bytes());
                    
                    match inserted_user {
                        Ok(user) => Ok(HttpResponse::Ok().json(success_response(user, "success"))),
                        Err(err) => {
                            eprintln!("Error fetching user: {:?}", err);
                            Err(HttpResponse::InternalServerError().finish())
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error creating user: {:?}", err);
                    Err(HttpResponse::InternalServerError().finish())
                }
            }
            // match create_user {
            //     Ok(users) => {
            //         // create token here
            //         Ok(HttpResponse::Ok().json(success_response(users, "success")))
            //     }
            //     Err(err) => {
            //         Ok(HttpResponse::InternalServerError().json(server_error_response({}, "Failed to create account, Please try again later.", err)))
            //     }
            // }

            // eprintln!("Error fetching users: {:?}", create_user);
            // // Err(HttpResponse::InternalServerError().finish())

            // Err(HttpResponse::BadRequest().json(bad_request_response({}, "User is not exists with this email.", err.to_string())))
        }
    }
}
