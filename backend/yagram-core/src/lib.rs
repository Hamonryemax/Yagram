mod app_data;
mod app_settings;
mod domains;
mod errors;
mod telemetry;

use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_settings::ApplySettings as _;
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::Logger;
use actix_web::{get, web, web::scope, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use app_data::AppState;
use domains::{auth, messages, users};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use tracing_actix_web::TracingLogger;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
pub async fn start() -> Result<(), std::io::Error> {
    dotenv().ok();
    telemetry::init_telemetry("Yagram");
    let settings = app_settings::SettingsInitializer::init();

    let app_state = AppState::new(&settings.application)
        .await
        .expect("AppState create failed");

    Migrator::up(&app_state.db, None)
        .await
        .expect("Migration failed");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&settings.actix.tls.private_key, SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(&settings.actix.tls.certificate)
        .unwrap();

    let state = web::Data::new(app_state);
    let settings_to_move = settings.clone();

    HttpServer::new(move || {
        let settings = settings_to_move.clone();
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Logger::new("%a %s %{User-Agent}i"))
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(settings.application.redis_connection.clone()),
                    Key::generate(),
                )
                .cookie_http_only(false)
                .cookie_same_site(SameSite::Strict)
                .build(),
            )
            .app_data(state.clone())
            .service(ping)
            .service(
                scope("/auth")
                    .service(auth::handlers::login)
                    .service(auth::handlers::authorize),
            )
            .service(
                scope("/api")
                    .wrap(HttpAuthentication::bearer(auth::validator))
                    .service(users::handlers::get_user)
                    .service(messages::handlers::connect),
            )
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .apply_settings(&settings)
    .run()
    .await?;

    // Ensure all spans have been shipped to Jaeger.
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
