use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct OFFData {
  pub has_data: bool,
  pub product: ProductRoot,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ProductRoot {
  pub code: String,
  pub product: item,
  pub status_verbose: String,
}

#[allow(unused, non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct item {
  pub brands: Option<String>,
  pub countries: Option<String>,
  pub ingredients_text: Option<String>,
  pub nutrient_levels: Option<nutrient_levels>,
  pub nutriments: Option<nutriments>,
  pub nutriscore_grade: Option<String>,
  pub pnns_groups_1: Option<String>,
  pub pnns_groups_2: Option<String>,
  pub product_name: Option<String>,
  pub product_type: Option<String>,
  pub quantity: Option<String>,
}

#[allow(unused, non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct nutrient_levels {
  pub fat: Option<String>,
  pub salt: Option<String>,
  #[serde(rename = "saturated-fat")]
  pub saturated_fat: Option<String>,
}

#[allow(unused, non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct nutriments {
  pub energy_100g: Option<i32>,
  pub energy_unit: Option<String>,
  pub sugars_unit: Option<String>,
  pub sugars_value: Option<i32>,
}

impl std::fmt::Display for nutriments {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Sugar: {}{}\nEnergy per 100g: {} {}",
      if self.sugars_value.is_some() {
        self.sugars_value.unwrap().to_string()
      } else {
        "".to_string()
      },
      self.sugars_unit.clone().unwrap_or(String::from("Unknown")),
      self.energy_100g.unwrap_or(-1),
      self.energy_unit.clone().unwrap_or(String::from("value"))
    )
  }
}

impl std::fmt::Display for nutrient_levels {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Fat leves: {}\nSalt levels: {}\nSaturated fat levels: {}",
      self.fat.clone().unwrap_or(String::from("Unknown")),
      self.salt.clone().unwrap_or(String::from("Unknown")),
      self
        .saturated_fat
        .clone()
        .unwrap_or(String::from("Unknown"))
    )
  }
}
