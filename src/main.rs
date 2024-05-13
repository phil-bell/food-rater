use actix_web::{get, App, HttpServer, Result as AwResult};
use maud::{html, Markup};
use std::io;

#[get("/")]
async fn index() -> AwResult<Markup> {
    Ok(html! {
        html {
            body {
                h1 { "Hello there!" }
                h1 { "Genereal Kanobi!" }
            }
        }
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
