use wayfinder::build::Builder;

mod routes;

fn main() {
    Builder::from_env()
        .input_config(routes::routes())
        .output_file("routes.rs")
        .build();
}
