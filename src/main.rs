use crate::controller::router;
use crate::docker::run_compose;
use dotenv::dotenv;
mod controller;
mod docker;
mod db;
mod entity;
 fn main() {

    dotenv().ok();
    let _runtime_docker = run_compose::main();
    let _hello_controller = router::main();

}