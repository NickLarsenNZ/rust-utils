use std::fmt::Display;

use url::{ParseError, Url};

#[derive(Clone, Debug, PartialEq)]
pub struct HyperLink {
    text: String,
    link: Url,
}

impl HyperLink {
    pub fn new(text: impl AsRef<str>, link: impl AsRef<str>) -> Result<Self, ParseError> {
        let text = text.as_ref().to_owned();
        let link = link.as_ref().try_into()?;
        Ok(Self { text, link })
    }
}

impl Display for HyperLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let HyperLink { text, link } = self;
        write!(f, "\x1b]8;;{link}\x1b\\{text}\x1b]8;;\x1b\\\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hyperlink_construction() {
        let expected = HyperLink {
            text: "Display Text".to_owned(),
            link: Url::parse("http://example.com").expect("valid url"),
        };
        let result =
            HyperLink::new("Display Text", "http://example.com").expect("valid construction");

        assert_eq!(expected, result);
    }

    #[test]
    fn hyperlink_display() {
        let hyperlink =
            HyperLink::new("Display Text", "http://example.com").expect("valid construction");

        let expected = "\x1b]8;;http://example.com/\x1b\\Display Text\x1b]8;;\x1b\\\n";
        let result = format!("{hyperlink}");

        assert_eq!(expected, result);
    }
}
