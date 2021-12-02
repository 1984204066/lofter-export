use nipper::Document;
use regex::Regex;
use std::{fs::*, io::Write, str::Chars};
use urlencoding::decode;
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("./html-file/").into_iter().filter_map(|e| {
        if !e.as_ref().unwrap().path().is_file() {
            None
        } else {
            e.ok()
        }
    }) {
        // let entry = entry.unwrap();
        let html = read_to_string(entry.path()).unwrap();
        let document = Document::from(&html);
        let title = document.select("h2.ttl").text().to_string();
        let date = document.select("a.date").text().to_string();
        let re = Regex::new(r"(?P<m>\d+)\.(?P<d>\d+)\.(?P<year>\d{4})").unwrap();
        let caps = re.captures(&date).unwrap();
        let date = caps.name("year").unwrap().as_str().to_string()
            + "-"
            + format!("{:0>2}", caps.name("m").unwrap().as_str()).as_str()
            + "-"
            + format!("{:0>2}", caps.name("d").unwrap().as_str()).as_str();

        let article = document.select("div.txtcont");
        let title = decode(&title).unwrap();
        let re = Regex::new(r"[《 》【】（）？：。，()]").unwrap();
        let title = re.replace_all(&title, "");
        let title = title.replace("&nbsp;", "");
        // println!("{}\n{}\n{}", title, date, article.html());
        // println!("{}\n{}", title, date);

        let ofile = "".to_string() + &date + "-" + &title;
        let html = article.html();
        {
            let mut art = std::fs::File::create("./part-html/".to_string() + &ofile).unwrap();
            art.write_all(html.as_bytes());
            let txt = article.text().to_string();
            let tag = assign_tag(&title, &txt);
            println!("{}  --  {}", &ofile, &tag);
            let prefix = format!(
                "---
layout: post
title:  \"{}\"
date:   {}
tags:
      - {}
---\n\n",
                title, date, tag
            );
            let mut top = std::fs::File::create("./markdown/".to_string() + &ofile).unwrap();

            top.write_all(prefix.as_bytes());
        }
    }
}

fn assign_tag(title: &str, txt: &str) -> String {
    let (may, tag) = whatsit(title);
    if may == Similor::Strong {
        return tag;
    }
    if may == Similor::None {
        let (may, tag) = whatsit(txt);
        return tag;
    }
    return tag;
}

#[derive(PartialEq)]
enum Similor {
    None = 0,
    Maybe = 3,
    Strong = 6,
}

fn whatsit(txt: &str) -> (Similor, String) {
    let tag = r#"佛学"#.to_string();
    let strong1 = is_佛学(txt);
    if strong1 == Similor::Strong {
        return (strong1, tag);
    }

    let tag = r#"中医"#.to_string();
    let strong2 = is_中医(txt);
    if strong2 == Similor::Strong {
        return (strong2, tag);
    }

    let tag = "it".to_string();
    let strong3 = is_it(txt);
    if strong3 == Similor::Strong {
        return (strong3, tag);
    }
    // strong && return tag;
    if strong1 == Similor::None && strong2 == Similor::None && strong3 == Similor::None {
        return (Similor::None, "随笔".to_string());
    }
    get_one_maybe(strong1, strong2, strong3)
}

fn get_one_maybe(may佛学: Similor, may中医: Similor, mayit: Similor) -> (Similor, String) {
    let m1 = may佛学 as u32;
    let m2 = may中医 as u32;
    let m3 = mayit as u32;
    if m1 + m2 + m3 > Similor::Maybe as u32 {
        return (Similor::None, "随笔".to_string());
    }
    if m1 == Similor::Maybe as u32 {
        return (Similor::Maybe, "佛学".to_string());
    }
    if m2 == Similor::Maybe as u32 {
        return (Similor::Maybe, "中医".to_string());
    }
    if m3 == Similor::Maybe as u32 {
        return (Similor::Maybe, "it".to_string());
    }
    return (Similor::None, "随笔".to_string());
}

fn is_佛学(txt: &str) -> Similor {
    let re = Regex::new(r"佛陀|禅修|阿含|如是我闻|八正|无我|无为法|巴利|南传|日月明行").unwrap();
    if re.is_match(txt) {
        return Similor::Strong;
    }
    let re = Regex::new(r"[佛陀僧禅咒]").unwrap();
    if re.is_match(txt) {
        return Similor::Maybe;
    }
    Similor::None
}

fn is_中医(txt: &str) -> Similor {
    let re = Regex::new(r"中医|黄帝|内经|柴胡|桂枝|阴阳|病症|更年期|月经").unwrap();
    if re.is_match(txt) {
        return Similor::Strong;
    }
    let re = Regex::new(r"[病症医药汤针炙帝丸]").unwrap();
    if re.is_match(txt) {
        return Similor::Maybe;
    }
    Similor::None
}

fn is_it(txt: &str) -> Similor {
    let re =Regex::new(r"(?i)函数|软件|emacs|lisp|程序|计算机|boost|android|kernel|url|脚本|package|组合|系统|内存|绑定|下载|网盘|安装|cpp|c\+\+|gcc|client|server|sqlite|memory|开源|gnome|kde|rust").unwrap();
    if re.is_match(txt) {
        return Similor::Strong;
    }
    Similor::None
}
