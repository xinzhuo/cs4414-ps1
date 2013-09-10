use std::{os, uint, float};

fn main() {
	let mut counter = 0;
	let mut sum = 0.0;
 	let args: ~[~str] = os::args();
  	for args.iter().skip(1).advance |e| {
  		match float::from_str(*e){
  			Some(x) => {
  				sum += x;
  				counter += 1;
  			}
  			None => {
  				println(fmt!("Bad input: %s", *e));
  			}
  		}
	}
	println(fmt!("Average: %", sum / (counter as float)));
}