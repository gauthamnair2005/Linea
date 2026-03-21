use linea_ast::parse;
use linea_codegen::generate_rust_code;

#[test]
fn missing_module_fails_with_clear_error() {
    let src = "import does_not_exist";
    let program = parse(src).expect("parse should succeed");
    let err = generate_rust_code(&program).expect_err("codegen should fail");
    let msg = err.to_string();
    assert!(msg.contains("module 'does_not_exist' was not found"));
}

#[test]
fn unknown_import_symbol_fails_with_exports_preview() {
    let src = "import math { doesNotExist }";
    let program = parse(src).expect("parse should succeed");
    let err = generate_rust_code(&program).expect_err("codegen should fail");
    let msg = err.to_string();
    assert!(msg.contains("doesNotExist"));
    assert!(msg.contains("math"));
    assert!(msg.contains("Available symbols"));
}
