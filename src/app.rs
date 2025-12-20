use eframe::egui;
use std::time::Duration;
mod serial_read;
#[derive(Debug, Default)]
pub struct Data {
  has_data: bool,
  data: String,
}

#[derive(Debug, Default)]
pub struct BarcodeScanner {
  port: String,
  current_barcode: i32,
  last_barcode: i32,
}

impl eframe::App for BarcodeScanner {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> () {
    egui_extras::install_image_loaders(ctx);
    egui::CentralPanel::default().show(ctx, |ui| {
      if self.current_barcode != self.last_barcode {
        self.last_barcode = self.current_barcode;
      }
      ui.heading("Barcode Scanner");
      let port = ui.text_edit_singleline(&mut self.port);
      if !port.has_focus() { /*set the port after editing the field is done */ }
      ui.label(format!("Barcode: {}", &self.last_barcode.to_string()));
      ui.columns(3, |column| {
        column[0].label("Google Api");
        column[1].label("Open Libary");
        column[2].label("Food Facts");
        column[1].group(|ui| {
          ui.label("");
        })
      });
    });
    ctx.request_repaint_after(Duration::from_millis(100));
  }
}
