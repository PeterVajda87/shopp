use crate::entities::category::Model;

markup::define! {
    CategoryPage<'a>(title: &'a str, category: Model) {
        @markup::doctype()
        html {
            { title }
        }
        body {
            h1 {@category.name}
            '\n'
        }
    }
}
