use crate::app::app::BarcodeScanner;
use log::*;
use std::io::Read;
use std::str::from_utf8;
use tokio::time::Duration;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use serial::prelude::*;

#[cfg(target_os = "windows")]
use serial_windows::*;

#[cfg(target_os = "windows")]
pub fn read_serial(port: String) {
  todo!("Not Implemented");
}

impl BarcodeScanner {
  #[cfg(any(target_os = "linux", target_os = "macos"))]
  pub fn read_serial(&mut self) {
    let mut port = serial::open(&self.port).unwrap();
    port
      .reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600).unwrap();
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
      })
      .unwrap();
    port.set_timeout(Duration::from_millis(1000000000)).unwrap();
    let buf: &mut [u8; 14] = &mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    port.read_exact(buf).unwrap();
    let barcode_id = from_utf8(buf).unwrap();
    debug!("{}", &barcode_id);
    self.current_barcode = barcode_id.trim().parse::<i32>().unwrap();
  }
}
/*
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn read_serial(port: String) {}
*/
