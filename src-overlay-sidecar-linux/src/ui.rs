use std::{fs, path::PathBuf};

use axum::{Router, routing::get_service};
use hyper::StatusCode;
use log::error;
use tower_http::services::ServeDir;

use crate::ARGS;

pub async fn serve_ui() -> u16 {
    let mut path = PathBuf::from("ui");
    if ARGS.get().as_ref().unwrap().core_pid==0{
        log::trace!("debug ui");
        path=PathBuf::from("../src-overlay-ui/build/");
    }
    let path=fs::canonicalize(path).unwrap();
    log::trace!("using ui path:{:?}", path);
    assert!(path.is_dir());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let serve_dir = get_service(ServeDir::new(path)).handle_error(|err| async move {
        error!("Server error: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    });

    let port = listener.local_addr().unwrap().port();
    log::debug!("staring ui on: {:?}", listener.local_addr().unwrap());
    let app: Router<()> = Router::new().fallback_service(serve_dir);
    tokio::task::spawn(async { axum::serve(listener, app).await.unwrap() });
    port
}
