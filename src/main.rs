pub mod app;
pub mod book_data;
pub mod food_data;
use eframe::*;

fn main() -> eframe::Result {
  env_logger::init();
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 880.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Barcode Scanner",
    options,
    Box::new(|cc| {
      egui_extras::install_image_loaders(&cc.egui_ctx);
      Ok(Box::<app::app::BarcodeScanner>::default())
    }),
  )
}
