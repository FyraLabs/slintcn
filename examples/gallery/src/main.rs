use slint::ComponentHandle;
use slintcn_gallery::Gallery;

fn main() -> Result<(), slint::PlatformError> {
    Gallery::new()?.run()
}
