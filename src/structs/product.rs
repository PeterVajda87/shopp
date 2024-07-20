use super::sku::SkuData;
use sea_orm::FromQueryResult;

#[derive(FromQueryResult, Debug)]
pub struct ProductData {
    pub name: String,
    pub description: Option<String>,
}

pub struct ProductWithSku {
    pub product_data: ProductData,
    pub sku_data: Vec<SkuData>,
}

impl ProductWithSku {
    pub fn new(product_data: ProductData, sku_data: Vec<SkuData>) -> Self {
        Self {
            product_data,
            sku_data,
        }
    }
}
