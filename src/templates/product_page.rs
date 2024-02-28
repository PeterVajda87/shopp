markup::define! {
    ProductPage<'a>(title: &'a str) {
        @markup::doctype()
        html {
            head {
                title { @title }
            }
        }
    }
}
