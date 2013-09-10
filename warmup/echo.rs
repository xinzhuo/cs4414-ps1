use std::{os, uint};

fn main() {
  let args: ~[~str] = os::args();
  for args.iter().skip(1).advance |e| {
  	println(*e);
  }
}
