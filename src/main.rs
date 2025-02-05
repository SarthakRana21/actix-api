use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{error::ErrorNotFound, web::{self}, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {   
    name: String
}

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

#[actix_web::get("/user/{id}")]
async fn get_user(
    user_id: web::Path<u32>, 
    db: web::Data<UserDb>
) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();

    match db.get(&user_id) {
        Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        None => Err(ErrorNotFound("user not found"))
    }
}
#[actix_web::get("/")]
async fn home() -> impl Responder {
    format!("Welcome to Actix Web Server")
}
#[actix_web::post("/users")]
async fn create_user(
    user_data: web::Json<User>,
    db: web::Data<UserDb>
) -> impl Responder {
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();
    db.insert(new_id, user_data.into_inner());

    return HttpResponse::Created().json(User {
        name
    });
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;
    println!("server is running at port {port}");

    let user_db = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new().app_data(app_data)
        .service(get_user)
        .service(home)
        .service(create_user)
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await

}