use scraper::{Selector, Html};
use crate::news_article::NewsArticle;

pub fn filter_line<'a, I>(pattern: &str, lines: &mut I) -> Option<&'a str>
    where I: Iterator<Item = &'a str>
{
    lines.find(|l| l.contains(pattern))
}

pub fn filter_fragment<'a, I>(
    start_pattern: &str,
    end_pattern: &str,
    mut lines: I
) -> Option<Vec<&'a str>>
    where I: Iterator<Item = &'a str> + Clone
{
    let mut result = vec![];

    let start_line = filter_line(start_pattern, &mut lines.clone()).unwrap();
    let end_line = filter_line(end_pattern, &mut lines.clone()).unwrap();

    let mut in_fragment = false;

    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };

        //println!("line=k{}k\nstart_line=\n{}\nend_line={}\nin_fragment={}", line, start_line, end_line, in_fragment);
        
        println!("start{}end", line);

        if line == start_line && in_fragment == false {
            in_fragment = true;
        }

        match in_fragment {
            true => {
                if line == end_line {
                    break;
                }

                result.push(line);
            }
            false => continue,
        }
    }

    result.retain(|line| !line.is_empty());
    Some(result)
}

pub fn parse_name<'a, I>(document: &mut I) -> String
    where I: Iterator<Item = &'a str>
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

pub fn parse_identity<'a, I>(document: &mut I) -> String
    where I: Iterator<Item = &'a str>
{
    let line = filter_line("text-style-link", document).unwrap();

    let fragment = Html::parse_fragment(line);
    let selector = Selector::parse("a").unwrap();
    let stuff = fragment.select(&selector).next().unwrap();

    let mut identity = stuff.value().attr("href").unwrap().to_string();
    identity.remove(0);
    identity
}

pub fn parse_news_list<'a, I>(document: I) -> Vec<NewsArticle> 
    where I: Iterator<Item = &'a str> + Clone
{
    /*let news_html = filter_fragment(
        "class=\"panel tab-content\"",
        "class=\"panel hidden-md-up\"",
        document
    ).unwrap().join("\n");

    println!("{:#?}", html);

    println!("{}", news_html);
    vec![]*/
    todo!()
}

pub fn parse_news_article<'a, I>(document: &mut I) -> NewsArticle
    where I: Iterator<Item = &'a str>
{
    todo!()
}

pub fn parse_formkey<'a, I>(document: &mut I) -> String
    where I: Iterator<Item = &'a str>
{
    let line = filter_line("formkey", document).unwrap();

    let fragment = Html::parse_fragment(line);
    let selector = Selector::parse("input").unwrap();
    let element = fragment.select(&selector).next().unwrap();

    element.value().attr("value").unwrap().into()
}

pub fn parse_school<'a, I>(document: &mut I) -> String
    where I: Iterator<Item = &'a str>
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
