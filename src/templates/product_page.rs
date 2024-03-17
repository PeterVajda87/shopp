use crate::templates::home_page::Head;

markup::define! {
    ProductPage<'a>(title: &'a str) {
        @markup::doctype()
        html {
            @Head { title }
        }
    }
}
