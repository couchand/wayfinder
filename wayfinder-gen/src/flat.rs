use itertools::Either;

use crate::trie::*;
use wayfinder_core::*;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Charlike {
    Static(char),
    Dynamic(String), // the param name
    Separator,
}

#[derive(Debug, Clone)]
pub struct FlattenedPath {
    segments: Vec<PathSegment>,
}

impl FlattenedPath {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Charlike> + 'a {
        self.segments
            .iter()
            .map(|segment| match segment {
                PathSegment::Static(s) => Either::Left(
                    s.chars()
                        .map(Charlike::Static)
                        .chain(std::iter::once(Charlike::Separator)),
                ),
                PathSegment::Dynamic(d) => Either::Right(
                    vec![Charlike::Dynamic(d.name.clone()), Charlike::Separator].into_iter(),
                ),
            })
            .flatten()
    }

    pub fn dynamics<'a>(&'a self) -> impl Iterator<Item = &'a Param> + 'a {
        self.segments.iter().filter_map(|segment| match segment {
            PathSegment::Dynamic(s) => Some(s),
            _ => None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FlattenedRoute {
    pub path: FlattenedPath,
    pub resources: Vec<Resource>,
    pub query_parameters: Vec<Param>,
}

pub struct FlattenedRoutes {
    routes: Vec<FlattenedRoute>,
}

impl<'a> From<&'a Routes> for FlattenedRoutes {
    fn from(routes: &Routes) -> FlattenedRoutes {
        let routes = FlattenedRoutes::flatten(routes, vec![], vec![]);
        FlattenedRoutes { routes }
    }
}

impl FlattenedRoutes {
    fn flatten(
        routes: &Routes,
        path: Vec<PathSegment>,
        mut query_parameters: Vec<Param>,
    ) -> Vec<FlattenedRoute> {
        let mut flattened = vec![];

        for param in routes.query_parameters.iter() {
            query_parameters.push(param.clone());
        }

        flattened.push(FlattenedRoute {
            path: FlattenedPath {
                segments: path.clone(),
            },
            resources: routes.resources.clone(),
            query_parameters: query_parameters.clone(),
        });

        for child in routes.routes.iter() {
            let mut new_path = path.clone();
            new_path.push(child.path_segment.clone());
            flattened.extend_from_slice(&FlattenedRoutes::flatten(
                &child.routes,
                new_path,
                query_parameters.clone(),
            ));
        }

        flattened
    }

    /*
        pub fn iter<'a>(&'a self) -> impl Iterator<Item=&FlattenedRoute> + 'a {
            self.routes.iter()
        }
    */

    pub fn to_trie(&self) -> Trie<Charlike, FlattenedRoute> {
        let mut t = Trie::new();

        for route in self.routes.iter() {
            t = t
                .add(route.path.iter(), route.clone())
                .map_err(|_| "all paths should be unique!")
                .unwrap();
        }

        t
    }
}

#[derive(Debug)]
pub struct FlattenedModules {
    pub modules: Vec<FlattenedModule>,
}

impl<'a> From<&'a Routes> for FlattenedModules {
    fn from(routes: &Routes) -> FlattenedModules {
        let modules = FlattenedModules::flatten(routes, vec![], vec![]);
        FlattenedModules { modules }
    }
}

impl FlattenedModules {
    fn flatten(
        routes: &Routes,
        path: Vec<PathSegment>,
        query_parameters: Vec<Param>,
    ) -> Vec<FlattenedModule> {
        use std::collections::HashMap;

        let mut actions = HashMap::new();

        let mut routes_to_process = vec![(routes, path, query_parameters)];

        loop {
            let (routes, path, mut query_parameters) = match routes_to_process.pop() {
                None => break,
                Some((r, p, qp)) => (r, p, qp),
            };

            for param in routes.query_parameters.iter() {
                query_parameters.push(param.clone());
            }

            let flat_path = FlattenedPath {
                segments: path.clone(),
            };
            for resource in routes.resources.iter() {
                if resource.is_redirect {
                    continue;
                }

                let mut query_parameters = query_parameters.clone();
                query_parameters.extend_from_slice(&resource.query_parameters);

                match actions
                    .entry(resource.modules.clone().into_iter().next().unwrap()) // TODO: multiplicity
                    .or_insert(HashMap::new())
                    .insert(
                        resource.name.clone(),
                        FlattenedAction {
                            name: resource.name.clone(),
                            method: resource.method.clone(),
                            path: flat_path.clone(),
                            route_parameters: flat_path.dynamics().cloned().collect(),
                            query_parameters,
                        },
                    ) {
                    Some(_) => panic!(
                        "Duplicate controller action `{}::{}`!",
                        resource.modules.iter().next().unwrap(), resource.name,
                    ),
                    None => {}
                }
            }

            for child in routes.routes.iter() {
                let mut new_path = path.clone();
                new_path.push(child.path_segment.clone());

                routes_to_process.push((&child.routes, new_path, query_parameters.clone()));
            }
        }

        let mut modules = vec![];

        for (name, mut actions) in actions.drain() {
            let mut module = FlattenedModule {
                name,
                actions: actions.drain().map(|(_, v)| v).collect(),
            };

            module.actions.sort_unstable_by_key(|a| a.name.clone());

            modules.push(module);
        }

        modules.sort_unstable_by_key(|c| c.name.clone());

        modules
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &FlattenedModule> + 'a {
        self.modules.iter()
    }
}

#[derive(Debug)]
pub struct FlattenedModule {
    pub name: String,
    pub actions: Vec<FlattenedAction>,
    // pub modules: Vec<FlattenedModule>,
}

#[derive(Debug)]
pub struct FlattenedAction {
    pub name: String,
    pub method: Method,
    pub path: FlattenedPath,
    pub route_parameters: Vec<Param>,
    pub query_parameters: Vec<Param>,
}
