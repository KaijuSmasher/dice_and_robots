use actix_files::Files;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use crate::{
    api::tcp,
    model::{player::Player, positions},
};

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    println!("Starting Webserver on Localhost(http://127.0.0.1:8080)");
    HttpServer::new(move || {
        App::new()
            .service(roll)
            .service(Files::new("/", "src/webcontent").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/roll")]
async fn roll(player: web::Json<Player>) -> String {
    let _position = positions::Position::create_position(player.dice_to_roll.dice_used);
    format!("Welcome {}", player.name)
}
