use crate::{
    database::{ AppState },
    response::{success_response},
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

use sqlx::mysql::MySqlRow;
use sqlx::query_as;

use crate::{
    controllers:: {
        user::{
            login,
            // login:: {
            //     login,
            //     rType:: {
            //         RequestBody_RTypes,
            //         FindUser_RType,
            //     }
            // },
            registration,
            // registration:: {
            //     registration,
            //     rType:: {
            //         RequestBody_RTypes,
            //         FindUser_RType,
            //     }
            // }
        }
    }
};

#[post("/login")]
pub async fn users_login_handler( opts: web::Json<login::rType::RequestBody_RTypes>, data: web::Data<AppState> ) -> impl Responder {
    match login::login::users_login(opts, data).await {
        Ok(users) => users,
        Err(response) => response,
    }
}

#[post("/registration")]
pub async fn users_registration_handler( opts: web::Json<registration::rType::RequestBody_RTypes>, data: web::Data<AppState> ) -> impl Responder {
    match registration::registration::users_registration(opts, data).await {
        Ok(users) => users,
        Err(response) => response,
    }
}




pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(users_login_handler)
        .service(users_registration_handler);

    conf.service(scope);
}
