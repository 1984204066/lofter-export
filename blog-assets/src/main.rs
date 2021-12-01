use nipper::Document;
use std::{fs::*, io::Write, str::Chars};
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
		"-" + format!("{:0>2}", caps.name("m").unwrap().as_str()).as_str() +
		"-" + format!("{:0>2}", caps.name("d").unwrap().as_str()).as_str();

	    let article  = document.select("div.txtcont");
	    let title = decode(&title).unwrap();
	    let re = Regex::new(r"[《 》【】（）？：。，()]").unwrap();
	    let title = re.replace_all(&title, "");
	    let title = title.replace("&nbsp;", "");
	    // println!("{}\n{}\n{}", title, date, article.html());
	    // println!("{}\n{}", title, date);

	    let ofile = "".to_string() + &date + "-" +  &title;
	    println!("{}", &ofile);
	    let html = article.html();
	    {
		let mut art = std::fs::File::create("./part-html/".to_string() + &ofile).unwrap();
		art.write_all(html.as_bytes());
		// let html = html.chars();
		let tag=  get_tag(&html.chars());
		let prefix = format!("---
layout: post
title:  \"{}\"
date:   {}
tags:
      - {}
---\n\n#{}\n\n", title, date, tag, title);
		let mut top = std::fs::File::create("./markdown/".to_string() + &ofile).unwrap();

		top.write_all(prefix.as_bytes());
	    }

	}
}

fn get_tag(html: &Chars) -> String {
    let re = Regex::new(r"['佛' '僧' '禅']").unwrap();

    // if html.find(|c| {
    // 	let i = *c as u32;
    // 	let keys = ['佛', '僧', '禅'];
    // 	keys.iter().find_map(|s| {
    // 	    let b=  *s as u32;
    // 	    i == b}).is_some()
    // 	// let b = '佛' as u32;
    // 	// i == b
    // }) {
    // 	tag = "佛法";
    // } else if html.find("医") {
    // 	tag = "中医";
    // } else if html.find("") {
    // 	tag = "it";
    // } else {
    // 	tag = "随笔";
    // }
    "随笔".to_string()
}
