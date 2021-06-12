// Todo:
// Labeling
// Collect aug data
// Merge train and aug data
mod lib;
//use lib::label;
//use lib::collect;
use lib::merge;

fn main() {
    //label();
    // let acl_hard = vec![5,10,20,30,50,100,150,200,250,300,350,400,450,500,550,600];
    let hyper_hard = vec![5,10,20,30,50,100,150,200,250, 300, 350, 400, 450, 500, 550, 600];
    
    // for i in 1..=10 {
    // 	collect(i);
    // }
    // for i in 1..=10 {
    // 	merge(i);	
    // }

    // for x in hard.iter() {
    // 	collect(*x);
    // }
    // for x in hard.iter() {
    // 	merge(*x);
    // }
    // for x in hyper_hard.iter() {
    // 	collect(*x);
    // }
    for x in hyper_hard.iter() {
    	merge(*x);
    }
}
