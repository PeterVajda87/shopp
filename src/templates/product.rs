use crate::routes::product::ProductData;

markup::define! {
    ProductPage<'a>(title: &'a str, product_data: ProductData) {
        @markup::doctype()
        html {
            { title }
        }
        body {
            h1 {@product_data.name}
            '\n'
            h2 {@product_data.description.text}
        }
    }
}
