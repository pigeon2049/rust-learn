use crate::controller::router;
use crate::docker::run_compose;

mod controller;
mod docker;
mod db;
mod entity;
 fn main() {
    let _runtime_docker = run_compose::main();
    let _hello_controller = router::main();

}