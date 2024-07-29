use crate::structs::product::ProductData;

markup::define! {
    ProductPage<'a>(title: &'a str, product_data: ProductData) {
        @markup::doctype()
        html {
            { title }
        }
        body {
            h1 {@product_data.product_data.name}
            '\n'
            h2 {@product_data.product_data.description.as_ref().unwrap()}

            @for sku in &product_data.sku_data {
                h2 {@sku.name}
            }
        }
    }
}
