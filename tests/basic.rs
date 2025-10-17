#[test]
fn layout_exists() {
    let layout = phpvm::dirs::ensure_layout();
    assert!(layout.is_ok());
}
