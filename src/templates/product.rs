use crate::entities::product::Model;

markup::define! {
    ProductPage<'a>(title: &'a str, product: Model) {
        @markup::doctype()
        html {
            { title }
        }
        body {
            h1 {@product.name}
            '\n'
        }
    }
}
