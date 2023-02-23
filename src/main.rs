use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct IndexTemplate {
    title: String,
    body: String,
}

async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = IndexTemplate {
        title: "Handlebars Example".to_string(),
        body: "This is the body of the index page.".to_string(),
    };
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("index", "src/templates/index.hbs")
        .unwrap();


    HttpServer::new(move || {
        App::new()
            .app_data(handlebars.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
