extern crate clap;
extern crate crypto;
extern crate curl;
extern crate rand;

use std::io::{stdout, Write};

use clap::{Arg, App};
use curl::easy::Easy;
use crypto::digest::Digest;
use crypto::md5::Md5;

const APP_KEY: &'static str = env!("YOUDAO_KEY");
const APP_SECRET: &'static str = env!("YOUDAO_SECRET");
const BASE_URL: &'static str = "https://openapi.youdao.com/api";
const DEFAULT_FROM: &'static str = "auto";
const DEFAULT_TO: &'static str = "auto";

fn main() {
    let matches = App::new("youdao-cli")
        .version("0.1.0")
        .author("xjw0914 <contactxjw@gmail.com>")
        .about("A Command Line Tool For Youdao Translation")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input word to translate")
            .required(true)
            .index(1))
        .arg(Arg::with_name("from")
            .short("f")
            .long("from")
            .value_name("FROM")
            .help("Sets the source language")
            .default_value(DEFAULT_FROM)
            .takes_value(true))
        .arg(Arg::with_name("to")
            .short("t")
            .long("to")
            .value_name("TO")
            .help("Sets the target language")
            .default_value(DEFAULT_TO)
            .takes_value(true))
        .get_matches();

    let mut easy = Easy::new();
    let q = matches.value_of("INPUT").unwrap();
    let from = matches.value_of("FROM").unwrap();
    let to = matches.value_of("TO").unwrap();

    easy.url(&build_url(q, from, to)).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
}

fn build_url(q: &str, from: &str, to: &str) -> String {
    let salt = rand::random::<u8>();
    let mut sh = Md5::new();
    let input = format!("{}{}{}{}", APP_KEY, q, salt, APP_SECRET);
    sh.input_str(&input);
    let sign = sh.result_str().to_uppercase();

    format!(
        "{}?from={}&to={}&appKey={}&salt={}&q={}&sign={}",
        BASE_URL,
        from,
        to,
        APP_KEY,
        salt,
        q,
        &sign
    )
}
