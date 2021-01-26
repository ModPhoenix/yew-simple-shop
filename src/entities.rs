use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination<T> {
  pub items: Vec<T>,
  pub total_items: u32,
  pub page: u32,
  pub per_page: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Product {
  pub id: String,
  pub name: String,
  pub price: u32,
  pub origin: String,
  pub created_at: String,
  pub updated_at: String,
  pub is_editable: bool,
}

pub type ProductsList = Pagination<Product>;
