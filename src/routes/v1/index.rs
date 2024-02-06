use actix_web::{web, HttpResponse};

use super::{users, orders};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1")
    .configure(users::config)
    .configure(orders::config));
}