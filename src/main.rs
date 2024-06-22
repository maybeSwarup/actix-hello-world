use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    "Hello world!"
}

#[get("/show")]
async fn show_users() -> impl Responder {
    let users = vec!["Gangadhar", "Shaktimaan"];
    let body = serde_json::to_string(&users).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(web::scope("/app").route("index.html", web::get().to(index)))
            .service(
                web::scope("/users").service(show_users)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
