use tokio::net::{TcpListener};
use hyper::{
    service::{service_fn},
    Body, Request, Response,
    server::conn::Http,
    rt::Executor
};
use std::convert::Infallible;


#[tokio::main]
async fn main() {

    // bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    print!("Listening!");

    // loop through to handle connections asynchronously
    loop {
        // the second item contains the IP and the port of the new connection
        let (socket, _) = listener.accept().await.unwrap();

        print!("Accepted!");

        // new task is spawned for each incoming socket connection
        // the socket is moved to the new task and processed there
        tokio::spawn(async move {

            // instead of our custom fn, we're going to use hyper's function

            let service = service_fn(handle);

            // need to split the steps because http2_only returns a &mut self
            // whereas with_executor returns a Http<E2>
            let mut http = Http::new();

            http.http2_only(true);

            let http = http.with_executor(TokioExecutor);

            if let Err(err) = http.serve_connection(socket, service).await {
                eprintln!("connection error: {}", err);
            }
        });
    }

    
}

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("hello from hyper http/2 (no TLS)")))
}


#[derive(Clone)]
struct TokioExecutor;

// Implement the `hyper::rt::Executor` trait for `TokioExecutor` so that it can be used to spawn
// tasks in the hyper runtime.
// An Executor allows us to manage execution of tasks which can help us improve the efficiency and
// scalability of the server.
impl<F> Executor<F> for TokioExecutor
where
    F:std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) {
        tokio::spawn(fut);
    }
}
