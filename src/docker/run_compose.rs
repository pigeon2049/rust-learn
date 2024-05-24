use compose_rs::{Compose, ComposeCommand};
const DOCKER_COMPOSE_CONTENT: &str = include_str!("../../docker-compose.yaml");
use std::fs::File;
use std::io::Write;

#[tokio::main]
pub(crate) async fn main() {

    // Write the Docker Compose content to a temporary file
    let temp_file_path = "temp-docker-compose.yaml";
    let mut temp_file = File::create(temp_file_path)
        .expect("Failed to create temporary file");
    temp_file
        .write_all(DOCKER_COMPOSE_CONTENT.as_bytes())
        .expect("Failed to write to temporary file");


    let compose = Compose::builder()
        .path(temp_file_path)
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

    // Remove the temporary file
    if let Err(e) = std::fs::remove_file(temp_file_path) {
        eprintln!("Failed to remove temporary file: {}", e);
    }
}