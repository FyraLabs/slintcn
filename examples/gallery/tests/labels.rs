mod common;

use common::*;
use slint::ComponentHandle;
use slintcn_gallery::LabelPreview;

fn test_preview() -> LabelPreview {
    init();
    let preview = LabelPreview::new().unwrap();
    preview.show().unwrap();
    preview
}

fn label(preview: &LabelPreview, text: &str) -> ElementHandle {
    ElementHandle::find_by_accessible_label(preview, text)
        .find(|element| element.accessible_role() == Some(AccessibleRole::Text))
        .expect("label not found")
}

#[test]
fn clicking_label_toggles_associated_checkbox() {
    let preview = test_preview();
    let label = label(&preview, "Accept terms and conditions");
    click(&label);
    assert!(preview.get_accepted());
    click(&label);
    assert!(!preview.get_accepted());
}

#[test]
fn clicking_label_focuses_associated_input() {
    let preview = test_preview();
    click(&label(&preview, "Email address"));
    assert!(preview.get_email_focused());
}

#[test]
fn disabled_label_does_not_toggle_associated_checkbox() {
    let preview = test_preview();
    let label = label(&preview, "Disabled label");
    assert_eq!(label.accessible_enabled(), Some(false));
    click(&label);
    assert!(!preview.get_disabled_accepted());
}
