mod common;

use common::*;
use slint::ComponentHandle;
use slint::platform::Key;
use slintcn_gallery::TabsPreview;

fn preview() -> TabsPreview {
    init();
    let preview = TabsPreview::new().unwrap();
    preview.show().unwrap();
    settle();
    preview
}

#[test]
fn clicking_and_activating_updates_the_selection() {
    let preview = preview();
    assert_eq!(
        find_role(&preview, AccessibleRole::Tab, "Account").accessible_item_selected(),
        Some(true)
    );
    click(&find_role(&preview, AccessibleRole::Tab, "Password"));
    assert_eq!(preview.get_account(), "password");
    assert_eq!(
        find_role(&preview, AccessibleRole::Tab, "Password").accessible_item_selected(),
        Some(true)
    );
    assert_eq!(
        find_role(&preview, AccessibleRole::Tab, "Account").accessible_item_selected(),
        Some(false)
    );
    find_role(&preview, AccessibleRole::Tab, "Account").invoke_accessible_default_action();
    settle();
    assert_eq!(preview.get_account(), "account");
}

#[test]
fn disabled_tabs_do_not_select() {
    let preview = preview();
    let disabled = find_role(&preview, AccessibleRole::Tab, "Disabled");
    assert_eq!(disabled.accessible_enabled(), Some(false));
    click(&disabled);
    disabled.invoke_accessible_default_action();
    settle();
    assert_eq!(preview.get_account(), "account");
    assert_eq!(disabled.accessible_item_selected(), Some(false));
}

#[test]
fn arrows_move_focus_without_selecting_and_skip_disabled_tabs() {
    let preview = preview();
    click(&find_role(&preview, AccessibleRole::Tab, "Account"));
    press_key(preview.window(), Key::RightArrow);
    assert_eq!(preview.get_account(), "account");
    press_key(preview.window(), Key::Space);
    assert_eq!(preview.get_account(), "password");
    press_key(preview.window(), Key::RightArrow);
    press_key(preview.window(), Key::Space);
    assert_eq!(preview.get_account(), "account");
    press_key(preview.window(), Key::RightArrow);
    press_key(preview.window(), Key::Home);
    press_key(preview.window(), Key::Space);
    assert_eq!(preview.get_account(), "account");
}

#[test]
fn right_arrow_wraps_to_the_start_and_end_selects_last_enabled_tab() {
    let preview = preview();
    click(&find_role(&preview, AccessibleRole::Tab, "Password"));
    press_key(preview.window(), Key::RightArrow);
    press_key(preview.window(), Key::Space);
    assert_eq!(preview.get_account(), "account");
    press_key(preview.window(), Key::End);
    press_key(preview.window(), Key::Space);
    assert_eq!(preview.get_account(), "password");
}

#[test]
fn vertical_tabs_use_vertical_arrows() {
    let preview = preview();
    click(&find_role(
        &preview,
        AccessibleRole::Tab,
        "Vertical account",
    ));
    press_key(preview.window(), Key::RightArrow);
    assert_eq!(preview.get_vertical(), "account");
    press_key(preview.window(), Key::DownArrow);
    press_key(preview.window(), Key::Space);
    assert_eq!(preview.get_vertical(), "password");
    press_key(preview.window(), Key::UpArrow);
    press_key(preview.window(), Key::Space);
    assert_eq!(preview.get_vertical(), "account");
}

#[test]
fn gallery_opens_the_tabs_section() {
    let gallery = gallery();
    click(&find_role(&gallery, AccessibleRole::Tab, "Tabs"));
    assert_eq!(gallery.get_selected_section(), "tabs");
    assert!(exists(&gallery, "Account"));
}

#[test]
fn gallery_unmounts_inactive_examples() {
    let gallery = gallery();
    assert!(!exists(&gallery, "Profile"));
    click(&find_role(&gallery, AccessibleRole::Tab, "Fields"));
    assert_eq!(gallery.get_selected_section(), "fields");
    assert!(exists(&gallery, "Profile"));
    click(&find_role(&gallery, AccessibleRole::Tab, "Buttons"));
    assert!(!exists(&gallery, "Profile"));
    assert!(exists(&gallery, "Archive"));
}
