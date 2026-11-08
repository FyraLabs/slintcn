mod common;

use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

#[test]
fn button_click_runs_its_callback() {
    let gallery = gallery();
    click(&find(&gallery, "Archive"));
    assert_eq!(gallery.get_last_result(), "Archived");
}

#[test]
fn disabled_button_ignores_clicks() {
    let gallery = gallery();
    let disabled: Vec<_> = find_all(&gallery, "Disabled");
    assert!(!disabled.is_empty());
    for button in &disabled {
        assert_eq!(button.accessible_enabled(), Some(false));
        click(button);
    }
    assert_eq!(gallery.get_last_result(), "");
}

#[test]
fn switch_toggles_on_click() {
    let gallery = gallery();
    let switch = find_role(&gallery, AccessibleRole::Switch, "Airplane mode");
    assert_eq!(switch.accessible_checked(), Some(false));
    click(&switch);
    assert_eq!(switch.accessible_checked(), Some(true));
    assert_eq!(gallery.get_last_result(), "Airplane mode on");
}

#[test]
fn disabled_switch_stays_off() {
    let gallery = gallery();
    let switch = find_role(&gallery, AccessibleRole::Switch, "Location services");
    assert_eq!(switch.accessible_enabled(), Some(false));
    click(&switch);
    assert_eq!(switch.accessible_checked(), Some(false));
}

#[test]
fn checkbox_toggles_through_its_accessible_action() {
    let gallery = gallery();
    let checkbox = find_role(&gallery, AccessibleRole::Checkbox, "Accept terms and conditions");
    checkbox.invoke_accessible_default_action();
    settle();
    assert_eq!(checkbox.accessible_checked(), Some(true));
    assert_eq!(gallery.get_last_result(), "Terms accepted");
}

#[test]
fn dialog_closes_on_escape() {
    let gallery = gallery();
    click(&find(&gallery, "Open dialog"));
    assert!(exists(&gallery, "Save changes"), "dialog did not open");
    press_key(gallery.window(), Key::Escape);
    assert_eq!(gallery.get_last_result(), "Edit dialog dismissed");
    assert!(!exists(&gallery, "Save changes"));
}

#[test]
fn alert_dialog_cancel_runs_its_callback() {
    let gallery = gallery();
    let card_cancel = find(&gallery, "Cancel").absolute_position();
    click(&find(&gallery, "Delete account"));
    let cancel: Vec<_> = find_all(&gallery, "Cancel").into_iter().filter(|e| e.absolute_position() != card_cancel).collect();
    assert_eq!(cancel.len(), 1, "alert dialog did not open");
    click(&cancel[0]);
    assert_eq!(gallery.get_last_result(), "Delete cancelled");
}
