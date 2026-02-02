/*
 *  main.rs
 *  backend_code
 *
 *  Created by Joel Lopes Da Silva on 1/25/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use actix_cors::Cors;
use actix_web::{ http::header, web, App, HttpResponse, HttpServer, Responder };
#[allow(unused_imports)]
use reqwest::Client as HttpClient;
#[allow(unused_imports)]
use async_trait::async_trait;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;
use serde::{ Serialize, Deserialize };

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

#[allow(dead_code)]
impl Database {

    // Initialization

    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }


    // Tasks management

    fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn get_task(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn insert_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn remove_task(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    fn update_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }


    // Users management

    fn get_all_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    fn get_user(&self, id: &u64) -> Option<&User> {
        self.users.get(id)
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == username)
    }

    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn remove_user(&mut self, id: &u64) {
        self.users.remove(id);
    }

    fn update_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }


    // Persistence

    fn load_from_file() -> std::io::Result<Self> {
        let file_content = fs::read_to_string("database.json")?;
        let database: Self = serde_json::from_str(&file_content)?;
        Ok(database)
    }

    fn save_to_file(&self) -> std::io::Result<()> {
        let serialized_database = serde_json::to_string(&self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(serialized_database.as_bytes())?;
        Ok(())
    }

    fn remove_saved_database_file() -> std::io::Result<()> {
        fs::remove_file("database.json")?;
        Ok(())
    }

}

struct AppState {
    database: Mutex<Database>
}

async fn create_task(
    app_state: web::Data<AppState>,
    task: web::Json<Task>,
) -> impl Responder {
    match app_state.database.lock() {
        Ok(mut database) => {
            database.insert_task(task.into_inner());
            _ = database.save_to_file();
            HttpResponse::Ok().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn read_task(
    app_state: web::Data<AppState>,
    id: web::Path<u64>,
) -> impl Responder {
    match app_state.database.lock() {
        Ok(database) => {
            match database.get_task(&id.into_inner()) {
                Some(task) => HttpResponse::Ok().json(task),
                None => HttpResponse::NotFound().finish(),
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn read_all_task(
    app_state: web::Data<AppState>,
) -> impl Responder {
    match app_state.database.lock() {
        Ok(database) => {
            let tasks = database.get_all_tasks();
            HttpResponse::Ok().json(tasks)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_task(
    app_state: web::Data<AppState>,
    task: web::Json<Task>,
) -> impl Responder {
    match app_state.database.lock() {
        Ok(mut database) => {
            database.update_task(task.into_inner());
            _ = database.save_to_file();
            HttpResponse::Ok().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_task(
    app_state: web::Data<AppState>,
    id: web::Path<u64>,
) -> impl Responder {
    match app_state.database.lock() {
        Ok(mut database) => {
            database.remove_task(&id.into_inner());
            _ = database.save_to_file();
            HttpResponse::Ok().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn register(
    app_state: web::Data<AppState>,
    user: web::Json<User>,
) -> impl Responder {
    match app_state.database.lock() {
        Ok(mut database) => {
            database.insert_user(user.into_inner());
            _ = database.save_to_file();
            HttpResponse::Ok().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn login(
    app_state: web::Data<AppState>,
    user: web::Json<User>,
) -> impl Responder {
    match app_state.database.lock() {
        Ok(database) => {
            match database.get_user_by_name(&user.username) {
                Some(stored_user) if stored_user.password == user.password => {
                    HttpResponse::Ok().body("Logged in!")
                },
                _ => HttpResponse::BadRequest().body("Invalid username or password"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = match Database::load_from_file() {
        Ok(database) => database,
        Err(_) => Database::new(),
    };

    let data = web::Data::new(
        AppState {
            database: Mutex::new(database),
        }
    );

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _request_head| {
                        let is_localhost = origin.as_bytes()
                            .starts_with(b"http://localhost");
                        let is_null = origin == "null";
                        is_localhost || is_null
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone())
            .route("/task", web::post().to(create_task))
            .route("/task", web::get().to(read_all_task))
            .route("/task", web::put().to(update_task))
            .route("/task/{id}", web::get().to(read_task))
            .route("/task/{id}", web::delete().to(delete_task))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
