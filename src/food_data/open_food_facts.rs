use crate::app::fooddata::*;
use log::{debug, error, info, warn};
use openfoodfacts as off;

pub fn get_open_food_facts_data(ean13: i64) -> OFFData {
  let result: OFFData;
  let client = off::v0().build().unwrap();
  debug!("Getting OFF data");
  let data = client
    .product(ean13.to_string().as_str(), None)
    .map(|resp| resp.json::<ProductRoot>().unwrap());
  debug!("Got OFF data");
  match data {
    Ok(product) => {
      if "product found" == product.status_verbose {
        debug!("{:?}", product);
        info!("Got data form OFF");
        result = OFFData {
          has_data: true,
          product,
        };
      } else {
        warn!("OFF Api return code isn't a success code");
        result = OFFData {
          has_data: false,
          product: ProductRoot::default(),
        };
      }
    }
    Err(e) => {
      error!("An error happened while getting OFF data: {}", e);
      result = OFFData {
        // This might be changed to be an err later but I can't be bothered
        has_data: false,
        product: ProductRoot::default(),
      }
    }
  };
  result
}

// fn convert_to_string(product: ProductRoot) -> String {
//   let product = product.product;
//   format!(
//     "Product: {}\nBrand: {}\nCountrie: {}\nIngredients: {}\nNutriscore: {}\n Groups: {}, {}\nType: {}",
//     product.product_name,
//     product.brands,
//     product.countries,
//     product.ingredients_text,
//     product.nutriscore_grade,
//     product.pnns_groups_1,
//     product.pnns_groups_2,
//     product.product_type
//   )
// }
