use crate::app::bookdata::*;
use crate::book_data::google::get_google_book_data;
use crate::book_data::open_libary::get_open_libary_data;
use crate::food_data::open_food_facts::get_open_food_facts_data;
use eframe::egui::{self, Response};
use std::time::Duration;
use tokio::runtime::Runtime;

#[derive(Debug, Default, Clone)]
pub struct BarcodeScanner {
  google_book_data: Data,
  open_food_facts_data: Data,
  open_libary_data: Data,
  new_port: String,
  pub port: String,
  pub current_barcode: i32,
  last_barcode: i32,
}

impl eframe::App for BarcodeScanner {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> () {
    let runtime = Runtime::new().unwrap();
    egui_extras::install_image_loaders(ctx);
    egui::CentralPanel::default().show(ctx, |ui| {
      // Get check if we got a new Barcode and get the needed data
      if self.current_barcode != self.last_barcode {
        let openfoodfacts_future = get_open_food_facts_data(self.current_barcode);
        let googlebookapi_future = get_google_book_data(self.current_barcode);
        let openlibary_future = get_open_libary_data(self.current_barcode);
        self.last_barcode = self.current_barcode;
        self.open_food_facts_data = Runtime::block_on(&runtime, openfoodfacts_future);
        self.google_book_data = Runtime::block_on(&runtime, googlebookapi_future);
        self.open_libary_data = Runtime::block_on(&runtime, openlibary_future);
      }

      ui.heading("Barcode Scanner");
      let port: Response = ui.text_edit_singleline(&mut self.new_port);
      if !port.has_focus() && !self.port.is_empty() && self.new_port != self.port {
        /* set the port after editing the field is done */
        //let (tx, rx) = channel::<String>(100);
        tokio::spawn(async move {
          let mut new_self = self.clone();
          BarcodeScanner::read_serial(&mut new_self);
        });
      }
      ui.label(format!("Barcode: {}", &self.last_barcode.to_string()));

      ui.columns(3, |column| {
        if !self.google_book_data.has_data {
          column[0].label("Google Api");
        } else {
          column[0].group(|ui| {
            ui.label(&self.google_book_data.data);
          });
        }

        if !self.open_food_facts_data.has_data {
          column[2].label("Food Facts");
        } else {
          column[2].group(|ui| {
            ui.label(&self.open_food_facts_data.data);
          });
        }

        if !self.open_libary_data.has_data {
          column[1].label("Open Libary");
        } else {
          column[1].group(|ui| {
            ui.label(&self.open_libary_data.data);
          });
        }
      });
    });
    ctx.request_repaint_after(Duration::from_millis(100));
  }
}
