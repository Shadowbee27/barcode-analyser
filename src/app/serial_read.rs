use log::*;
use serialport::*;
use std::io::Read;
use std::str::from_utf8;
use std::sync::mpsc;
use std::time::Duration;

pub fn read_serial(port: String, tx: mpsc::Sender<i64>) {
  info!("Your log level is set to info");
  debug!("Your log level is set to debug");
  warn!("Your log level is set to warn");
  error!("Your log level is set to error");
  let mut port = new(port, 9600)
    .timeout(Duration::from_hours(3709551615))
    .open()
    .expect("Failed to open port");

  loop {
    let buf: &mut [u8; 14] = &mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    info!("Reading from Port");
    port.read(buf.as_mut_slice()).expect("Found no data!");
    let barcode_id = from_utf8(buf).unwrap();
    println!("{}", &barcode_id);
    info!("Sending barcode: {}", &barcode_id);
    let _ = tx.send(barcode_id.trim().parse::<i64>().unwrap());
  }
}
