markup::define! {
    HomePage<'a>(title: &'a str) {
        @markup::doctype()
        html {
            head {
                title { @title }
            }
        }
    }
}
