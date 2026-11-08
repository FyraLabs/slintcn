mod common;

use common::*;
use slint::ComponentHandle;
use slint::platform::Key;
use slintcn_gallery::Gallery;

fn gallery_with_open_popover() -> Gallery {
    let gallery = gallery();
    click(&find(&gallery, "Add to list"));
    assert!(exists(&gallery, "List name"), "popover did not open");
    gallery
}

#[test]
fn clicking_inside_keeps_it_open() {
    let gallery = gallery_with_open_popover();
    click(&find_role(&gallery, AccessibleRole::TextInput, "List name"));
    assert!(exists(&gallery, "List name"));
}

#[test]
fn submitting_the_field_closes_it() {
    let gallery = gallery_with_open_popover();
    let input = find_role(&gallery, AccessibleRole::TextInput, "List name");
    click(&input);
    input.set_accessible_value("Friends");
    press_key(gallery.window(), Key::Return);
    assert_eq!(gallery.get_last_result(), "Added to Friends");
    assert!(!exists(&gallery, "List name"));
}

#[test]
fn escape_closes_it() {
    let gallery = gallery_with_open_popover();
    press_key(gallery.window(), Key::Escape);
    assert!(!exists(&gallery, "List name"));
}

#[test]
fn clicking_outside_closes_it() {
    let gallery = gallery_with_open_popover();
    click_at(gallery.window(), 900., 20.);
    assert!(!exists(&gallery, "List name"));
}
