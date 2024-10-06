use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::thread;


#[derive(Deserialize, Serialize)]
struct Command {
    name: String,
    data: String,
}

async fn command_listener(command: web::Json<Command>) -> impl Responder {
    println!("Received command: {}", command.name);
    HttpResponse::Ok().json(format!("Command {} received with data: {}", command.name, command.data))
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create and start the HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(self.hello()))  // Define the route and bind it to the handler
    })
    .bind("127.0.0.1:8080")?  // Bind the server to localhost:8080
    .run()  // Run the server
    .await
}