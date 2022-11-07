use ankiserver::ApplicationError;
use send::Send;
fn main() -> Result<(), ApplicationError> {
    Send::new().set_desktop()?.set_start().send()?;
    Ok(())
}
