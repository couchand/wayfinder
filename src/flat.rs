use crate::config::*;
use crate::trie::*;

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
    pub fn iter<'a>(&'a self) -> impl Iterator<Item=Charlike> + 'a {
        self.segments.iter()
            .map(|segment| {
                let res: Box<Iterator<Item=Charlike>> = match segment {
                    PathSegment::Static(s) => {
                        Box::new(
                            s.chars()
                                .map(Charlike::Static)
                                .chain(std::iter::once(Charlike::Separator))
                        )
                    },
                    PathSegment::Dynamic(d) => {
                        Box::new(vec![
                            Charlike::Dynamic(d.name.clone()),
                            Charlike::Separator,
                        ].into_iter())
                    },
                };
                res
            })
            .flatten()
    }

    pub fn dynamics<'a>(&'a self) -> impl Iterator<Item=&'a Param> + 'a {
        self.segments.iter()
            .filter_map(|segment| match segment {
                PathSegment::Dynamic(s) => Some(s),
                _ => None,
            })
    }
}

#[derive(Debug,Clone)]
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
            path: FlattenedPath { segments: path.clone() },
            resources: routes.resources.clone(),
            query_parameters: query_parameters.clone(),
        });

        for child in routes.routes.iter() {
            let mut new_path = path.clone();
            new_path.push(child.path_segment.clone());
            flattened.extend_from_slice(
                &FlattenedRoutes::flatten(
                    &child.routes,
                    new_path,
                    query_parameters.clone(),
                )
            );
        }

        flattened
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&FlattenedRoute> + 'a {
        self.routes.iter()
    }

    pub fn to_trie(&self) -> Trie<Charlike, FlattenedRoute> {
        let mut t = Trie::new();

        for route in self.routes.iter() {
            t = t.add(route.path.iter(), route.clone())
                .map_err(|_| "all paths should be unique!")
                .unwrap();
        }

        t
    }
}
