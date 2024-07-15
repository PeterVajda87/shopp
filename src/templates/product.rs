markup::define! {
    ProductPage<'a>(title: &'a str) {
        @markup::doctype()
        html {
            { title }
        }
        body {
            // h1 {@product_data.product.name}
            // '\n'
            // h2 {@product_data.description.as_ref().unwrap().text}
        }
    }
}
