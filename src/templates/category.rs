markup::define! {
    CategoryPage<'a>(title: &'a str) {
        @markup::doctype()
        html {
            { title }
        }
        // body {
        //     h1 {@category.name}
        //     '\n'
        // }
    }
}
