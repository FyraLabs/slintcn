#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::time::Duration;

use i_slint_backend_testing::testing_backend::{TestingBackend, TestingBackendOptions};
use slint::platform::{PointerEventButton, WindowEvent};
use slint::{ComponentHandle, LogicalPosition, SharedString};

pub use i_slint_backend_testing::{AccessibleRole, ElementHandle};

/// Installs the testing backend with the software renderer on this test thread.
pub fn init() {
    slint::platform::set_platform(Box::new(TestingBackend::new(TestingBackendOptions {
        mock_time: true,
        threading: false,
        renderer_name: Some("software".into()),
    })))
    .expect("platform already initialized on this thread");
}

/// Opens one gallery section in a window tall enough to cover all of its examples.
pub fn gallery_section(section: &str) -> slintcn_gallery::Gallery {
    init();
    let gallery = slintcn_gallery::Gallery::new().unwrap();
    gallery.set_selected_section(section.into());
    gallery
        .window()
        .set_size(slint::LogicalSize::new(960., 4800.));
    gallery.show().unwrap();
    settle();
    gallery
}

pub fn gallery() -> slintcn_gallery::Gallery {
    gallery_section("buttons")
}

/// Advances mocked time past every 150–200ms transition.
pub fn settle() {
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(500));
}

pub fn press_key(window: &slint::Window, text: impl Into<SharedString>) {
    let text = text.into();
    window.dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    window.dispatch_event(WindowEvent::KeyReleased { text });
    settle();
}

/// Elements with this accessible label, skipping the `Text` that repeats a control's label.
pub fn find_all(component: &impl ComponentHandle, label: &str) -> Vec<ElementHandle> {
    ElementHandle::find_by_accessible_label(component, label)
        .filter(|e| e.accessible_role() != Some(AccessibleRole::Text))
        .collect()
}

/// Finds the one element with this accessible label, including inside open popups.
pub fn find(component: &impl ComponentHandle, label: &str) -> ElementHandle {
    let mut found = find_all(component, label);
    assert_eq!(
        found.len(),
        1,
        "expected one element labelled {label:?}, found {}",
        found.len()
    );
    found.remove(0)
}

/// Finds the one element with this role and accessible label.
pub fn find_role(
    component: &impl ComponentHandle,
    role: AccessibleRole,
    label: &str,
) -> ElementHandle {
    let mut found: Vec<_> = find_all(component, label)
        .into_iter()
        .filter(|e| e.accessible_role() == Some(role))
        .collect();
    assert_eq!(
        found.len(),
        1,
        "expected one {role:?} labelled {label:?}, found {}",
        found.len()
    );
    found.remove(0)
}

pub fn exists(component: &impl ComponentHandle, label: &str) -> bool {
    !find_all(component, label).is_empty()
}

pub fn click(element: &ElementHandle) {
    element.mock_single_click(PointerEventButton::Left);
    settle();
}

pub fn click_at(window: &slint::Window, x: f32, y: f32) {
    let position = LogicalPosition::new(x, y);
    let button = PointerEventButton::Left;
    window.dispatch_event(WindowEvent::PointerMoved { position });
    window.dispatch_event(WindowEvent::PointerPressed { position, button });
    window.dispatch_event(WindowEvent::PointerReleased { position, button });
    window.dispatch_event(WindowEvent::PointerExited);
    settle();
}

fn target_dir() -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target"))
}

fn failures_dir() -> PathBuf {
    target_dir().join("screenshot-failures")
}

/// Largest per-channel difference that still counts as the same pixel.
const CHANNEL_TOLERANCE: u8 = 8;

/// Compares the window with `target/screenshots/<name>.png`, recording it on the first run.
/// `SLINTCN_UPDATE_SCREENSHOTS=1` rewrites existing baselines.
pub fn assert_screenshot(window: &slint::Window, name: &str) {
    settle();
    let snapshot = window.take_snapshot().expect("take_snapshot failed");
    let actual = image::RgbaImage::from_raw(
        snapshot.width(),
        snapshot.height(),
        snapshot.as_bytes().to_vec(),
    )
    .expect("snapshot buffer has the wrong size");

    let baseline_dir = target_dir().join("screenshots");
    let baseline_path = baseline_dir.join(format!("{name}.png"));
    let update = std::env::var_os("SLINTCN_UPDATE_SCREENSHOTS").is_some_and(|v| v != "0");
    if update || !baseline_path.exists() {
        std::fs::create_dir_all(&baseline_dir).unwrap();
        actual.save(&baseline_path).unwrap();
        return;
    }

    let expected = image::open(&baseline_path)
        .expect("unreadable baseline")
        .to_rgba8();
    let reason = if expected.dimensions() != actual.dimensions() {
        format!(
            "size {:?} differs from the baseline's {:?}",
            actual.dimensions(),
            expected.dimensions()
        )
    } else {
        let mut diff = image::RgbaImage::new(actual.width(), actual.height());
        let mut differing = 0usize;
        for (x, y, a) in actual.enumerate_pixels() {
            let e = expected.get_pixel(x, y);
            let delta =
                a.0.iter()
                    .zip(e.0)
                    .map(|(a, e)| a.abs_diff(e))
                    .max()
                    .unwrap();
            let pixel = if delta > CHANNEL_TOLERANCE {
                differing += 1;
                [255, 0, 0, 255]
            } else {
                let gray = ((a.0[0] as u16 + a.0[1] as u16 + a.0[2] as u16) / 6) as u8;
                [gray, gray, gray, 255]
            };
            diff.put_pixel(x, y, image::Rgba(pixel));
        }
        if differing == 0 {
            return;
        }
        std::fs::create_dir_all(failures_dir()).unwrap();
        diff.save(failures_dir().join(format!("{name}.diff.png")))
            .unwrap();
        format!("{differing} pixels differ")
    };

    std::fs::create_dir_all(failures_dir()).unwrap();
    let actual_path = failures_dir().join(format!("{name}.actual.png"));
    actual.save(&actual_path).unwrap();
    let actual_path = actual_path.canonicalize().unwrap_or(actual_path);
    panic!("screenshot {name}: {reason}; see {}", actual_path.display());
}
