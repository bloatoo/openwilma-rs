use scraper::{Selector, Html, ElementRef};

pub fn filter_line<'a, I>(pattern: &str, lines: &mut I) -> Option<&'a str>
where
    I: Iterator<Item = &'a str>
{
    lines.find(|l| l.contains(pattern))
}

pub fn parse_name<'a, I>(document: &mut I) -> String
where
    I: Iterator<Item = &'a str>
{
    let line = filter_line("class=\"teacher\"", document).unwrap();

    let fragment = Html::parse_fragment(line);
    let selector = Selector::parse("span").unwrap();
    let element = fragment.select(&selector).next().unwrap();
    let child = element.children().next().unwrap();

    let text = child
        .value()
        .as_text()
        .unwrap()
        .to_string();

    text
} 

pub fn parse_school<'a, I>(document: &mut I) -> String
where
    I: Iterator<Item = &'a str>
{
    let line = filter_line("class=\"school\"", document).unwrap();

    let fragment = Html::parse_fragment(line);
    let selector = Selector::parse("span").unwrap();
    let element = fragment.select(&selector).next().unwrap();
    let child = element.children().next().unwrap();

    let text = child
        .value()
        .as_text()
        .unwrap()
        .to_string();

    text
} 
