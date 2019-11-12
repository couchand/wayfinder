extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;

#[macro_use]
pub mod errors;

mod config;
pub use config::*;

pub mod parse;
pub mod flat;
pub mod trie;
pub mod gen;

#[cfg(test)]
mod tests;

pub enum Match<T> {
    NotFound,
    NotAllowed,
    Route(T),
    Redirect(T),
}

use std::fmt;
impl<T: fmt::Debug> fmt::Debug for Match<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Match::NotFound => write!(f, "Match::NotFound"),
            Match::NotAllowed => write!(f, "Match::NotAllowed"),
            Match::Route(t) => write!(f, "Match::Route({:?})", t),
            Match::Redirect(t) => write!(f, "Match::Redirect({:?})", t),
        }
    }
}

pub struct Error {
    param: String,
    what: Box<dyn fmt::Debug>,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("wayfinder::Error")
            .field("param", &self.param)
            .field("what", &self.what)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing '{}' parameter {:?}", self.param, self.what)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn fail<
        S: AsRef<str>,
        T: fmt::Debug + 'static,
    >(param: S, what: T) -> Error {
        Error {
            param: param.as_ref().to_string(),
            what: Box::new(what),
        }
    }
}
