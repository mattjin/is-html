use regex::Regex;

pub fn is_html(str: &str) -> bool {
    let re = Regex::new(r"\s?<!doctype html>|(<html\b[^>]*>|<body\b[^>]*>|<x-[^>]+>)+").unwrap();
    re.is_match(str)
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
    }
}