use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
    // pub user_roles: Option<Vec<Tuple>>,
    pub user_roles: String,

    // pub user_roles_id: Option<i32>,
    // pub user_roles_user_id: Array,
    // pub user_roles_role_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserRole {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
}



pub struct AppState {
    pub db: Arc<Mutex<MySqlPool>>,
}

impl AppState {
    pub async fn init() -> Result<AppState, sqlx::Error> {
        dotenv().ok();
        
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        println!("database_url! {}", database_url);

        let pool = match MySqlPoolOptions::new().max_connections(10).connect(&database_url).await{
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };


        Ok(AppState {
            db: Arc::new(Mutex::new(pool)),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub skip: Option<usize>,
    pub limit: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UpdateUserSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}