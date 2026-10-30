slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    Gallery::new()?.run()
}
