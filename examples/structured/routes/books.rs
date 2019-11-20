use wayfinder_core::{RouteConfig, Routes, NestedRoutes, get, post, put, delete, header, param};

pub fn routes() -> RouteConfig {
    RouteConfig {
        headers: vec![header!(use uuid::Uuid;)],
        routes: Routes {
            resources: vec![
                get!{Books::Index},
                post!{Books::Create},
            ],
            routes: vec![
                NestedRoutes::new(
                    "new",
                    Routes {
                        resources: vec![
                            get!{Books::New},
                        ],
                        ..Default::default()
                    },
                ),
                NestedRoutes::new(
                    param!(id: Uuid),
                    Routes {
                        resources: vec![
                            get!{Books::Show},
                            put!{Books::Update},
                            delete!{Books::Destroy},
                        ],
                        routes: vec![
                            NestedRoutes::new(
                                "edit",
                                Routes {
                                    resources: vec![
                                        get!{Books::Edit},
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
