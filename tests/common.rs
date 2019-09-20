use pytranscode_test;

#[test]
fn common() {
    for test in pytranscode_test::TEST_NAMES {
        let data = pytranscode_test::run_test(test).unwrap();
        dbg!(data);
    }
}
