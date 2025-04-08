use crate::{
    api::{self},
    model::{player::Player, positions},
};
use actix_files::Files;
use actix_web::{post, web, App, HttpServer};
use tokio::time::Duration;

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    tokio::spawn(async {
        println!("Starting Tcp Connection on 192.168.2.40:30002");
        let mut _tcp = api::tcp::RobotArm::start_tcp("192.168.2.40:30002")
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
        let _ = api::tcp::RobotArm::send_script(&mut _tcp).await;
    });

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
