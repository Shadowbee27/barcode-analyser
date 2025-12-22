#[cfg(target_os = "linux")]
use serial::*;
#[cfg(target_os = "windows")]
use serial_windows::*;

#[cfg(target_os = "windows")]
pub fn read_serial(port: String) {
  todo!("Not Implemented");
}

#[cfg(target_os = "linux")]
pub fn read_serial(port: String) {
  todo!("Not Implemented");
}
