use poem::{get, handler, listener::TcpListener, web::Path, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
pub(crate) async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/user/:id", get(crate::controller::user::show_user));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}