use actix_cors::Cors;
use actix_web::{
    get, post,
    web,
    App, HttpResponse, HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(forward_get)
            .service(forward_post)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

#[get("/api/{endpoint:.*}")]
async fn forward_get(body: String, endpoint: web::Path<String>) -> HttpResponse {
    let client = reqwest::Client::new();

    let request = client
        .get(format!("https://new.myfreemp3juices.cc/api/{}", endpoint))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded",
        )
        .header("Cache-Control", "no-cache, no-store, must-revalidate")
        .header("Pragma", "no-cache")
        .header("Expires", "0")
        .body(body);

    let response = request.send().await.unwrap();

    HttpResponse::Ok().body(response.text().await.unwrap())
}

#[post("/api/{endpoint:.*}")]
async fn forward_post(body: String, endpoint: web::Path<String>) -> HttpResponse {
    let client = reqwest::Client::new();

    let request = client
        .post(format!("https://new.myfreemp3juices.cc/api/{}", endpoint))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded",
        )
        .header("Cache-Control", "no-cache, no-store, must-revalidate")
        .header("Pragma", "no-cache")
        .header("Expires", "0")
        .body(body);

    let response = request.send().await.unwrap();

    HttpResponse::Ok().body(response.text().await.unwrap())
}
