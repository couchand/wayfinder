//! A little HTTP route matcher generator.
//!
//! Other route matchers are configured at runtime, which means they
//! pay a hefty price of dynamic algorithms for something that is
//! nearly always static.  Wayfinder does things a little differently:
//! configure routes at build-time.
//!
//! You can specify the route structure in one of two ways.  The most
//! readable format is a route config file, which might look like:
//!
//! ```ignore
//! use uuid::Uuid;
//!
//! /
//!   GET Index
//!
//!   users
//!     GET Users::List
//!     POST Users::Create
//!
//!     {id: uuid}
//!       GET Users::Show
//! ```
//!
//! Then add a build script to generate the route matching code:
//!
//! ```ignore
//! Builder::from_env()
//!     .input_file("app.routes")
//!     .output_file("routes.rs")
//!     .build();
//! ```
//!
//! And import the generated route module into your app.
//!
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/routes.rs"));
//! ```
//!
//! When a request comes in, match it against the route table:
//!
//! ```ignore
//! use routes::{Match, Route};
//! match routes::match_route(path, method) {
//!     Ok(Match::Route(Route::Index(action))) => index_controller(action),
//!     Ok(Match::Route(Route::Users(action))) => users_controller(action),
//!     _ => error_handler(),
//! }
//! ```
//!
//! See the documentation for the generated module for more information,
//! or the examples for a complete application.
//!
//! You can also build up your route config using the structs and macros
//! provided here.  The equivalent code for the above routes might be:
//!
//! ```
//! # use wayfinder::{get, post, param, header, RouteConfig, Routes, NestedRoutes};
//! let config = RouteConfig {
//!     headers: vec![header!(
//!         use uuid::Uuid;
//!     )],
//!     routes: Routes {
//!         resources: vec![
//!             get!(Index)
//!         ],
//!         routes: vec![NestedRoutes::new(
//!             "users",
//!             Routes {
//!                 resources: vec![
//!                     get!(Users::List),
//!                     post!(Users::Create),
//!                 ],
//!                 routes: vec![NestedRoutes::new(
//!                     param!(id: Uuid),
//!                     Routes {
//!                         resources: vec![
//!                             get!(Users::Show)
//!                         ],
//!                         ..Routes::default()
//!                     }
//!                 )],
//!                 ..Routes::default()
//!             }
//!         )],
//!         ..Routes::default()
//!     },
//! };
//! ```
//!
//! Sure, it's a little more verbose, but you get the benefit of Rust's
//! superb error handling, and it's much less magic.  Update your build
//! script to use the route config:
//!
//! ```ignore
//! Builder::from_env()
//!     .input_config(config)
//!     .output_file("routes.rs")
//!     .build();
//! ```

pub mod build;
mod core;
pub mod gen;
pub mod parse;

// TODO: crate-level docs
// TODO: better re-exports
pub use crate::core::*;
