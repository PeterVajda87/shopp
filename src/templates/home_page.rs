markup::define! {
    HomePage<'a>(title: &'a str) {
        @markup::doctype()
        html {
            @Head { title }
            @Body { text: "Ahoj!" }
        }
    }
    Head<'a>(title: &'a str) {
        head {
            title { @title }
            link[rel = Some("icon"), type = Some("image/x-icon"), href = "static/favicon.ico"];
            '\n'
        }
    }
    Body<'a>(text: &'a str) {
        div {
            p { @text }
        }
    }
}
