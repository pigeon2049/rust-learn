use compose_rs::{Compose, ComposeCommand};

#[tokio::main]
pub(crate) async fn main() {
    let compose = Compose::builder()
        .path("docker-compose.yaml")
        .build()
        .unwrap();

    // Execute the `up` command to start services defined in the Docker Compose file
    if let Err(e) = compose.up().exec() {
        eprintln!("Error starting services: {}", e);
    }

    // Stream stats and print them in real-time for each service
    compose
        .ps()
        .exec()
        .unwrap()
        .into_iter()
        .for_each(|service| {
            println!("{:?}", service);
        });

}