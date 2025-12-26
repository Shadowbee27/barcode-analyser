use crate::app::bookdata::*;
use crate::app::serial_read::read_serial;
use crate::book_data::google::get_google_book_data;
use crate::food_data::open_food_facts::get_open_food_facts_data;
use eframe::egui::{self};
use egui::ComboBox;
use log::info;
use std::{
  sync::mpsc::{self, RecvTimeoutError},
  thread,
  time::Duration,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::runtime::Runtime;

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
  google_book_data: Data,
  open_food_facts_data: Data,
  new_port: String,
  pub port: String,
  pub current_barcode: i32,
  last_barcode: i32,
  serial_rx: Option<mpsc::Receiver<i32>>,
  serial_handle: Option<thread::JoinHandle<()>>,
}

impl std::fmt::Display for ScannerSetting {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self == &ScannerSetting::Serial {
      write!(f, "Serial")
    } else if self == &ScannerSetting::Manual {
      write!(f, "Manual")
    } else {
      write!(f, "NotSet")
    }
  }
}

impl eframe::App for BarcodeScanner {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let runtime = Runtime::new().unwrap();
    egui_extras::install_image_loaders(ctx);
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Barcode Scanner");

      // Get check if we got a new Barcode and get the needed data
      if self.current_barcode != self.last_barcode {
        info!("Getting new data for barcode: {}", self.current_barcode);
        let openfoodfacts_future = get_open_food_facts_data(self.current_barcode);
        let googlebookapi_future = get_google_book_data(self.current_barcode);
        self.last_barcode = self.current_barcode;
        self.open_food_facts_data = Runtime::block_on(&runtime, openfoodfacts_future);
        self.google_book_data = Runtime::block_on(&runtime, googlebookapi_future);
      }
      // Display the current barcode
      ui.label(format!("Barcode: {}", &self.last_barcode.to_string()));

      // Columns for data sources
      ui.columns(2, |column| {
        // If we have data from Google display it
        if !self.google_book_data.has_data {
          column[0].label("Google Book Api");
        } else {
          column[0].group(|ui| {
            ui.label(&self.google_book_data.data);
          });
        }

        // If we have data from OFF display it
        if !self.open_food_facts_data.has_data {
          column[1].label("Food Facts");
        } else {
          column[1].group(|ui| {
            ui.label(&self.open_food_facts_data.data);
          });
        }
      });

      // settings group
      ui.group(|ui| {
        ui.heading("Settings:");

        ui.label("Mode:");
        ComboBox::from_label("")
          .selected_text(format!("{}", self.scanner_setting))
          .show_ui(ui, |ui| {
            for val in ScannerSetting::iter() {
              ui.selectable_value(&mut self.scanner_setting, val.clone(), format!("{}", &val));
            }
          });

        if self.scanner_setting == ScannerSetting::Serial {
          ui.label("Serial Port:");

          let port = ui.text_edit_singleline(&mut self.new_port);
          if !port.has_focus() && !self.new_port.is_empty() && self.new_port != self.port {
            self.port = self.new_port.clone();
            let (tx, rx) = mpsc::channel();
            let port_clone = self.port.clone();
            self.serial_rx = Some(rx);
            self.serial_handle = Some(thread::spawn(move || read_serial(port_clone, tx)));
            // TODO!
            // let task = tokio::spawn(async move { println!("Serial Port change") });
          }

          if self.port.is_empty() {
            ui.heading("Please set a serrial Port");
          }
        }
      });

      // Check the recv if we got new data. If the recv is disconected we panic as this is not supose to happen
      if self.scanner_setting == ScannerSetting::Serial && !self.port.is_empty() {
        match &self.serial_rx {
          Some(v) => match v.recv_timeout(Duration::from_secs(1)) {
            Ok(recv) => {
              self.current_barcode = recv;
            }
            Err(e) => {
              if e == RecvTimeoutError::Disconnected {
                panic!("Serial Port is inialized but no reciver is active");
              }
            }
          },
          None => panic!("Serial Port is inialized but no reciver is active"),
        }
      }
    });
    ctx.request_repaint_after(Duration::from_millis(100));
  }
}
