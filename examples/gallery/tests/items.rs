mod common;

use common::*;
use slint::ComponentHandle;
use slint::platform::Key;
use slintcn_gallery::ItemPreview;

fn test_preview() -> ItemPreview {
    init();
    let preview = ItemPreview::new().unwrap();
    preview.show().unwrap();
    preview
}

#[test]
fn clickable_item_responds_to_pointer_and_accessible_action() {
    let preview = test_preview();
    let item = find_role(&preview, AccessibleRole::Button, "Verified profile");
    click(&item);
    assert_eq!(preview.get_last_result(), "Verified profile");
    assert_eq!(preview.get_activation_count(), 1);
    item.invoke_accessible_default_action();
    assert_eq!(preview.get_last_result(), "Verified profile");
    assert_eq!(preview.get_activation_count(), 2);
}

#[test]
fn nested_action_does_not_activate_the_item() {
    let preview = test_preview();
    click(&find(&preview, "Edit"));
    assert_eq!(preview.get_last_result(), "Edit account");
    assert_eq!(preview.get_activation_count(), 1);
    click(&find(&preview, "Account settings"));
    assert_eq!(preview.get_last_result(), "Account settings");
    assert_eq!(preview.get_activation_count(), 2);
}

#[test]
fn pointer_activation_moves_keyboard_focus_to_the_item() {
    let preview = test_preview();
    press_key(preview.window(), Key::Tab);
    click(&find(&preview, "Verified profile"));
    assert!(preview.get_verified_focused());
    press_key(preview.window(), Key::Space);
    assert_eq!(preview.get_last_result(), "Verified profile");
    assert_eq!(preview.get_activation_count(), 2);
}

#[test]
fn disabled_item_ignores_pointer_and_accessible_action() {
    let preview = test_preview();
    let item = find_role(&preview, AccessibleRole::Button, "Disabled item");
    assert_eq!(item.accessible_enabled(), Some(false));
    click(&item);
    item.invoke_accessible_default_action();
    assert_eq!(preview.get_last_result(), "");
}

#[test]
fn clickable_item_responds_to_space_and_enter_after_tab_focus() {
    let preview = test_preview();
    press_key(preview.window(), Key::Tab);
    press_key(preview.window(), Key::Tab);
    assert!(preview.get_verified_focused());
    press_key(preview.window(), Key::Space);
    assert_eq!(preview.get_last_result(), "Verified profile");
    assert_eq!(preview.get_activation_count(), 1);
    press_key(preview.window(), Key::Return);
    assert_eq!(preview.get_last_result(), "Verified profile");
    assert_eq!(preview.get_activation_count(), 2);
}

#[test]
fn trailing_metadata_keeps_track_items_on_one_content_row() {
    let preview = test_preview();
    let item = find_role(&preview, AccessibleRole::ListItem, "Midnight City Lights");
    let title = ElementHandle::find_by_accessible_label(&preview, "Midnight City Lights")
        .find(|element| element.accessible_role() == Some(AccessibleRole::Text))
        .unwrap();
    let duration = ElementHandle::find_by_accessible_label(&preview, "3:45")
        .next()
        .unwrap();
    assert!(
        item.size().height < 70.,
        "item {:?}, title {:?} at {:?}, duration {:?} at {:?}",
        item.size(),
        title.size(),
        title.absolute_position(),
        duration.size(),
        duration.absolute_position()
    );
    assert!(duration.absolute_position().x > title.absolute_position().x + title.size().width);
}

#[test]
fn long_descriptions_are_clamped_to_two_lines() {
    let preview = test_preview();
    let description = ElementHandle::find_by_element_type_name(&preview, "ItemDescription")
        .find(|element| {
            element
                .accessible_label()
                .is_some_and(|label| label.starts_with("This description wraps"))
        })
        .unwrap();
    assert_eq!(description.size().height, 42.);
}

#[test]
fn static_items_have_list_item_roles_without_activation() {
    let preview = test_preview();
    let item = find_role(&preview, AccessibleRole::ListItem, "Basic item");
    click(&item);
    item.invoke_accessible_default_action();
    assert_eq!(preview.get_last_result(), "");
}
