use log::*;
use std::io::Read;
use std::str::from_utf8;
use std::sync::mpsc;
use std::time::Duration;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use serial::prelude::*;

#[cfg(target_os = "windows")]
use serial_windows::*;

#[cfg(target_os = "windows")]
pub fn read_serial(port: String) {
  todo!("Not Implemented");
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn read_serial(port: String, tx: mpsc::Sender<i64>) {
  info!("Your log level is set to info");
  debug!("Your log level is set to debug");
  warn!("Your log level is set to warn");
  error!("Your log level is set to error");
  let mut port = serial::open(&port).unwrap();
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

  loop {
    port.set_timeout(Duration::from_millis(1000000000)).unwrap();
    let buf: &mut [u8; 14] = &mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    info!("Reading from Port");
    port.read_exact(buf).unwrap();
    let barcode_id = from_utf8(buf).unwrap();
    info!("{}", &barcode_id);
    info!("Sending barcode: {}", &barcode_id);
    let _ = tx.send(barcode_id.trim().parse::<i64>().unwrap());
  }
}
