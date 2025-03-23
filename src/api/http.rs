use actix_files::Files;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use crate::model::player::{self, Player};

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(Files::new("/", "src/webcontent").index_file("index.html"))
            //.service(home)
            .service(roll)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn home() -> impl Responder {
    let html = include_str!("../webcontent/index.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[post("/roll")]
async fn roll(player: web::Json<Player>) -> String {
    println!("Test");
    format!("Welcome {}, id {}", player.name, player.id)
}
