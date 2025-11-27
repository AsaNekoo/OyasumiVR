use std::fs;

use axum::{Router, routing::get_service};
use hyper::StatusCode;
use tower_http::services::ServeDir;

pub async fn serve_ui() -> u16 {
    let path = fs::canonicalize("ui").unwrap();
    log::trace!("using ui path:{:?}",path);
    assert!(path.is_dir());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let serve_dir = get_service(ServeDir::new(path)).handle_error(|err| async move {
        eprintln!("Server error: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    });

    let port = listener.local_addr().unwrap().port();
    log::debug!("staring ui on: {:?}", listener.local_addr().unwrap());
    let app: Router<()> = Router::new().fallback_service(serve_dir);
    tokio::task::spawn(async{axum::serve(listener, app).await.unwrap()});
    port
}
