use regex::Regex;

const HTML_TAGS: [&str; 117] = [
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "math",
    "menu",
    "menuitem",
    "meta",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "pre",
    "progress",
    "q",
    "rb",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "slot",
    "small",
    "source",
    "span",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "svg",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
];

pub fn is_html(str: &str) -> bool {
    let re = Regex::new(r"\s?<!doctype html>|(<html\b[^>]*>|<body\b[^>]*>|<x-[^>]+>)+").unwrap();
    let re_full_str = HTML_TAGS.map(|x| format!("<{}\\b[^>]*>", x)).join("|");
    let re_full = Regex::new(re_full_str.as_str()).unwrap();
    re.is_match(str) || re_full.is_match(str)
}

#[cfg(test)]
mod tests {
    use crate::is_html;

    #[test]
    fn it_works() {
        assert!(is_html("<!doctype html>"));
        assert!(is_html("\n\n<!doctype html><html>"));
        assert!(is_html("<html>"));
        assert!(is_html("<html></html>"));
        assert!(is_html("<html lang='en'></html>"));
        assert!(is_html("<html><body></html>"));
        assert!(is_html("<html><body class='no-js'></html>"));

        assert!(is_html("<p>foo</p>"));
        assert!(is_html("<a href='#'>foo</a>"));
    }

    #[test]
    fn it_not_works() {
        assert!(!is_html("<cake>foo</cake>"));
        assert!(!is_html("<any>rocks</any>"));
        assert!(!is_html("<cake>foo</cake>"));
        assert!(!is_html("<bodyx>not</bodyx>"));
    }
}
