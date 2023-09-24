use actix_web::{App, HttpServer, web::Data};

mod persistence;

pub async fn serve() -> std::io::Result<()> {
    println!("Starting Dreka server");

    let db = persistence::db::Db::new("memory").await.expect("Error establishing a database connection");
    let db_data = Data::new(db);

    HttpServer::new(move || {
    App::new()
        .app_data(db_data.clone())
    })
    .bind(("127.0.0.1", 45486))?
    .run()
    .await
}