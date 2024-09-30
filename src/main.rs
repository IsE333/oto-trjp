extern crate encoding_rs;
extern crate encoding_rs_io;

use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let f0 = File::open("glottalstopletter.txt").unwrap();
    let reader0 = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(f0),
    );

    let glottal_letter = reader0.lines().next().unwrap().unwrap();

    let f1 = File::open("dict.txt").unwrap();
    let mut dictionary: Vec<(String, String)> = Vec::new();
    let reader1 = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(f1),
    );

    for line in reader1.lines() {
        let l: Vec<String> = line.unwrap().split(" ").map(|s| s.to_string()).collect();
        let (l1, l2) = (l[1].clone(), l[0].clone());
        dictionary.push((l1, l2));
    }

    let f2 = File::open("oto.ini").unwrap();
    print!("{:?}", f2);

    let reader2 = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(f2),
    );

    let mut new_aliases: Vec<Alias> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    for line in reader2.lines() {
        let line = line.unwrap();
        lines.push(line.clone() + "\n");
        let a = check(line, dictionary.as_slice(), glottal_letter.clone());
        if a.last != "" {
            new_aliases.push(a);
        }
    }

    let mut writer = BufWriter::new(File::create("oto_new.ini").unwrap());

    for line in lines {
        writer.write(line.as_bytes()).unwrap();
    }
    for alias in new_aliases {
        let new_line = change_alias(alias);
        println!("{}", new_line);
        writer.write(new_line.as_bytes()).unwrap();
    }
}

fn change_alias(alias: Alias) -> String {
    alias.line.replace(
        &("=".to_string() + &alias.alias),
        &("=".to_string() + &alias.last),
    ) + "\n"
}

fn check(line: String, dict: &[(String, String)], glottal_letter: String) -> Alias {
    let mut alias = get_alias(line);
    let (mut found, mut value) = (false, "".to_string());
    if alias.second == " " {
        (found, value) = find_in_dict(alias.first.clone(), dict);
        if found {
            alias.last = value;
        }
    } else if alias.first == "-"
        || alias.first == "a"
        || alias.first == "e"
        || alias.first == "o"
        || alias.first == "u"
        || alias.first == "i"
    {
        (found, value) = find_in_dict(alias.second.clone(), dict);
        if found {
            alias.last = alias.first.clone() + " " + &value;
        }
    } else if alias.first == "?" {
        (found, value) = find_in_dict(alias.second.clone(), dict);
        if found {
            alias.last = glottal_letter + &value;
        }
    } else if alias.first == "N" {
        (found, value) = find_in_dict(alias.second.clone(), dict);
        if found {
            alias.last = "n ".to_owned() + &value;
        }
    }
    alias
}

fn find_in_dict(alias: String, dict: &[(String, String)]) -> (bool, String) {
    for line in dict {
        if alias == line.0 {
            return (true, line.1.to_string());
        }
    }
    (false, "".to_string())
}

fn get_alias(line: String) -> Alias {
    let alias_str = line
        .split("=")
        .last()
        .unwrap()
        .split(",")
        .nth(0)
        .unwrap()
        .to_string(); //alias in the line

    let alias = Alias::new(alias_str, line);

    alias
}

struct Alias {
    alias: String,
    first: String,
    second: String,
    size: i32,
    last: String, //-aeoui kana
    line: String,
}
impl Alias {
    fn new(alias: String, line: String) -> Alias {
        let mut parts = alias.split(" ");
        let first = parts.next().unwrap_or(" ").to_string();
        let second = parts.next().unwrap_or(" ").to_string();

        Alias {
            alias: alias.clone(),
            first: first,
            second: second,
            size: parts.clone().count() as i32,
            last: "".to_string(),
            line: line,
        }
    }
}
