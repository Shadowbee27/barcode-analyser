use log::*;
use serialport::*;
use std::io::Read;
use std::str::from_utf8;
use std::sync::mpsc;
use std::time::Duration;

pub fn read_serial(port: String, tx: mpsc::Sender<i64>) {
  info!("Opening port on {}", &port);
  let mut port = new(port, 9600)
    .timeout(Duration::from_hours(3709551615))
    .open()
    .expect("Failed to open port");

  loop {
    let buf: &mut [u8; 14] = &mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    info!("Reading from Port");
    port.read(buf.as_mut_slice()).expect("Found no data!");
    let barcode_id = from_utf8(buf).unwrap();
    info!("Sending barcode: {}", &barcode_id);
    let r = tx.send(barcode_id.trim().parse::<i64>().unwrap());
    println!("{r:?}");
  }
}
