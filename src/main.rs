use ureq;
use url;
use anyhow::Result;
use std::io;
use select::document::Document;
use select::predicate::{Name, Attr};

fn main() {
    println!("Stardew Valley Wiki CLI");
    loop {
        println!("What do you want to search?");
        let mut input = String::new();
        let stdin = io::stdin();
        let _ = stdin.read_line(&mut input).expect("");
        let sanitized_input = sanitize_input(input.as_str());
        let res = make_request(sanitized_input.as_str());
        match res {
            Ok(val) => scrape_select(val.as_str(), sanitized_input),
            Err(e) => println!("{e}\n\nItem doesn't exist/unavailable.\nTry matching the case *exactly.*\n"),
        }
    }
}
fn sanitize_input(input: &str) -> String {
    let trim = input.trim();
    let replace_with_underscore = trim.replace(" ", "_");
    replace_with_underscore
}
fn make_request(search_term: &str) -> Result<String> {
    let base_url = "https://stardewvalleywiki.com/";
    let parse_url = url::Url::parse(base_url).expect("Unable to parse url into a url::Url");
    let search_for = parse_url.join(search_term).expect("Unable to join search term with base_url");
    let agent = ureq::AgentBuilder::new().build();
    let get_resp = agent.get(search_for.as_str()).call()?;
    let page_string = get_resp.into_string()?;
    Ok(page_string)
}
fn scrape_select(page: &str, userinput: String) {
    println!("");
    println!("Search term: {userinput}");
    let document = Document::from(page);
    let _: Vec<()> = document
        .find(Name("table"))
        .map(|table| {
            let section = table.find(Attr("id", "infoboxsection"));
            let detail = table.find(Attr("id", "infoboxdetail"));
            for (s,d) in section.zip(detail) {
                let s_text = s.text();
                let d_text = d.text();
                let trimmed_section = s_text.trim();
                let trimmmed_detail = d_text.trim();
                let xp = trimmed_section.find("XP");
                let price = trimmed_section.find("Price");
                
                match xp {
                    Some(_) => continue,
                    None => (),
                }
                match price {
                    Some(_) => continue,
                    None => (),
                }
                match trimmmed_detail.find(".mw-parser-output") {
                    Some(_) => continue,
                    None => {
                        println!("{}: {}", s.text().trim(), d.text().trim());
                        }  
                    }
                }
        })
        .collect(); 
    println!("");
}