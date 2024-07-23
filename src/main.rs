use actix::prelude::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mime_guess::from_path;
use redis_service::{GetRedisValue, RedisActor};
use std::env;

mod redis_service;

fn get_mime_type(path: &str) -> &'static str {
    match from_path(path).first_raw() {
        Some(mime) => mime,
        None => "text/html",
    }
}

fn get_cache_duration(mime: &str) -> usize {
    match mime {
        "text/html" => env::var("CACHE_DURATION_TEXT_HTML")
            .unwrap_or_else(|_| "3600".to_string())
            .parse()
            .unwrap_or(3600),
        "image/jpeg" => env::var("CACHE_DURATION_IMAGE_JPEG")
            .unwrap_or_else(|_| "172800".to_string())
            .parse()
            .unwrap_or(172800),
        _ => 3600,
    }
}

#[get("/{path:.*}")]
async fn index(path: web::Path<String>, redis_addr: web::Data<Addr<RedisActor>>) -> impl Responder {
    let path = path.into_inner();
    let prefix = env::var("REDIS_PREFIX").expect("REDIS_PREFIX must be set");
    let key = if path.is_empty() {
        format!("{}index.html", prefix)
    } else if path == "fr_CH" {
        format!("{}index.html", prefix)
    } else if path.ends_with('/') {
        format!("{}{}index.html", prefix, path)
    } else {
        format!("{}{}", prefix, path)
    };

    match redis_addr.send(GetRedisValue { key }).await {
        Ok(Ok(data)) => {
            let mime_type = get_mime_type(&path);
            let cache_duration = get_cache_duration(mime_type);
            HttpResponse::Ok()
                .content_type(mime_type)
                .insert_header(("Cache-Control", format!("max-age={}", cache_duration)))
                .body(data)
        }
        _ => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_addr = RedisActor::new(&redis_url).start();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a valid u16");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_addr.clone()))
            .service(index)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
