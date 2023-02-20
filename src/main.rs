use axum::{routing::get, Router};
use dotenv::dotenv;
use maud::{DOCTYPE, Markup, html};
use std::env;

mod html;
mod tarif;
mod form;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT non défini");
    
    let listening_adress = std::net::SocketAddr::from(([127,0,0,1], port));
    println!("Listening on {listening_adress}");
    
    let app = Router::new()
        .route("/", 
            get(tarif::form::loader)
            .post(tarif::search::loader)
        );
        
    axum::Server::bind(&listening_adress)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn header() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="https://unpkg.com/@picocss/pico@1.*/css/pico.min.css";
            title { "Tarif" };
        }
    }
}

pub fn page(content: Markup) -> Markup {
    html! {
        (header())
        body {
            main class="container" {
                (content)
            }
        }
    }
}