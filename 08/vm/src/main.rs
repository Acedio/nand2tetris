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

fn push_named_address(address: String) -> String {
    format!(r"@{address}
              D=M   // get value
              @SP
              M=M+1 // update SP
              A=M-1
              M=D
              ", address = address)
}

fn push_name(name: String) -> String {
    format!(r"@{name}
              D=A   // get value of name
              @SP
              M=M+1
              A=M-1
              M=D
              ", name = name)
}

fn push_ram(address: u16, index: u16) -> String {
    push_named_address((address + index).to_string())
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
        "constant" => push_name(index.to_string()),
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

fn pop_named_address(address: String) -> String {
    format!(r"@SP
              AM=M-1 // Decrement and seek to SP
              D=M
              @{}
              M=D
              ", address)
}

fn pop_ram(address: u16, index: u16) -> String {
    pop_named_address((address + index).to_string())
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

fn label_op(label: &str, func_name: &String) -> String {
    format!(r"(fn:{func_name}:lbl:{label})
              ", func_name = func_name, label = label)
}

fn goto_op(label: &str, func_name: &String) -> String {
    format!(r"@fn:{func_name}:lbl:{label}
              0;JMP
              ", func_name = func_name, label = label)
}

fn if_goto_op(label: &str, func_name: &String) -> String {
    format!(r"@SP
              AM=M-1 // get and update SP at the same time
              D=M    // save the bool
              @fn:{func_name}:lbl:{label}
              D;JNE
              ", func_name = func_name, label = label)
}

fn function_op(func_name: &str, local_vars: u8) -> String {
    format!(r"(fn:{func_name})
              @{local_vars}
              D=A
              (fn:{func_name}:local_vars_top)
              @fn:{func_name}:local_vars_done
              D=D-1;JLT
              @SP
              M=M+1
              A=M-1
              M=0  // clear local var
              @fn:{func_name}:local_vars_top
              0;JMP
              (fn:{func_name}:local_vars_done)
              ", func_name = func_name, local_vars = local_vars.to_string())
}

fn call_op(func_name: &str, args: u8, line_no: usize) -> String {
    format!(r"{push_return}
              {push_lcl}
              {push_arg}
              {push_this}
              {push_that}
              // Update to use the new arg (= SP - args - 5)
              @SP
              D=M
              @{arg_offset}
              D=D-A
              @ARG
              M=D
              // LCL = SP
              @SP
              D=M
              @LCL
              M=D
              // Jump to fn
              @fn:{func_name}
              0;JMP
              (fn:{func_name}:return_for_ln{line_no})
              ",
              push_return =
                  push_name(format!("fn:{}:return_for_ln{}", func_name, line_no.to_string())),
              push_lcl = push_named_address("LCL".to_owned()),
              push_arg = push_named_address("ARG".to_owned()),
              push_this = push_named_address("THIS".to_owned()),
              push_that = push_named_address("THAT".to_owned()),
              arg_offset = (args + 5).to_string(),
              func_name = func_name,
              line_no = line_no.to_string())
}

fn return_op() -> String {
    format!(r"{pop_return_value_to_15}
              // save ARG (the soon to be location of SP) in @14 for later
              @ARG
              D=M
              @14
              M=D
              // move SP to LCL, where our previous virtual addresses are
              @LCL
              D=M
              @SP
              M=D
              // pop virtual addresses
              {pop_that}
              {pop_this}
              {pop_arg}
              {pop_lcl}
              {pop_return_address_to_13}
              // reset SP to @14, where ARG was
              @14
              D=M
              @SP
              M=D
              {push_return_value_from_15}
              // jump to return address
              @13
              A=M
              0;JMP
              ",
              pop_return_value_to_15 = pop_named_address("15".to_owned()),
              pop_return_address_to_13 = pop_named_address("13".to_owned()),
              pop_that = pop_named_address("THAT".to_owned()),
              pop_this = pop_named_address("THIS".to_owned()),
              pop_arg = pop_named_address("ARG".to_owned()),
              pop_lcl = pop_named_address("LCL".to_owned()),
              push_return_value_from_15 = push_named_address("15".to_owned()))
}

fn print_bootstrap() {
    clean_print(r"// begin bootstrap
                  @256
                  D=A
                  @SP
                  M=D

                  @1
                  D=-A
                  @LCL
                  M=D

                  @2
                  D=-A
                  @ARG
                  M=D

                  @3
                  D=-A
                  @THIS
                  M=D

                  @4
                  D=-A
                  @THAT
                  M=D

                  ".to_owned());
    clean_print(call_op("Sys.init", 0, 0));
    clean_print(r"(Sys.init.loop)
                  @Sys.init.loop
                  0;JMP
                  // end bootstrap".to_owned());
}

fn process_line(line: &String, line_no: usize, func_name: &mut String) {
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
        "label" => {
            assert_eq!(2, tokens.len());
            label_op(tokens[1], func_name)
        }
        "goto" => {
            assert_eq!(2, tokens.len());
            goto_op(tokens[1], func_name)
        }
        "if-goto" => {
            assert_eq!(2, tokens.len());
            if_goto_op(tokens[1], func_name)
        }
        "function" => {
            assert_eq!(3, tokens.len());
            *func_name = String::from(tokens[1]);
            function_op(tokens[1], tokens[2].parse().ok().expect("could not parse lcl var count"))
        }
        "call" => {
            assert_eq!(3, tokens.len());
            call_op(tokens[1], tokens[2].parse().ok().expect("could not parse arg count"), line_no)
        }
        "return" => {
            return_op()
        }
        _ => format!("!!! unsupported instruction type: {}\n", tokens[0]),
    });
}

fn main() {
    let stdin = io::stdin();
    let mut func_name = String::from("undefined");
    print_bootstrap();
    for maybe_line in stdin.lock().lines().enumerate() {
        let (line_no, maybe_line) = maybe_line;
        match maybe_line {
            Ok(line) => process_line(&line, line_no, &mut func_name),
            Err(error) => println!("wtf at line {}: {}", line_no, error),
        }
    }
}
