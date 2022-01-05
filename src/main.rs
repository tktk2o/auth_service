mod config;
mod handlers;

use actix_web::{App, HttpServer, middleware::Logger};
use color_eyre::Result;
use tracing::info;

use crate::{config::Config, handlers::app_config};

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server configuration");

    info!("Starting server at http://{}:{}/", config.host, config.port);

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .configure(app_config)
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;
    Ok(())
}
