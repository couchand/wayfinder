use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::core::RouteConfig;
use crate::gen::codegen;
use crate::parse;
use crate::parse::errors::show_errors;

macro_rules! fail {
    () => {
        std::process::exit(101)
    };
    ($($args:tt)+) => {
        {
            eprintln!($($args)+);
            std::process::exit(101)
        }
    };
}

enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: Write, R: Write> Write for Either<L, R> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Either::Left(l) => l.write(buf),
            Either::Right(r) => r.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Either::Left(l) => l.flush(),
            Either::Right(r) => r.flush(),
        }
    }
}

enum Source {
    File(PathBuf),
    Config(RouteConfig),
}

enum Target {
    File(PathBuf),
    Stdout,
}

pub struct Builder {
    use_cargo: bool,
    source_dir: Option<PathBuf>,
    source: Option<Source>,
    target_dir: Option<PathBuf>,
    target: Option<Target>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            use_cargo: false,
            source_dir: None,
            source: None,
            target_dir: None,
            target: None,
        }
    }

    pub fn from_env() -> Builder {
        let source_dir = match env::var("CARGO_MANIFEST_DIR") {
            Err(env::VarError::NotPresent) => fail!("Env var CARGO_MANIFEST_DIR not found."),
            Err(env::VarError::NotUnicode(_)) => fail!("Env var CARGO_MANIFEST_DIR not valid Unicode."),
            Ok(d) => Some(d.into()),
        };
        let target_dir = match env::var("OUT_DIR") {
            Err(env::VarError::NotPresent) => fail!("Env var OUT_DIR not found."),
            Err(env::VarError::NotUnicode(_)) => fail!("Env var OUT_DIR not valid Unicode."),
            Ok(d) => Some(d.into()),
        };

        Builder {
            use_cargo: true,
            source_dir,
            source: None,
            target_dir,
            target: None,
        }
    }

    pub fn input_file<P: Into<PathBuf>>(mut self, path: P) -> Builder {
        self.source = Some(Source::File(path.into()));
        self
    }

    pub fn input_config(mut self, routes: RouteConfig) -> Builder {
        self.source = Some(Source::Config(routes));
        self
    }

    pub fn output_file<P: Into<PathBuf>>(mut self, path: P) -> Builder {
        self.target = Some(Target::File(path.into()));
        self
    }

    pub fn output_stdout(mut self) -> Builder {
        self.target = Some(Target::Stdout);
        self
    }

    pub fn build(self) {
        let source = match self.source {
            None => fail!("Source not configured.  Try builder.input_file(\"app.routes\")."),
            Some(s) => s,
        };
        let target = match self.target {
            None => fail!("Target not configured.  Try builder.output_file(\"routes.rs\")."),
            Some(t) => t,
        };

        let input = match source {
            Source::Config(c) => c,
            Source::File(filename) => {
                let input_file = match self.source_dir {
                    None => filename,
                    Some(dir) => dir.join(filename),
                };
                if self.use_cargo {
                    println!("cargo:rerun-if-changed={}", input_file.display());
                }

                let contents = match std::fs::read_to_string(input_file.clone()) {
                    Ok(c) => c,
                    Err(e) => fail!("Unable to load file {}: {}", input_file.display(), e),
                };

                match parse::route_config(&contents) {
                    Ok(config) => config.1,
                    result => {
                        eprintln!("Unable to parse route config file {}:", input_file.display());
                        show_errors(&mut std::io::stderr(), &contents, result, "");
                        fail!()
                    }
                }
            }
        };

        let mut output = match target {
            Target::Stdout => Either::Left(std::io::stdout()),
            Target::File(filename) => {
                let output_file = match self.target_dir {
                    None => filename,
                    Some(dir) => dir.join(filename),
                };

                match File::create(&output_file) {
                    Ok(f) => Either::Right(f),
                    Err(e) => fail!("Unable to create output file {}: {}", output_file.display(), e),
                }
            }
        };

        match codegen(&mut output, &input) {
            Ok(_) => {}
            Err(e) => {
                fail!("Error generating code: {}", e);
            }
        }
    }
}
