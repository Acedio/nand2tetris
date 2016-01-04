use std::io;
use std::io::prelude::*;

// TODO: multi-file
static FILE_NAME: &'static str = "fakefile.vm";

fn clean_print(lines: String) {
    for line in lines.lines() {
        println!("{}", line.trim());
    }
}

// op is between D and M, where D = y and M = x
fn binary_op(op: &str) -> String {
    // maybe always assume M=@SP for each instruction?
    format!(r"@SP
              AM=M-1 // get and update SP at the same time
              D=M    // save y
              A=A-1
              M={}   // calculate x `op` y
              ", op)
}

fn unary_op(op: &str) -> String {
    format!(r"@SP
              A=M-1
              M={}
              ", op)
}

// line_no to make a unique label
fn cmp_op(op: &str, line_no: usize) -> String {
    format!(r"@SP
              AM=M-1 // get and update SP at the same time
              D=M    // save y
              A=A-1
              D=M-D  // comparing M (x) vs D (y)
              M=-1   // true by default
              @iftrue.{}
              D;{}
              @SP    // have to get SP again now
              A=M-1
              M=0    // false if we didn't jump
              (iftrue.{})
              ", line_no, op, line_no)
}

fn push_virtual(symbol: &str, index: u16) -> String {
    format!(r"@{}
              D=A
              @{}
              A=M
              A=D+A // move to address + index
              D=M   // get value
              @SP
              M=M+1 // update SP
              A=M-1
              M=D
              ", index, symbol)
}

fn push_ram(address: u16, index: u16) -> String {
    format!(r"@{}
              D=M   // get value
              @SP
              M=M+1 // update SP
              A=M-1
              M=D
              ", address + index)
}

fn push_static(file: &str, index: u16) -> String {
    format!(r"@{}.{}
              D=M   // get value
              @SP
              M=M+1 // update SP
              A=M-1
              M=D
              ", file, index)
}

fn push_op(segment: &str, index: u16) -> String {
    match segment {
        "argument" => push_virtual("ARG", index),
        "local"    => push_virtual("LCL", index),
        "this"     => push_virtual("THIS", index),
        "that"     => push_virtual("THAT", index),
        "temp"     => push_ram(5, index),
        "pointer"  => push_ram(3, index),
        "static"   => push_static(FILE_NAME, index),
        "constant" => { format!(r"@{}
                                  D=A
                                  @SP
                                  A=M
                                  M=D  // push constant value on to stack
                                  D=A+1
                                  @SP
                                  M=D  // update SP
                                  ", index) }
        _ => format!("!!! unimplemented segment: {}\n", segment),
    }
}

fn pop_virtual(symbol: &str, index: u16) -> String {
    format!(r"@{}
              D=M
              @{}
              D=D+A
              @13
              M=D    // Store destination index in RAM[13]
              @SP
              AM=M-1 // Decrement and seek to SP
              D=M
              @13
              A=M
              M=D
              ", symbol, index)
}

fn pop_ram(address: u16, index: u16) -> String {
    format!(r"@SP
              AM=M-1 // Decrement and seek to SP
              D=M
              @{}
              M=D
              ", address + index)
}

fn pop_static(file: &str, index: u16) -> String {
    format!(r"@SP
              AM=M-1 // Decrement and seek to SP
              D=M
              @{}.{}
              M=D
              ", file, index)
}

fn pop_op(segment: &str, index: u16) -> String {
    match segment {
        "argument" => pop_virtual("ARG", index),
        "local"    => pop_virtual("LCL", index),
        "this"     => pop_virtual("THIS", index),
        "that"     => pop_virtual("THAT", index),
        "temp"     => pop_ram(5, index),
        "pointer"  => pop_ram(3, index),
        "static"   => pop_static(FILE_NAME, index),
        _          => format!("!!! unimplemented segment: {}\n", segment),
    }
}

fn process_line(line: &String, line_no: usize) {
    let line = line.split("//").next().unwrap_or("");  // remove comments
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.is_empty() { return }
    print!("// line {}: ", line_no);
    for token in &tokens {
        print!("{} ", token);
    }
    println!("");
    clean_print(match tokens[0] {
        "add" => binary_op("D+M"),
        "sub" => binary_op("M-D"),
        "neg" => unary_op("-M"),
        "eq" => cmp_op("JEQ", line_no),
        "gt" => cmp_op("JGT", line_no),
        "lt" => cmp_op("JLT", line_no),
        "and" => binary_op("D&M"),
        "or" => binary_op("D|M"),
        "not" => unary_op("!M"),
        "push" => {
            assert_eq!(3, tokens.len());
            push_op(tokens[1], tokens[2].parse().ok().expect("could not parse index"))
        }
        "pop" => {
            assert_eq!(3, tokens.len());
            pop_op(tokens[1], tokens[2].parse().ok().expect("could not parse index"))
        }
        _ => format!("!!! unsupported instruction type: {}\n", tokens[0]),
    });
}

fn main() {
    let stdin = io::stdin();
    for maybe_line in stdin.lock().lines().enumerate() {
        let (line_no, maybe_line) = maybe_line;
        match maybe_line {
            Ok(line) => process_line(&line, line_no),
            Err(error) => println!("wtf at line {}: {}", line_no, error),
        }
    }
}
