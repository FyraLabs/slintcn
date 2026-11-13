mod common;

use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

#[test]
fn item_runs_its_callback_and_closes_the_menu() {
    let gallery = gallery_section("buttons");
    assert!(!exists(&gallery, "Next week"));
    click(&find(&gallery, "More snooze options"));
    assert!(exists(&gallery, "Next week"), "menu did not open");

    click(&find(&gallery, "Next week"));
    assert_eq!(gallery.get_last_result(), "Snoozed until next week");
    assert!(!exists(&gallery, "Next week"), "menu stayed open");
}

#[test]
fn escape_closes_without_activating() {
    let gallery = gallery_section("buttons");
    click(&find(&gallery, "More snooze options"));
    press_key(gallery.window(), Key::Escape);
    assert!(!exists(&gallery, "Next week"));
    assert_eq!(gallery.get_last_result(), "");
}

#[test]
fn clicking_the_trigger_again_closes_the_menu() {
    let gallery = gallery_section("buttons");
    let trigger = find(&gallery, "More snooze options");
    click(&trigger);
    click(&trigger);
    assert!(!exists(&gallery, "Next week"));
}

#[test]
fn keyboard_moves_focus_and_activates() {
    let gallery = gallery_section("buttons");
    click(&find(&gallery, "More snooze options"));
    press_key(gallery.window(), Key::Tab);
    press_key(gallery.window(), Key::Tab);
    press_key(gallery.window(), Key::Return);
    assert_eq!(gallery.get_last_result(), "Snoozed until tomorrow");
    assert!(!exists(&gallery, "Tomorrow"));
}

#[test]
fn disabled_item_does_nothing() {
    let gallery = gallery_section("menus");
    click(&find(&gallery, "View"));
    let minimap = find(&gallery, "Minimap");
    assert_eq!(minimap.accessible_enabled(), Some(false));
    click(&minimap);
    assert!(exists(&gallery, "Minimap"), "menu closed");
    assert_eq!(minimap.accessible_checked(), Some(false));
}

#[test]
fn checkbox_item_keeps_its_state_after_reopening() {
    let gallery = gallery_section("menus");
    click(&find(&gallery, "View"));
    assert_eq!(
        find(&gallery, "Activity bar").accessible_checked(),
        Some(false)
    );
    click(&find(&gallery, "Activity bar"));
    assert_eq!(gallery.get_last_result(), "Activity bar shown");

    click(&find(&gallery, "View"));
    assert_eq!(
        find(&gallery, "Activity bar").accessible_checked(),
        Some(true)
    );
}

#[test]
fn radio_item_moves_the_selection() {
    let gallery = gallery_section("menus");
    let radio = |label| find_role(&gallery, AccessibleRole::RadioButton, label);
    click(&find(&gallery, "View"));
    assert_eq!(radio("Bottom").accessible_checked(), Some(true));
    click(&radio("Right"));

    click(&find(&gallery, "View"));
    assert_eq!(radio("Right").accessible_checked(), Some(true));
    assert_eq!(radio("Bottom").accessible_checked(), Some(false));
}
