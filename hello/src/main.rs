use std::fs;

fn main() {
    // println!("Hello, world!");
    let url = "https://www.baidu.com/";
    let output = "rust.md";

    println!("Fethcing url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown ...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved to {}", output)
}
