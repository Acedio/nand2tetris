use std::io;
use std::io::prelude::*;

fn clean_print(lines : String) {
    for line in lines.lines() {
        println!("{}", line.split("//")
                           .next()
                           .expect("couldn't split out comments")
                           .trim());
    }
}

// op is between D and M, where D = y and M = x
fn binary_op(op : &str) {
    // maybe always assume M=@SP for each instruction?
    clean_print(format!(r"
    @0
    AM=M-1 // get and update SP at the same time
    D=M    // save y
    A=A-1
    M={}   // calculate x `op` y
    ", op));
}

fn unary_op(op : &str) {
    clean_print(format!(r"
    @0
    A=M-1
    M={}", op));
}

fn cmp_op(op : &str) {
    clean_print(format!(r"
    @0
    AM=M-1 // get and update SP at the same time
    D=M    // save y
    A=A-1
    D=M-D  // comparing M (x) vs D (y)
    M=-1   // true by default
    @iftrue
    D;{}
    M=0    // false if no jump
    iftrue:
    ", op));
}

fn push_op(segment: &str, index: u16) {
    match segment {
        "constant" => { clean_print(format!(r"@{}
                                      D=A
                                      @0
                                      A=M
                                      M=D  // push constant value on to stack
                                      D=A+1
                                      @0
                                      M=D  // update SP", index)) }
        _ => println!("!!! unimplemented segment: {}", segment),
    }
}

fn process_line(line: &String) {
    let line = line.split("//").next().unwrap_or("");  // remove comments
    let tokens : Vec<&str> = line.split_whitespace().collect();
    if tokens.is_empty() { return }
    match tokens[0] {
        "add" => binary_op("D+M"),
        "sub" => binary_op("M-D"),
        "neg" => unary_op("-M"),
        "eq" => cmp_op("JEQ"),
        "gt" => cmp_op("JGT"),
        "lt" => cmp_op("JLT"),
        "and" => binary_op("D&M"),
        "or" => binary_op("D|M"),
        "not" => unary_op("!M"),
        "push" => {
            assert_eq!(3, tokens.len());
            push_op(tokens[1], tokens[2].parse().ok().expect("could not parse index"));
        }
        _ => println!("unsupported instruction type: {}", tokens[0]),
    }
}

fn main() {
    let stdin = io::stdin();
    for maybe_line in stdin.lock().lines() {
        match maybe_line {
            Ok(line) => process_line(&line),
            Err(error) => println!("wtf: {}", error),
        }
    }
}
