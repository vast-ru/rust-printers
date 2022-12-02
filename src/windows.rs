use std::process::Command;
use wmi::{COMLibrary, WMIConnection};
use wmi::utils::WMIError;
use serde::Deserialize;

use crate::printer;
use crate::process;

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct Win32_Printer {
    DriverName: String,
    Name: String,
}

/**
 * Get printers on Windows using WMI
 */
pub fn get_printers() -> Vec<printer::Printer> {

    let com_con = COMLibrary::new().unwrap_or_else(|e| {
        match e {
            WMIError::HResultError { hres }  => match hres {
                // RPC_E_TOO_LATE - CoInitializeSecurity has already been called
                -2147417831 => COMLibrary::without_security(),
                _ => Err(e),
            },
            _ => Err(e),
        }.unwrap()
    });

    let wmi_con = WMIConnection::new(com_con).unwrap();

    let results: Vec<Win32_Printer> = wmi_con.query()
        .unwrap_or(Vec::with_capacity(0));

    let mut printers: Vec<printer::Printer> = Vec::with_capacity(results.len());

    for r in results {
        printers.push(printer::Printer::new(r.Name, r.DriverName, &self::print));
    }

    return printers;
}

/**
 * Print on windows using lpr
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {

    let process = process::exec(
        Command::new("lpr").arg("-S 127.0.0.1").arg("-P").arg(printer_system_name).arg(file_path)
    );

    if process.is_err() {
        return Result::Err(process.unwrap_err());
    }

    return Result::Ok(true);

}
