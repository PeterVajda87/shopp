markup::define! {
    IndexPage<'a>(title: &'a str) {
        @markup::doctype()
        html {
            head {
                title { @title }
            }
        }
    }
}
