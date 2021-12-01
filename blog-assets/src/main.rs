use nipper::Document;
use std::{fs::*, io::Write};
use walkdir::WalkDir;
use regex::Regex;
use urlencoding::decode;

fn main() {
    for entry in WalkDir::new("./html-file/").into_iter()
	.filter_map(|e|
		    if !e.as_ref().unwrap().path().is_file() {
			None
		    } else {
			e.ok()
		    }) {
	    // let entry = entry.unwrap();
	    let html = read_to_string(entry.path()).unwrap();
	    let document = Document::from(&html);
	    let title = document.select("h2.ttl").text().to_string();
	    let date  = document.select("a.date").text().to_string();
	    let re = Regex::new(r"(?P<m>\d+)\.(?P<d>\d+)\.(?P<year>\d{4})").unwrap();
	    let caps = re.captures(&date).unwrap();
	    let date= caps.name("year").unwrap().as_str().to_string() +
		"-" + caps.name("m").unwrap().as_str() +
		"-" + caps.name("d").unwrap().as_str();

	    let article  = document.select("div.txtcont");
	    let title = decode(&title).unwrap();
	    let re = Regex::new(r"[《 》【】（）？：。，()]").unwrap();
	    let title = re.replace_all(&title, "");
	    let title = title.replace("&nbsp;", "");
	    // println!("{}\n{}\n{}", title, date, article.html());
	    println!("{}\n{}", title, date);

	    let ofile = "./part-html/".to_string() + &date + "-" +  &title;
	    println!("{}", &ofile);
	    {
		let mut ofile = std::fs::File::create(ofile).unwrap();
		ofile.write_all(article.html().as_bytes());
	    }

	}
}
