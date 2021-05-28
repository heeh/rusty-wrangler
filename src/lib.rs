// Todo:
// Labeling
// Random picking
// Augment by size

use std::io::prelude::*;
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
    fs::create_dir_all(path);
    let filename = format!("{:0>5}{}", num, ".jsonl");

    let lab_file = [path,&*filename].join("");

    println!("{:?}", lab_file);
    let mut output = File::create(lab_file)?;
    for j in v {
    	let line = serde_json::to_string(&j)?;
    	write!(output, "{}", line)?;
    }
    Ok(())
}



pub fn label() {
    let ori_file = "/home/heeh/Projects/augment/data/original/citation_intent/train.jsonl".to_string();
    let aug_path = "/home/heeh/Projects/augment/data/aug_large/citation_intent/";
    let lab_path = "/home/heeh/Projects/augment/data/aug_large_lab/citation_intent/";    

    println!("= Loading the data into OriData structs");
    let ori_data = read_ori_jsonl(ori_file);
    let ori = ori_data.unwrap();

    println!("= Loading the data into AugData structs");
    for i in 0..ori.len() {
    	let filename = format!("{:0>5}{}", i, ".jsonl");
    	let aug_file = [aug_path,&*filename].join("");
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


pub fn collect() {
    
}

pub fn merge() {
    
}
