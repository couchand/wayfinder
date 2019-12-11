use std::env;
use std::fs::{DirEntry, File};
use std::path::PathBuf;
use itertools::process_results;

fn main() {
    let root_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cases_path = root_path.join("cases");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("cases.rs");
    let mut out_file = File::create(&out_path).expect("create file");

    let cases: Vec<_> = process_results(
        std::fs::read_dir(cases_path).expect("cases path"),
        |entries| entries.map(|entry| entry).collect()
    )
        .expect("case entry");

    write_cases(&mut out_file, &cases).expect("write cases");
}

fn write_cases<W: std::io::Write>(w: &mut W, cases: &[DirEntry]) -> std::io::Result<()> {
    writeln!(w, "#[cfg(test)]")?;
    writeln!(w, "mod test_cases {{")?;
    writeln!(w, "    use super::diff;")?;

    for case in cases {
        let case_name = case.file_name().into_string().expect("file name");

        writeln!(w, "    #[test]")?;
        writeln!(w, "    fn test_{}() {{", case_name)?;

        writeln!(w, "        let routes = include_str!(\"{}\");", case.path().join("routes.routes").to_str().expect("case route path"))?;

        writeln!(w, "        let rs = include_str!(\"{}\");", case.path().join("routes.rs").to_str().expect("case_path"))?;

        writeln!(w, "        let mut dest = vec![];")?;

        writeln!(w, "        match wayfinder_parse::route_config(&routes) {{")?;

        writeln!(w, "            Ok(config) => wayfinder_gen::codegen(&mut dest, &config.1).unwrap(),")?;
        writeln!(w, "            result => {{")?;
        writeln!(w, "                wayfinder_parse::errors::show_errors(&mut std::io::stderr(), &routes, result, \"\");")?;
        writeln!(w, "                assert!(false);")?;

        writeln!(w, "            }}")?;
        writeln!(w, "        }}")?;

        writeln!(w, "        let actual = String::from_utf8(dest).expect(\"result as utf8\");")?;
        writeln!(w, "        diff::TestResult::new(rs, &actual).assert();")?;

        writeln!(w, "    }}")?;
    }

    writeln!(w, "}}")?;

    Ok(())
}
