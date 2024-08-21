use assert_cli::Assert;

#[test]
fn test_main_functionality() {
	let args = [
        "seaorm-markdown",
        "--project-root",
        "tests/test-project",
    ];

    Assert::main_binary()
        .with_args(&args)
        .succeeds()
        .and()
        .stdout()
        .is(EXPECTED)
        .unwrap();
}

const EXPECTED: &str = r#"
# test-project
"#;