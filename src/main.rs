mod codegen;
use codegen::{
    CodeGenerator,
    PythonStatement,
};
use quote::quote;
use rustfmt_nightly::{
    Config,
    EmitMode,
    Input,
    Session,
};
use rustpython_parser::parser::parse_program;

const TEST1: &str = r#"print("This line will be printed.")"#;
const TEST2: &str = r#"x = 1
if x == 1:
    # indented four spaces
    print("x is 1.")"#;
const TEST: &str = include_str!("test3.py");
fn main() {
    let ast = parse_program(TEST).unwrap();
    //dbg!(&ast);
    let mut codegen = CodeGenerator::new();
    codegen.add_many(
        ast.statements
            .iter()
            .map(|el| PythonStatement::from(&el.node)),
    );
    let code_out = codegen.export_tokens();

    let raw = quote! {
        mod pystd;
        use pystd::*;
        fn main(){
            #code_out
        }
    };
    let raw = raw.to_string();

    let mut pretty_buf = Vec::with_capacity(raw.len());

    {
        let mut config = Config::default();
        config.set().emit_mode(EmitMode::Stdout);

        let mut sess = Session::new(config, Some(&mut pretty_buf));
        sess.format(Input::Text(raw)).unwrap();
    }

    let output = std::str::from_utf8(&pretty_buf)
        .unwrap()
        .trim_start_matches("stdin:")
        .trim();
    println!("{}", output);
}
