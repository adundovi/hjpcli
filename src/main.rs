use clap::{Arg, App};

fn main() {
    let matches = App::new("hjpcli")
        .version("0.1")
        .author("Andrej Dundović <andrej@dundovic.com.hr>")
        .about("A command line interface for the Croatian Language Portal (HJP)")
        .arg(Arg::new("WORD")
            .about("Sets the search word")
            .required(true)
            .index(1))
        .get_matches();

    if let Some(word) = matches.value_of("WORD") {
        hjpcli::search(word).unwrap();
    }
}
