#![feature(test)]

extern crate ykstackmaps;
extern crate fizzbuzz;
extern crate hwtracer;
extern crate test;

use test::black_box;

//use ykstackmaps::StackMapParser;

use fizzbuzz::fizzbuzz;
use hwtracer::backends::TracerBuilder;

pub fn main() {
    let mut tracer = TracerBuilder::new().build().unwrap().thread_tracer();
    tracer.start_tracing().unwrap();
    fizzbuzz(black_box(1));
    let trace = tracer.stop_tracing().unwrap();

    for b in trace.iter_blocks() {
        if let Err(e) = b {
            println!("error: {}", e);
            return;
        }
        let b = b.unwrap();
        println!("0x{:x}", b.start_vaddr());
    }
}
