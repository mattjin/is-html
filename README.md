# is-html

> Check if a string is HTML

You should not use this for any kind of validation, sanitation, or XSS checks.

## Install

```
$ cargo install is-html
```
## Usage

```rust
use is_html::is_html;

is_html("<p>I am HTML</p>");
//=> true

is_html("<!doctype><html><body><h1>I ❤ rust</h1></body></html>");
//=> true

is_html("<cake>I am XML</cake>");
//=> false

is_html(">+++++++>++++++++++>+++>+<<<<-");
//=> false
```

Note: It does not detect deprecated HTML tags.

## Reference
[is-html](https://github.com/sindresorhus/is-html)