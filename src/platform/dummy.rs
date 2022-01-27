use crate::ClipboardProvider;

use raw_window_handle::HasRawWindowHandle;

struct Dummy;

pub fn connect<W: HasRawWindowHandle>(
    _window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn std::error::Error>> {
    Ok(Box::new(Dummy))
}

impl ClipboardProvider for Dummy {
    fn read(&self) -> Result<String, Box<dyn std::error::Error>> {
        Err(Box::new(Error::Unimplemented))
    }

    fn write(
        &mut self,
        contents: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(Error::Unimplemented))
    }
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
enum Error {
    #[error("unimplemented")]
    Unimplemented,
}
