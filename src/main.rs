// CLOSED SOURCE PROJECT
// ALL RIGHTS ANTHONY FRETWELL 

// JRVS MAIN

// actix-web server
// https://actix.rs/docs/getting-started/

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// response handlers

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Server online")

}

async fn mwstest() -> impl Responder {
    HttpResponse::Ok().body("MWS ENDPOINT TEST PAGE")
}

// main function runs async requests to their route
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("mwstest", web::get().to(mwstest))
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await
}