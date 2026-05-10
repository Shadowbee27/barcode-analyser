use crate::app::fooddata::*;
use log::{debug, error};
use openfoodfacts as off;

pub fn get_open_food_facts_data(ean13: i64) -> OFFData {
  let result: OFFData;
  let client = off::v2().build().unwrap();
  let data = client.product(ean13.to_string().as_str(), None).unwrap();
  let json: ProductRoot = serde_json::from_str(data.text().unwrap().as_str()).unwrap();
  debug!("{:?}", json);
  if "product found" == json.status_verbose {
    debug!("{:?}", json);
    result = OFFData {
      has_data: true,
      product: json,
    };
  } else {
    error!("Code is: {}", json.status_verbose);
    result = OFFData {
      has_data: false,
      product: ProductRoot::default(),
    };
  }
  result
}
