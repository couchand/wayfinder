use wayfinder::build::Builder;

fn main() {
    Builder::from_env()
        .input_file("../../example.routes")
        .output_file("routes.rs")
        .build();
}
