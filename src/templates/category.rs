use crate::structs::Category;

markup::define! {
    CategoryPage<'a>(title: &'a str, category: Category) {
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
