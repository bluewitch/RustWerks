// Import the required Actix web modules
use actix_web::{web, App, HttpServer};

// This function, `index`, is an endpoint handler. It takes a tuple wrapped in a web::Path object as an argument. 
// This tuple represents the path parameters of the HTTP request.
fn index(info: web::Path<(String,)>) -> String {
    // This endpoint handler returns a string "Hello {name}!" where {name} is the first path parameter.
    // It uses the `format!` macro to format the string.
    format!("Hello {}!", info.0)
}

// This attribute macro tells the Rust compiler to use the Actix runtime as the main runtime.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // The HttpServer::new function is used to create a new HTTP server.
    HttpServer::new(|| {
        // The App::new function is used to create a new application instance.
        // The application instance represents an individual HTTP application with its own middleware, route, and resource configurations.
        App::new()
            // The `route` function is used to set up a route that matches a specific path.
            // In this case, it matches any path that looks like "/{name}" where {name} can be any string.
            // The `web::get().to(index)` part specifies that this route should respond to HTTP GET requests by calling the `index` function.
            .route("/{name}", web::get().to(index))
    })
    // The `bind` function binds the server to a specific address and port.
    .bind("127.0.0.1:8080")?
    // The `run` function starts the server and blocks the current thread until the server is stopped or crashes.
    .run()
    // The `await` keyword is used to wait for the server to finish running before continuing execution.
    // This is necessary because the `run` function is asynchronous.
    .await
}
