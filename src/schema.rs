pub trait Schema {
    fn h1_start() -> &'static str;
    fn h1_end() -> &'static str;
    fn h2_start() -> &'static str;
    fn h2_end() -> &'static str;
    fn h3_start() -> &'static str;
    fn h3_end() -> &'static str;
    fn h4_start() -> &'static str;
    fn h4_end() -> &'static str;
    fn h5_start() -> &'static str;
    fn h5_end() -> &'static str;
    fn h6_start() -> &'static str;
    fn h6_end() -> &'static str;
}

pub struct HtmlSchema;

impl Schema for HtmlSchema {
    fn h1_start() -> &'static str {
        "<h1>"
    }

    fn h1_end() -> &'static str {
        "</h1>"
    }

    fn h2_start() -> &'static str {
        "<h2>"
    }

    fn h2_end() -> &'static str {
        "</h2>"
    }

    fn h3_start() -> &'static str {
        "<h3>"
    }

    fn h3_end() -> &'static str {
        "</h3>"
    }

    fn h4_start() -> &'static str {
        "<h4>"
    }

    fn h4_end() -> &'static str {
        "</h4>"
    }

    fn h5_start() -> &'static str {
        "<h5>"
    }

    fn h5_end() -> &'static str {
        "</h5>"
    }

    fn h6_start() -> &'static str {
        "<h6>"
    }

    fn h6_end() -> &'static str {
        "</h6>"
    }

}

impl HtmlSchema {
    pub fn new() -> Self {
        Self
    }
}