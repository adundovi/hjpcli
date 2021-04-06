use clap::{Arg, App};

#[derive(Clone, Copy)]
enum NeedleType {
    Id,
    Name,
    Class,
    Attribute
}

fn find_in_tree<'a>(nodes: &'a Vec<html_parser::Node>, needle: &str, n_type: NeedleType) -> Option<&'a html_parser::Element> {
    let mut m: Option<&html_parser::Element> = None;
    for n in nodes.iter() {
        m = match n {
            html_parser::Node::Element(e) =>
                match n_type {
                    NeedleType::Id => if e.id == Some(needle.to_string()) { Some(e) } else { find_in_tree(&e.children, needle, n_type) },
                    NeedleType::Name => if e.name == needle.to_string() { Some(e) } else { find_in_tree(&e.children, needle, n_type) },
                    NeedleType::Class => if e.classes.contains(&needle.to_string()) { Some(e) } else { find_in_tree(&e.children, needle, n_type) },
                    NeedleType::Attribute => if e.attributes.get(&needle.to_string()).is_some() { Some(e) } else { find_in_tree(&e.children, needle, n_type) },
                }
            _ => None,
        };
        match m {
            Some(_) => { break },
            None => {}
        }
    }
    m
}

fn extract_text(node: &html_parser::Node) -> String {
    let mut s: String = String::new();
    match node {
        html_parser::Node::Text(text) => s.push_str(&text),
        html_parser::Node::Element(e) => {
            for c in e.children.iter() {
                s.push_str(&" ");
                s.push_str(&extract_text(&c));
            }
        },
        _ => s.push_str(&" "),
    };
    s
}

fn print_node(node: &html_parser::Node) {
    println!("{:#?}",
                html_escape::decode_html_entities(
                    &extract_text(node)
        )
    )
}

#[tokio::main]
async fn search(word: &str) -> Result<(), Box<dyn std::error::Error>> {
        let params = [("word", word), ("search", &"Pretraga".to_string())];

        let url = url::Url::parse("http://hjp.znanje.hr/index.php?show=search")?;
        let client = reqwest::Client::new();
        let resp = client.post(url)
            .form(&params)
            .send()
            .await?
            .text()
            .await?;
    
        let p = html_parser::Dom::parse(&resp)?;
        let e = find_in_tree(&p.children, &"definicija", NeedleType::Id);
        match e {
            Some(inner) => print_node(&html_parser::Node::Element(inner.clone())),
            None => println!("Nema tražene riječi.")
        }
    Ok(())
}

fn main() {
    let matches = App::new("HJPcli")
        .version("0.1")
        .author("Andrej Dundović <andrej@dundovic.com.hr>")
        .about("Command line interface for HJP")
        .arg(Arg::new("WORD")
            .about("Sets the search word")
            .required(true)
            .index(1))
        .get_matches();

    if let Some(word) = matches.value_of("WORD") {
        search(word).unwrap();
    }
}
