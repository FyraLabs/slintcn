mod common;

use common::*;
use slint::language::ColorScheme;
use slint::{ComponentHandle, LogicalSize};
use slintcn_gallery::{ButtonGroupPreview, CardPreview, FieldPreview, MenuPreview};

const SCHEMES: [(ColorScheme, &str); 2] = [(ColorScheme::Light, "light"), (ColorScheme::Dark, "dark")];

#[test]
fn gallery() {
    let gallery = common::gallery();
    for (scheme, name) in SCHEMES {
        gallery.set_color_scheme(scheme);
        assert_screenshot(gallery.window(), &format!("gallery-{name}"));
    }
}

#[test]
fn fields() {
    init();
    let preview = FieldPreview::new().unwrap();
    preview.show().unwrap();
    for (scheme, name) in SCHEMES {
        preview.set_color_scheme(scheme);
        assert_screenshot(preview.window(), &format!("fields-{name}"));
    }
}

#[test]
fn cards() {
    init();
    let preview = CardPreview::new().unwrap();
    preview.show().unwrap();
    for (scheme, name) in SCHEMES {
        preview.set_color_scheme(scheme);
        assert_screenshot(preview.window(), &format!("cards-{name}"));
    }
}

#[test]
fn button_groups() {
    init();
    let preview = ButtonGroupPreview::new().unwrap();
    preview.show().unwrap();
    for (scheme, name) in SCHEMES {
        preview.set_color_scheme(scheme);
        assert_screenshot(preview.window(), &format!("button-groups-{name}"));
    }
}

#[test]
fn open_dropdown_menu() {
    init();
    let preview = ButtonGroupPreview::new().unwrap();
    preview.show().unwrap();
    for (scheme, name) in SCHEMES {
        preview.set_color_scheme(scheme);
        click(&find(&preview, "More snooze options"));
        press_key(preview.window(), slint::platform::Key::Tab);
        press_key(preview.window(), slint::platform::Key::Tab);
        assert_screenshot(preview.window(), &format!("dropdown-menu-{name}"));
        press_key(preview.window(), slint::platform::Key::Escape);
    }
}

#[test]
fn open_checkbox_and_radio_menu() {
    init();
    let preview = MenuPreview::new().unwrap();
    preview.window().set_size(LogicalSize::new(720., 480.));
    preview.show().unwrap();
    for (scheme, name) in SCHEMES {
        preview.set_color_scheme(scheme);
        click(&find(&preview, "View"));
        assert_screenshot(preview.window(), &format!("dropdown-menu-items-{name}"));
        press_key(preview.window(), slint::platform::Key::Escape);
    }
}

#[test]
fn open_popover() {
    init();
    let preview = MenuPreview::new().unwrap();
    preview.show().unwrap();
    for (scheme, name) in SCHEMES {
        preview.set_color_scheme(scheme);
        click(&find(&preview, "Dimensions"));
        assert_screenshot(preview.window(), &format!("popover-{name}"));
        press_key(preview.window(), slint::platform::Key::Escape);
    }
}
