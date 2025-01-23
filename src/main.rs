mod static_files;
mod store_manipulation;
mod templates;
use serde::Deserialize;

use crate::templates::render_template;

use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let app = Router::new()
        .route("/static/{*file}", get(static_files::handler))
        .route(
            "/",
            get(|Query(params): Query<Params>| async move {
                let mut ctx = tera::Context::new();

                let nixstore_all = store_manipulation::read_nix_store_handled().await;

                let current_page = params.page.unwrap_or(0);
                let nixstore = nixstore_all.get(&current_page).unwrap();

                ctx.insert("paths", &nixstore);
                ctx.insert("current_page", &current_page);
                render_template("home", &ctx)
            }),
        )
        .route(
            "/store/{path_str}",
            get(|Path(path_str): Path<String>| async move {
                let mut ctx = tera::Context::new();

                let path = std::path::PathBuf::from(format!("/nix/store/{path_str}"));
                let item = store_manipulation::parse_store_item(&path).await;

                if let Ok(item) = item {
                    ctx.insert("item", &item);
                } else {
                    ctx.insert("error", "Error parsing store item");
                }

                render_template("store", &ctx)
            }),
        );

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct Params {
    page: Option<usize>,
}
