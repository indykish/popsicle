use super::View;

pub struct ErrorView {
    pub view: View,
}

impl ErrorView {
    pub fn new() -> ErrorView {
        ErrorView {
            view: View::new("dialog-error", "Critical Error Occurred", "", |_| ()),
        }
    }
}
