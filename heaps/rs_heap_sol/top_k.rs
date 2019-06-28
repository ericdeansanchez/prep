use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::env;
use std::fs;

#[derive(Eq, PartialEq)]
struct Element<'a> {
    length: usize,
    line: &'a str,
}

impl<'a> Element<'a> {
    fn new(length: usize, line: &'a str) -> Element {
        Element {
            length,
            line
        }
    }
}

impl<'a> PartialOrd for Element<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.length.partial_cmp(&self.length)
    }
}

impl<'a> Ord for Element<'a> {
    fn cmp(&self, other: &Element) -> Ordering {
        other.length.cmp(&self.length)
    }
}

pub fn top_k<'a>(k: usize, input: &'a str) -> Vec<&'a str> {
    let mut min_heap: BinaryHeap<Element> = BinaryHeap::new();
    let stream: Vec<&str> = input.split('\n').collect();

    for elem in stream {
        min_heap.push(Element::new(elem.len(), elem));

        if min_heap.len() > k {
            min_heap.pop();
        }
    }

    let mut result: Vec<&str> = Vec::new();
    while !min_heap.is_empty() {
        let val: Element = min_heap
            .pop()
            .unwrap_or(Element::new(0, ""));
        result.push(val.line);
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[2];
    println!("In file {}\n", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file...");

    let result = top_k(5, &contents[0..]);
    for elem in result {
        println!("{}", elem);
    }
}
