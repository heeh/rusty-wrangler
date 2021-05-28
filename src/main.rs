// Todo:
// Labeling
// Random picking
// Augment by size

mod lib;
use lib::*;


fn main() {
    let ori_file = "/home/heeh/Projects/augment/data/original/citation_intent/train.jsonl".to_string();
    let aug_path = "/home/heeh/Projects/augment/data/aug_large/citation_intent/";

    // read original data
    println!("= Loading the data into OriData structs");
    let ori_data = read_ori_jsonl(ori_file);
    let ori = ori_data.unwrap();
    println!("{:?}", ori[0]);
    // Label and write aug data
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
	//let lab_path = "/home/heeh/Projects/augment/data/aug_large_lab/citation_intent/";
	// if let Err(e) = write_aug_jsonl(aug_jsonl, lab_path, i as i32) {
	//     println!("{:?}", e);
	// }
    }
}
