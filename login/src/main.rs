use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //dotenv().ok();
    let bind_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    println!("Starting login server on http://{}/", bind_addr);

    HttpServer::new(move || App::new().service(get_account).service(post_account))
        .bind(bind_addr)?
        .run()
        .await
}

#[get("/{username}/{password}")]
async fn get_account(data: web::Path<(String, String)>) -> impl Responder {
    let (username, password) = data.into_inner();

    println!("Username: {}", username);
    println!("Password: {}", password);
    HttpResponse::Ok().body("GET works!")
}

// TODO: Maybe use the id of the account here instead
#[post("/{username}")]
async fn post_account(data: web::Path<String>) -> impl Responder {
    println!("Username: {}", data);
    HttpResponse::Ok().body("POST works!")
}
