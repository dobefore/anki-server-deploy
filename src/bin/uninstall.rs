use ankiserver::{ApplicationError, ShellExecuteW, PCWSTR};

fn main() -> Result<(), ApplicationError> {
    let mut admin = "runas".encode_utf16().collect::<Vec<u16>>();
    admin.push(0);
    let app_path = format!("{}", std::env::current_dir()?.join("no.exe").display());
    let mut app_to_run = app_path.encode_utf16().collect::<Vec<_>>();
    app_to_run.push(0);
    let r = unsafe {
        // runas
        ShellExecuteW(
            None,
            PCWSTR::from_raw(admin.as_ptr()),
            PCWSTR::from_raw(app_to_run.as_ptr()),
            None,
            None,
            1,
        )
    };

    if r.0 < 32 {
        eprintln!("error: {:?}", r);
    }
    Ok(())
}
