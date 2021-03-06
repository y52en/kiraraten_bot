mod document;
mod module;
mod tweet;

use chrono::prelude::Local;
use rustc_serialize::json;
// use std::collections::HashMap;

use regex::Regex;

static URL: &str = "http://www.kiraraten.jp/goods_list.html";
// static URL: &str = "http://127.0.0.1:5500/";

static HOMEPATH: &str = "/home/pi/kiraraten_bot/";
// static HOMEPATH: &str = "./";

static _savepath: &str = "list.json";
static error_log: &str = "error.txt";

// static debug: bool = false;
static debug: bool = true;

#[tokio::main]
async fn main() {
    println!("{}", URL);
    let a = module::http_get(URL).await.unwrap();
    let dom = document::parse_html(a);

    // let goods_num = document::scraping(&dom, ".lv1", document::Get::inner_html);
    // let goods_category = document::scraping(&dom, ".lv2", document::Get::inner_html);
    // let goods_name = document::scraping(&dom, ".lv3", document::Get::inner_html);
    // let goods_limit = document::scraping(&dom, ".lv4", document::Get::inner_html);
    let goods_stock = document::scraping(&dom, ".lv5>span", document::Get::class);

    // let goods_num = del_space_and_tag(goods_num);
    // let goods_category = del_space_and_tag(goods_category);
    // let goods_name = del_space_and_tag(goods_name);

    // let goods_num = str_as_num(goods_num);

    let savepath = format!("{}{}",HOMEPATH ,_savepath);

    let end = ||{
        let enc = json::encode(&goods_stock).unwrap();
        module::writefile(&savepath, enc).unwrap();
    };

    let savedata = module::readfile(&savepath).unwrap();
    if savedata == String::new() {
        end();
        return
    }
    let mut dec: Vec<String> = json::decode(&savedata).unwrap();

    if dec.len() == 0 {
        dec = goods_stock.clone()
    }

    if goods_stock.len() != dec.len() {
        let err = "length error";
        module::writefile(format!("{}{}",HOMEPATH , error_log), err).unwrap();
        panic!(err);
    }

    let comp = rotate_vec(vec![goods_stock.clone(), dec]);

    let mut diff = 0;
    for (i,x) in comp.iter().enumerate() {
        if x[0] != x[1] {
            diff += 1;
            println!("{} {} i:{}",x[0] , x[1],i)
        }
    }
    if diff > 0 {
        let now = Local::now().format("%m/%d %H:%M");
        let tweet_str = format!("??????????????????????????????????????????\n??????????????????:{}\n?????????????????????:{}", diff,now);
        println!("{}", &tweet_str);
        if !debug {
            tweet::tweet(&tweet_str).await;
        }
    }
    
    end();
}



fn p<S: std::fmt::Debug>(s: S) {
    println!("{:#?}", s);
}

fn del_space_and_tag(v: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"\n|\t|</?strong>").unwrap();
    v.iter()
        .map(|string| re.replace_all(string, "").to_string())
        .collect::<Vec<String>>()
}

#[allow(dead_code)]
fn str_as_num(v: Vec<String>) -> Vec<i32> {
    let re = Regex::new(r"--").unwrap();
    let v = v
        .iter()
        .map(|string| re.replace_all(string, "-1").to_string())
        .collect::<Vec<String>>();

    v.iter()
        .map(|string| string.parse().unwrap())
        .collect::<Vec<i32>>()
}

// [
//     [0,0,0],
//     [0,4,5],
// ]
//      ???
//  [
//     [0,0],
//     [0,4],
//     [0,5]
// ]
//
//

fn rotate_vec<S: Clone>(v: Vec<Vec<S>>) -> Vec<Vec<S>> {
    let mut output = Vec::new();
    for x in 0..v.get(0).unwrap().len() {
        output.push(Vec::new());
        for y in 0..v.len() {
            output[x].push(v[y][x].clone());
        }
    }
    output
}
