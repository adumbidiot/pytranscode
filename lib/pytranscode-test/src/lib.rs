use std::{
    path::PathBuf,
    process::{
        Command,
        Output,
    },
};

include!(concat!(env!("OUT_DIR"), "/output.rs"));

#[derive(Debug)]
pub struct TestData {
    pub cmd_output: Result<Output, std::io::Error>,
    pub passed: bool,
}

pub fn run_test(test_name: &str) -> Option<TestData> {
    if let Some(test_n) = TEST_NAMES.iter().position(|el| el == &test_name) {
        let exe = PathBuf::from(OUT_DIR).join(&format!("{}.exe", test_name));
        dbg!(&exe);

        let cmd_output = Command::new(exe).output();
        let mut passed = false;

        if let Ok(output) = cmd_output.as_ref() {
            if output.stdout == TEST_OUTPUTS[test_n].as_bytes() {
                passed = true;
            }
        }

        return Some(TestData { cmd_output, passed });
    }

    None
}
