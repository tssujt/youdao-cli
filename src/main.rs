extern crate clap;
extern crate crypto;
extern crate curl;
extern crate rand;

use std::io::{stdout, Write};

use clap::{Arg, App};
use curl::easy::Easy;
use crypto::md5::Md5;
use crypto::digest::Digest;

const APP_KEY: &'static str = env!("YOUDAO_KEY");
const APP_SECRET: &'static str = env!("YOUDAO_SECRET");
const BASE_URL: &'static str = "https://openapi.youdao.com/api";
const FROM: &'static str = "EN";
const TO: &'static str = "zh-CHS";

fn main() {
    let matches = App::new("youdao-cli")
        .version("0.1.0")
        .author("xjw0914 <contactxjw@gmail.com>")
        .about("A Command Line Tool For Youdao Translation")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input word to translate")
            .required(true)
            .index(1))
        .get_matches();

    let mut easy = Easy::new();
    let q = matches.value_of("INPUT").unwrap();
    easy.url(&build_url(q)).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
}

fn build_url(q: &str) -> String {
    let salt = rand::random::<u8>();
    let mut sh = Md5::new();
    let input = format!("{}{}{}{}", APP_KEY, q, salt, APP_SECRET);
    sh.input_str(&input);
    let sign = sh.result_str().to_uppercase();

    format!(
        "{}?from={}&to={}&appKey={}&salt={}&q={}&sign={}",
        BASE_URL,
        FROM,
        TO,
        APP_KEY,
        salt,
        q,
        &sign
    )
}
