use crate::app::{bookdata::*, fooddata::*, serial_read::read_serial};
use crate::book_data::google::get_google_book_data;
use crate::food_data::open_food_facts::get_open_food_facts_data;
use gpui::*;
use gpui_component::{button::*, *};
use log::{error, info};
use std::{sync::mpsc, thread, time::Duration};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Default, Debug, PartialEq, EnumIter, Clone)]
enum ScannerSetting {
  #[default]
  NotSet,
  Manual,
  Serial,
}

#[derive(Debug, Default)]
pub struct BarcodeScanner {
  scanner_setting: ScannerSetting,
  google_book_data: GBookData,
  open_food_facts_data: OFFData,
  new_port: String,
  pub port: String,
  pub current_barcode: i64,
  serial_retry: i8,
  serial_error: String,
  man_barcode: String,
  last_barcode: i64,
  serial_rx: Option<mpsc::Receiver<i64>>,
  serial_handle: Option<thread::JoinHandle<()>>,
}

impl std::fmt::Display for ScannerSetting {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ScannerSetting::Manual => write!(f, "Manual"),
      ScannerSetting::NotSet => write!(f, "Unset"),
      ScannerSetting::Serial => write!(f, "Serial"),
    }
  }
}

pub const UNKNOWN: &str = "Unknown";

impl Render for BarcodeScanner {
  fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    div()
      .v_flex()
      .gap_2()
      .size_full()
      .items_center()
      .justify_center()
      .child("Hello, World!")
      .child(
        Button::new("ok")
          .primary()
          .label("Let's Go!")
          .on_click(|_, _, _| println!("Clicked!")),
      )
  }
}
