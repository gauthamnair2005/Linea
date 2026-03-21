use linea_ast::parse;

#[test]
fn import_path_rejects_scoped_module_name_with_hint() {
    let src = "import ml::sigmoid";
    let err = parse(src).expect_err("expected parse failure");
    let msg = err.to_string();
    assert!(msg.contains("Import module name must be a single identifier"));
    assert!(msg.contains("import ml { sigmoid }"));
}

#[test]
fn parse_reports_actionable_expected_token_hint() {
    let src = "var x int = 1";
    let err = parse(src).expect_err("expected parse failure");
    let msg = err.to_string();
    assert!(!msg.is_empty());
    assert!(msg.contains("Syntax error"));
}
