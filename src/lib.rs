mod error;
pub use error::ApplicationError;
pub use windows::core::PCWSTR;
pub use windows::{Win32::Foundation::*, Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*};
