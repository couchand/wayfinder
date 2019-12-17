use wayfinder::build::build_from_config;

mod routes;

fn main() {
    build_from_config(routes::routes())
}
