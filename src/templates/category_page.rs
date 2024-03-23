use crate::models::product::Product;
use crate::templates::home_page::Head;

markup::define! {
    CategoryPage<'a>(title: &'a str, products: &'a Vec<Product>) {
        @markup::doctype()
        html {
            @Head { title }
            body {
                @LoopProducts { products: products }
            }
        }
    }

    LoopProducts<'a>(products: &'a Vec<Product>) {
        @for product in products.iter() {
            @product "\n"
        }
    }
}
