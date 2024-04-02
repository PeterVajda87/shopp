use crate::models::product::Product;
use crate::templates::home_page::Head;

markup::define! {
    ProductPage<'a>(title: &'a str, product: Product) {
        @markup::doctype()
        html {
            @Head { title }
        }
        body {
            h1 {@product.title}
        }
    }
}
