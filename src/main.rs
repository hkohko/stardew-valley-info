use ureq;
use url;
use anyhow::Result;
use std::io;
use scraper::{Html, Selector, element_ref::Text, Element};

fn main() {
    println!("What do you want to search?");
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input).expect("bla");
    let sanitized_input = sanitize_input(input.as_str());
    let res = make_request(sanitized_input.as_str());
    match res {
        Ok(val) => scrape(val.as_str()),
        Err(e) => println!("{e}"),
    }
}
fn sanitize_input(input: &str) -> String {
    let trim = input.trim();
    let replace_with_underscore = trim.replace(" ", "_");
    replace_with_underscore
}
fn make_request(search_term: &str) -> Result<String>{
    let base_url = "https://stardewvalleywiki.com/";
    let parse_url = url::Url::parse(base_url).expect("Unable to parse url into a url::Url");
    let search_for = parse_url.join(search_term).expect("Unable to join search term with base_url");
    let agent = ureq::AgentBuilder::new().build();
    let get_resp = agent.get(search_for.as_str()).call()?;
    let page_string = get_resp.into_string()?;
    Ok(page_string)
}
fn scrape(page: &str) {
    let doc = Html::parse_document(page);
    let table = Selector::parse("table").expect("");
    let mut t: Vec<String> = doc.select(&table)
        .filter(|div| div.attr("id") == Some("infoboxtable"))
        .map(|div| div.html())
        .collect();
        
    let tbody = t.pop().unwrap();
    let tr_select = Selector::parse("tr").expect("");
    let tbody_doc = Html::parse_fragment(tbody.as_str());
    let td_vec: Vec<String> = tbody_doc.select(&tr_select)
        .map(|tr| tr.inner_html().trim().to_string())
        .collect();
        
    let td_select = Selector::parse("td").unwrap();
    
    for tds in td_vec.iter() {
        let doc = Html::parse_fragment(tds.as_str());
        let tree = doc.tree;
        let a = tree.nodes();
        for data in a {
            let value = data.value();
            let elements = value.as_text();
            if let Some(text) = elements {
                let txt = text.trim();
                println!("{txt}");
            }

        }
    }
}