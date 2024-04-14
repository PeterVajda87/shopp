use crate::models::product::{Product, ProductImage};
use crate::templates::home_page::Head;

markup::define! {
    ProductPage<'a>(title: &'a str, product: Product, product_image: ProductImage) {
        @markup::doctype()
        html {
            @Head { title }
        }
        body {
            h1 {@product.title}
            img[src = format!("{}/{}", "static", &product_image.path.as_ref().unwrap())];
            '\n'
        }
    }
}
