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
    // for i in 1..=10 {
    // 	collect(i);
    // }
//    merge(1);
    for i in 2..=10 {
    	merge(i);	
    }
}
