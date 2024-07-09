use crate::routes::product::ProductWithData;

markup::define! {
    ProductPage<'a>(title: &'a str, product_data: ProductWithData) {
        @markup::doctype()
        html {
            { title }
        }
        body {
            h1 {@product_data.product.name}
            '\n'
        }
    }
}
