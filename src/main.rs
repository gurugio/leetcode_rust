use cli_table::{
    format::{Border, Justify, Separator},
    print_stdout, Table, WithTitle,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::{cmp, fmt::Display};
//import std::fmt::Display;

#[macro_use]
extern crate serde_derive;

#[derive(Table, Serialize)]
struct DcmLinksViewer {
    #[table(title = "VM")]
    uuid: String,
    #[table(title = "Block Devs", display_fn = "print_net_links")]
    block_links: Vec<String>,
    #[table(title = "Net Devs", display_fn = "print_net_links")]
    net_links: Vec<String>,
}

/*
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
} */

fn print_net_links(value: &Vec<String>) -> impl Display {
    format!("{}", value.join("\n"))
}

fn main() {
    let mut links = vec![DcmLinksViewer {
        uuid: String::new(),
        block_links: Vec::new(),
        net_links: Vec::new(),
    }];

    links[0].uuid.push_str("1234-1234");
    links[0].block_links.push("/a/b/c/".to_string());
    links[0].block_links.push("/1/2/3/4/".to_string());
    links[0].net_links.push("abde-ffff".to_string());
    links[0].net_links.push("2222-ffff".to_string());

    println!("{}", serde_json::to_string(&links).unwrap());
    print_stdout(
        links
            .with_title()
            .color_choice(cli_table::ColorChoice::Never)
            .border(Border::builder().build())
            .separator(Separator::builder().title(Some(Default::default())).build()),
    )
    .unwrap();
}
