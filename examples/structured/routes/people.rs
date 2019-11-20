use wayfinder_core::{RouteConfig, Routes, NestedRoutes, get, post, put, delete, header, param};

pub fn routes() -> RouteConfig {
    RouteConfig {
        headers: vec![header!(use uuid::Uuid;)],
        routes: Routes {
            resources: vec![
                get!{People::Index},
                post!{People::Create},
            ],
            routes: vec![
                NestedRoutes::new(
                    "new",
                    Routes {
                        resources: vec![
                            get!{People::New},
                        ],
                        ..Default::default()
                    },
                ),
                NestedRoutes::new(
                    param!(id: Uuid),
                    Routes {
                        resources: vec![
                            get!{People::Show},
                            put!{People::Update, param!{name: String}},
                            delete!{People::Destroy},
                        ],
                        routes: vec![
                            NestedRoutes::new(
                                "edit",
                                Routes {
                                    resources: vec![
                                        get!{People::Edit},
                                    ],
                                    ..Default::default()
                                },
                            ),
                        ],
                        query_parameters: vec![],
                    },
                ),
            ],
            query_parameters: vec![],
        },
    }
}
