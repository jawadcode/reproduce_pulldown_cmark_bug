use pulldown_cmark::{html, Options, Parser};
use std::fs;

fn main() {
    let content_markdown = fs::read_to_string("post001.md").unwrap();
    let options = Options::all();
    let mut output = String::new();
    html::push_html(&mut output, Parser::new_ext(&content_markdown, options));
    println!("{output}");
}
