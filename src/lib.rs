#[derive(Clone, Copy)]
enum NeedleType {
    Id,
    Name,
    Class,
    Attribute
}

fn find_in_tree<'a>(nodes: &'a Vec<html_parser::Node>, needle: &str, n_type: NeedleType) -> Vec<&'a html_parser::Element> {
    let mut elements: Vec<&html_parser::Element> = vec![];
    for n in nodes.iter() {
        match n {
            html_parser::Node::Element(e) =>
                match n_type {
                    NeedleType::Id =>
                        if e.id == Some(needle.to_string()) { elements.push(e) }
                        else { elements.append(&mut find_in_tree(&e.children, needle, n_type)) },
                    NeedleType::Name =>
                        if e.name == needle.to_string() { elements.push(e) }
                        else { elements.append(&mut find_in_tree(&e.children, needle, n_type)) },
                    NeedleType::Class =>
                        if e.classes.contains(&needle.to_string()) { elements.push(e) }
                        else { elements.append(&mut find_in_tree(&e.children, needle, n_type)) },
                    NeedleType::Attribute =>
                        if e.attributes.get(&needle.to_string()).is_some() { elements.push(e) }
                        else { elements.append(&mut find_in_tree(&e.children, needle, n_type)) },
                }
            _ => { },
        };
    }
    elements
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
    println!("{}",
        html_escape::decode_html_entities(
            &extract_text(node)
        ).trim()
    )
}

fn print_word(el: &html_parser::Element) {
    // multiple definitions
    for row in find_in_tree(
                    &vec!(html_parser::Node::Element(el.clone())),
                    &"td", NeedleType::Name) {
        print_node(&html_parser::Node::Element(row.clone()));
    }
}

#[tokio::main]
pub async fn search(word: &str) -> Result<(), Box<dyn std::error::Error>> {
        let params = [("word", word), ("search", &"Pretraga".to_string())];

        let url = url::Url::parse("https://hjp.znanje.hr/index.php?show=search")?;
        let client = reqwest::Client::new();
        let resp = client.post(url)
            .form(&params)
            .send()
            .await?
            .text()
            .await?;
   
        let p = html_parser::Dom::parse(&resp)?;
        let elements = find_in_tree(&p.children, &"definicija", NeedleType::Id);
        if elements.len() == 0 {    
            println!("Nema tražene riječi.");
        } else {
            for r in elements.into_iter() {
                print_word(r);
            }
        }
    Ok(())
}
