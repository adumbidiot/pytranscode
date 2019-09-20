use pytranscode::CodeGen;
use rustpython_parser::parser;
use std::{
    io::Write,
    path::Path,
    process::Command,
};

struct Test {
    name: String,
    py_src: String,
    output: String,
}

fn main() {
    //panic!("{}", var);
    let rustc = std::env::var("RUSTC").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("output.rs");
    let py_std_path = Path::new(&out_dir).join("pystd.rs");

    let mut output = String::new();
    let mut test_names = Vec::new();
    let mut test_outputs = Vec::new();
    let tests: Vec<_> = std::fs::read_dir("test-files")
        .unwrap()
        .map(|el| el.unwrap().path())
        .filter_map(|path| {
            if path.extension() == Some("py".as_ref()) {
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let py_src = std::fs::read_to_string(&path).unwrap();
                let output = std::fs::read_to_string(path.with_extension("txt")).unwrap();
                return Some(Test {
                    name,
                    py_src,
                    output,
                });
            }
            None
        })
        .collect();

    for test in tests.iter() {
        test_names.push(&test.name);
        test_outputs.push(&test.output);

        let ast = parser::parse_program(&test.py_src).unwrap();
        let mut code_gen = CodeGen::new();
        code_gen.write_str("mod pystd;\nuse pystd::*;\n\n");
        code_gen.write_str("pub fn main() {\n");
        code_gen.add_statements(&ast.statements);
        code_gen.write_str("\n}");

        let path = Path::new(&out_dir).join(&format!("{}.rs", test.name));
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(code_gen.output().as_bytes()).unwrap();
    }

    output += "pub const TEST_NAMES: &[&str] = &[";

    for name in test_names.iter() {
        output += "\"";
        output += name;
        output += "\", ";
    }

    output += "];\n";

    output += "pub const TEST_OUTPUTS: &[&str] = &[";

    for out in test_outputs.iter() {
        output += "\"";
        output += out;
        output += "\", ";
    }

    output += "];\n";
    output += "pub const OUT_DIR: &str = r#\"";
    output += &out_dir;
    output += "\"#;";

    let mut f = std::fs::File::create(&dest_path).unwrap();
    f.write_all(output.as_bytes()).unwrap();

    let mut f = std::fs::File::create(&py_std_path).unwrap();
    f.write_all(pytranscode::PY_STD_RS.as_bytes()).unwrap();

    for test in tests.iter() {
        let output_status = Command::new(&rustc)
            .current_dir(&out_dir)
            .args(&["--crate-type", "bin"])
            .arg(&format!("{}.rs", test.name))
            .output()
            .unwrap();
        let out = &output_status.stdout;
        //let s = String::from_utf8(out).unwrap();
    }
}
