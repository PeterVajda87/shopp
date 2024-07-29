use crate::structs::sku::SkuData;

markup::define! {
    SkuPage<'a>(title: &'a str, sku_data: SkuData) {
        @markup::doctype()
        html {
            { title }
        }
        body {
        }
    }
}
