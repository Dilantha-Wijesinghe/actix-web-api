use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web};

use dao::Dao;

pub mod dao;

#[get("/")]
async fn hello(data: web::Data<Dao>) -> impl Responder {
    HttpResponse::Ok().json(data.read().unwrap())
}

#[post("/new")]
async fn create(data: web::Data<Dao>, req_body: String) -> impl Responder {
    if let Ok(_) = data.create(&req_body) {
        HttpResponse::Ok().body(req_body)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[delete("/del/{id}")]
async fn remove(data: web::Data<Dao>, id: web::Path<(u32,)>) -> impl Responder {
    let todo_id = id.into_inner();
    if let Ok(_) = data.remove(todo_id.0.into()) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Dao::new().unwrap()))
            .service(hello)
            .service(create)
            .service(remove)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
