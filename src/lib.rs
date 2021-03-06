// Todo:
// Labeling
// Random picking
// Augment by size


use indicatif::ProgressBar;


use std::io::prelude::*;

use std::fs::OpenOptions;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct OriData {
    pub text    : String,
    pub label   : String,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyperOriData {
    pub id      : i32,
    pub label   : String,
    pub text    : String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AugData {
    pub text    : String,
    pub label   : String,
    pub metadata: HashMap<String, String>,
    pub score   : String,
}

pub fn read_ori_jsonl(path: String) -> Result<Vec<OriData>, std::io::Error> {
    let mut v = Vec::<OriData>::new();
    match File::open(path){
	Ok(f) => {
	    for line in BufReader::new(f).lines() {
		let line = line.expect("couldn't get line");
		let e: OriData = serde_json::from_str(&line)?;
		v.push(e);
	    }
	    Ok(v)	    
	},
	Err(err) => Err(err)
    }
}

pub fn read_hyper_ori_jsonl(path: String) -> Result<Vec<HyperOriData>, std::io::Error> {
    let mut v = Vec::<HyperOriData>::new();
    match File::open(path){
	Ok(f) => {
	    for line in BufReader::new(f).lines() {
		let line = line.expect("couldn't get line");
		let e: HyperOriData = serde_json::from_str(&line)?;
		v.push(e);
	    }
	    Ok(v)	    
	},
	Err(err) => Err(err)
    }
}

pub fn read_aug_jsonl(path: String) -> Result<Vec<AugData>, std::io::Error> {
    let mut v = Vec::<AugData>::new();

    match File::open(path) {
	Ok(f) => {
	    for line in BufReader::new(f).lines() {
		let line = line.expect("couldn't get line");
		let e: AugData = serde_json::from_str(&line)?;
		v.push(e);
	    }
	    Ok(v)
	},
	Err(err) => Err(err),
    }
}

pub fn write_aug_jsonl(v: Vec<AugData>, path: &str, num: i32) -> std::io::Result<()> {
    println!("here");
    let _ = fs::create_dir_all(path);
    let filename = format!("{:0>5}{}", num, ".jsonl");

    let lab_file = [path,&*filename].join("");

    //println!("{:?}", lab_file);
    let mut output = File::create(lab_file)?;
    for j in v {
    	let line = serde_json::to_string(&j)?;
    	write!(output, "{}", [line,"\n".to_string()].join(""))?;
    }
    Ok(())
}



pub fn label() {
    // let ori_file = "/home/heeh/Projects/rusty-wrangler/data/original/citation_intent/train.jsonl".to_string();
    // let aug_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large/citation_intent/";
    // let lab_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large_lab/citation_intent/";    
    // let ori_file = "/home/heeh/Projects/rusty-wrangler/data/original/hyperpartisan_news/train.jsonl".to_string();
    // let aug_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large/hyperpartisan_news/";
    // let lab_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large_lab/hyperpartisan_news/";
    let ori_file = "/home/heeh/Projects/rusty-wrangler/data/original/hyperpartisan_news/train.jsonl".to_string();
    let aug_path = "/home/heeh/Projects/rusty-wrangler/data/aug_hard2/hyperpartisan_news/";
    let lab_path = "/home/heeh/Projects/rusty-wrangler/data/aug_hard2_lab/hyperpartisan_news/";        

    println!("= Loading the data into OriData structs");
    // let ori_data = read_ori_jsonl(ori_file);
    // let ori = ori_data.unwrap();

    let hyper_ori_data = read_hyper_ori_jsonl(ori_file);
    let ori = hyper_ori_data.unwrap();

    
    println!("= Loading the data into AugData structs");


    for i in 0..ori.len() {
    	let filename = format!("{:0>5}{}", i, ".jsonl");
    	let aug_file = [aug_path,&*filename].join("");
	println!("{:?}", aug_file);
        let aug_jsonl = read_aug_jsonl(aug_file);
    	let mut aug_jsonl = match aug_jsonl {
    	    Ok(v)    => v,
    	    Err(err) => {
    		continue;
    	    }
    	};
	for j in 0..aug_jsonl.len() {
	    aug_jsonl[j].label = ori[i].label.clone();
	}
	if let Err(e) = write_aug_jsonl(aug_jsonl, lab_path, i as i32) {
	    println!("{:?}", e);
	}
    }
}


pub fn collect(topk: usize) {
    // let ori_file = "/home/heeh/Projects/rusty-wrangler/data/original/citation_intent/train.jsonl".to_string();
    // let lab_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large_lab/citation_intent/";
    //let top_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large_top/citation_intent/";
    // let top_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large_hard/citation_intent/";

    // let ori_file = "/home/heeh/Projects/rusty-wrangler/data/original/hyperpartisan_news/train.jsonl".to_string();
    // let lab_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large_lab/hyperpartisan_news/";
    // // let top_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large_top/hyperpartisan_news/";
    // let top_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large_hard/hyperpartisan_news/";

    let ori_file = "/home/heeh/Projects/rusty-wrangler/data/original/hyperpartisan_news/train.jsonl".to_string();
    let lab_path = "/home/heeh/Projects/rusty-wrangler/data/aug_hard2_lab/hyperpartisan_news/";
    // let top_path = "/home/heeh/Projects/rusty-wrangler/data/aug_large_top/hyperpartisan_news/";
    let top_path = "/home/heeh/Projects/rusty-wrangler/data/aug_hard2_top/hyperpartisan_news/";
    

    
    // let ori_data = read_ori_jsonl(ori_file);
    // let ori = ori_data.unwrap();
    let hyper_ori_data = read_hyper_ori_jsonl(ori_file);
    let ori = hyper_ori_data.unwrap();



    
    let _ = fs::create_dir_all(top_path);
    let filename = format!("top{:0>5}{}", topk, ".jsonl");
    let top_file = [top_path,&*filename].join("");
    let _ = File::create(top_file);
    let top_file = [top_path,&*filename].join("");
    let mut output = OpenOptions::new()
        .write(true)
        .append(true)
        .open(top_file)
        .unwrap();

    let bar = ProgressBar::new(ori.len() as u64);

    

    // let hard = vec![8, 36, 44, 61, 112, 116, 138, 160, 201, 244, 299, 322, 540, 544, 547, 642, 656, 657, 692, 706, 719, 730, 736, 748, 752, 827, 857, 936, 950, 998, 1005, 1227, 1234, 1235, 1287, 1319, 1343, 1344, 1419, 1476, 1494, 1503, 1508, 1523, 1561, 1670];
    //let hard = vec![56, 59, 106, 141, 147, 239, 355, 398, 411];
    let hard = vec![0, 2, 3, 4, 5, 6, 7];
    for x in hard.iter() {
    //for i in 0..ori.len() {
	bar.inc(1);
    	//let filename = format!("{:0>5}{}", i, ".jsonl");
	let filename = format!("{:0>5}{}", *x, ".jsonl");
    	let lab_file = [lab_path,&*filename].join("");
        let lab_jsonl = read_aug_jsonl(lab_file);
    	let v = match lab_jsonl {
    	    Ok(v)    => v,
    	    Err(err) => {
    		//println!("{:?}", err);
    		continue;
    	    }
    	};

    	let mut n = topk;
    	if v.len() < topk {
    	    n = v.len();
    	}
	for i in 0..n {
   	    if let Ok(line) = serde_json::to_string(&v[i]) {
		//println!("{:?}",line);
    		let _ = write!(output, "{}", [line,"\n".to_string()].join(""));		
	    }
	}
    }
    bar.finish();
}

pub fn merge(topk: usize) {
    // let ori_path    = "/home/heeh/Projects/rusty-wrangler/data/original/citation_intent/";
    // let ori_file    = "/home/heeh/Projects/rusty-wrangler/data/original/citation_intent/train.jsonl".to_string();
    // let top_path    = "/home/heeh/Projects/rusty-wrangler/data/aug_large_top/citation_intent/";
    // let merged_path = ["/home/heeh/Projects/rusty-wrangler/data/merged/citation_intent/".to_string(), format!("top{:0>5}/", topk)].join("");
    // let top_path    = "/home/heeh/Projects/rusty-wrangler/data/aug_large_hard/citation_intent/";
    // let merged_path = ["/home/heeh/Projects/rusty-wrangler/data/merged_hard/citation_intent/".to_string(), format!("top{:0>5}/", topk)].join("");

    let ori_path    = "/home/heeh/Projects/rusty-wrangler/data/original/hyperpartisan_news/";
    let ori_file    = "/home/heeh/Projects/rusty-wrangler/data/original/hyperpartisan_news/train.jsonl".to_string();
    // let top_path    = "/home/heeh/Projects/rusty-wrangler/data/aug_large_top/hyperpartisan_news/";
    // let merged_path = ["/home/heeh/Projects/rusty-wrangler/data/merged/hyperpartisan_news/".to_string(), format!("top{:0>5}/", topk)].join("");
    // let top_path    = "/home/heeh/Projects/rusty-wrangler/data/aug_large_hard/hyperpartisan_news/";
    // let merged_path = ["/home/heeh/Projects/rusty-wrangler/data/merged_hard/hyperpartisan_news/".to_string(), format!("top{:0>5}/", topk)].join("");
    
    let top_path    = "/home/heeh/Projects/rusty-wrangler/data/aug_hard2_top/hyperpartisan_news/";
    let merged_path = ["/home/heeh/Projects/rusty-wrangler/data/aug_hard2_merge/hyperpartisan_news/".to_string(), format!("top{:0>5}/", topk)].join("");

    
    let filename    = format!("top{:0>5}{}", topk, ".jsonl");
    let top_file    = [top_path,&*filename].join("");

    // let ori_data    = read_ori_jsonl(ori_file);
    // let ori         = ori_data.unwrap();
    let hyper_ori_data = read_hyper_ori_jsonl(ori_file);
    let ori = hyper_ori_data.unwrap();

    let top_data    = read_aug_jsonl(top_file);
    let top         = top_data.unwrap();

    let _           = fs::create_dir_all(merged_path.clone());
    let merged_file = [merged_path.clone(),"train.jsonl".to_string()].join("");
    let _           = File::create(merged_file.clone());

    let mut output  = OpenOptions::new()
        .write(true)
        .append(true)
        .open(merged_file.clone())
        .unwrap();
    for i in 0..ori.len() {
   	if let Ok(line) = serde_json::to_string(&ori[i]) {
    	    let _ = write!(output, "{}", [line,"\n".to_string()].join(""));		
	}
    }
    for i in 0..top.len() {
   	if let Ok(line) = serde_json::to_string(&top[i]) {
    	    let _ = write!(output, "{}", [line,"\n".to_string()].join(""));		
	}
    }
    let _ = fs::copy([ori_path, "test.jsonl"].join(""), [merged_path.clone(), "test.jsonl".to_string()].join(""));
    let _ = fs::copy([ori_path, "dev.jsonl"].join(""), [merged_path.clone(), "dev.jsonl".to_string()].join(""));
}
