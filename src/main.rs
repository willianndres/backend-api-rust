mod repository;
mod user;
use actix_web::{web, App, HttpResponse, HttpServer};
use repository::{MemoryRepository, RepositoryInjector};
use std::sync::{atomic::AtomicU16, Arc};
use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //init env vars
    dotenv::dotenv().ok();
    //building address
    let _port = std::env::var("PORT").unwrap_or("8080".to_string());
    let _address = format!("127.0.0.1:{}", _port);
    //building shared state
    println!("Starting server");
    let _thread_counter = Arc::new(AtomicU16::new(1));
    let _repo = web::Data::new(RepositoryInjector::new(MemoryRepository::default()));
    //starting the server
    HttpServer::new(move || {
        let _thread_index = _thread_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("Starting thread {}", _thread_index);
        //starting the services 
        App::new()
            .data(_thread_index)
            .app_data(_repo.clone())
            .route("/", web::get().to(|| HttpResponse::Ok().body("Hola rust")))
            .service(web::resource("/user/{user_id}").route(web::get().to(get_user)))
            .route(
                "/health",
                web::get().to(|index: web::Data<u16>| {
                    HttpResponse::Ok()
                        .header("thread-id", index.to_string())
                        .finish()
                }),
            )
    })
    .bind(&_address)
    .unwrap_or_else(|err| {
        panic!(
            "NO SE PUEDE INICIAR EL SERVIDOR CON EL PUERTO {}:{:?}",
            _port, err
        )
    })
    .run()
    .await
}

async fn get_user(
    user_id: web::Path<String>,
    repo: web::Data<RepositoryInjector>,
) -> HttpResponse {
    if let Ok(parsed_user_id) = Uuid::parse_str(&user_id) {
        match repo.get_user(&parsed_user_id) {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().body("Not Found"),
        }
    } else {
        HttpResponse::BadRequest().body("Invalid UUID")
    }
}
