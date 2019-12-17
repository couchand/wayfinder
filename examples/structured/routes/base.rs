use wayfinder::{get, param, NestedRoutes, RouteConfig, Routes};

pub fn routes() -> RouteConfig {
    RouteConfig {
        headers: vec![],
        routes: Routes {
            resources: vec![],
            routes: vec![NestedRoutes::new(
                "users",
                Routes {
                    resources: vec![get!(-> People::New)],
                    ..Default::default()
                },
            )],
            query_parameters: vec![param!(lang: String)],
        },
    }
}
