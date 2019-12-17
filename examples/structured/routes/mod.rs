use wayfinder::RouteConfig;

mod base;
mod books;
mod people;

pub fn routes() -> RouteConfig {
    base::routes()
        .mount("people", people::routes())
        .mount("books", books::routes())
}
