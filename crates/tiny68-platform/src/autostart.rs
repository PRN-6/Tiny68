//! Windows startup (HKCU\Software\Microsoft\Windows\CurrentVersion\Run) configuration.

use std::path::PathBuf;
use windows::core::*;
use windows::Win32::System::Registry::*;

const RUN_KEY_PATH: PCWSTR = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");

/// Checks whether the application is configured to launch at Windows startup.
pub fn is_autostart_enabled(app_name: &str) -> bool {
    unsafe {
        let mut hkey = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            RUN_KEY_PATH,
            0,
            KEY_QUERY_VALUE,
            &mut hkey,
        )
        .is_err()
        {
            return false;
        }

        let name_u16: Vec<u16> = app_name.encode_utf16().chain(std::iter::once(0)).collect();
        let mut data_len = 0u32;
        let res = RegQueryValueExW(
            hkey,
            PCWSTR(name_u16.as_ptr()),
            None,
            None,
            None,
            Some(&mut data_len),
        );

        let _ = RegCloseKey(hkey);
        res.is_ok() && data_len > 0
    }
}

/// Enables or disables launching the application at Windows startup.
pub fn set_autostart(app_name: &str, enable: bool) -> std::result::Result<(), std::io::Error> {
    unsafe {
        let mut hkey = HKEY::default();
        let open_res = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            RUN_KEY_PATH,
            0,
            KEY_SET_VALUE | KEY_QUERY_VALUE,
            &mut hkey,
        );

        if open_res.is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Unable to open Windows startup registry key",
            ));
        }

        let name_u16: Vec<u16> = app_name.encode_utf16().chain(std::iter::once(0)).collect();

        let err_code = if enable {
            let exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("tiny68.exe"));
            let formatted = format!("\"{}\"", exe_path.display());
            let val_u16: Vec<u16> = formatted.encode_utf16().chain(std::iter::once(0)).collect();
            let byte_slice = std::slice::from_raw_parts(
                val_u16.as_ptr() as *const u8,
                val_u16.len() * std::mem::size_of::<u16>(),
            );

            RegSetValueExW(hkey, PCWSTR(name_u16.as_ptr()), 0, REG_SZ, Some(byte_slice))
        } else {
            RegDeleteValueW(hkey, PCWSTR(name_u16.as_ptr()))
        };

        let _ = RegCloseKey(hkey);

        if err_code.is_ok() {
            Ok(())
        } else if !enable {
            // Deleting a non-existent value is fine
            Ok(())
        } else {
            Err(std::io::Error::other(format!(
                "Registry operation failed with error code: {:?}",
                err_code
            )))
        }
    }
}
