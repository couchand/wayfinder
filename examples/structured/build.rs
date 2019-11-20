extern crate wayfinder_build;

use wayfinder_build::build_from_config;

mod routes;

fn main() {
    build_from_config(routes::routes())
}
