/**
 * Author: Von Divino
 * Date: May 5, 2024 4:14 AM
 * Description: Pricssor1 Assembler
 */


use std::env;
use std::fs;
use std::{fs::File, io::Write};
use std::u32;



fn parse_instruction(instruction: &str) -> Vec<&str>{
    let parts: Vec<&str> = instruction.split(" ").collect();
    parts
}

fn parse_dta_adr(dta_adr: &str) -> String {
    let dta_adr = if dta_adr == "" {
        "DTA0"
    } else {
        dta_adr
    };
    let adr = dta_adr.chars().last().unwrap().to_string();
    let adr_num: i8 = adr.parse().unwrap();
    if adr_num < 0 || adr_num > 7 {
        panic!();
    }
    String::from(format!("{:03b}", adr_num))
}

fn parse_out_adr(out_adr: &str) -> String {
    let out_adr = if out_adr == "" {
        "OUT0"
    } else {
        out_adr
    };
    let adr = out_adr.chars().last().unwrap().to_string();
    let adr_num: i8 = adr.parse().unwrap();
    if adr_num < 0 || adr_num > 7 {
        panic!();
    }
    String::from(format!("{:03b}", adr_num))
}


fn bin_to_hex(bin:&str) -> String {
    let hex = u32::from_str_radix(bin, 2).unwrap();
    format!("{:01$x}", hex, 4)
}

fn hex_to_decimal(hex:&char) -> i8 {
    if ['0','1','2','3','4','5','6','7','8','9'].contains(hex) {
        let dec: i8 = hex.to_string().parse().unwrap();
        dec
    } else if *hex == 'a' {
        10
    } else if *hex == 'b' {
        11
    } else if *hex == 'c' {
        12
    } else if *hex == 'd' {
        13
    } else if *hex == 'e' {
        14
    } else if *hex == 'f' {
        15
    } else {
        panic!()
    }
} 

fn parse_mem_adr(mem_adr: &str) -> String {
    let mut adr = mem_adr.chars().rev();
    let lower_adr = adr.next().unwrap().to_lowercase().last().unwrap();
    let upper_adr  = adr.next().unwrap().to_lowercase().last().unwrap();

    String::from_iter([
        format!("{:04b}", hex_to_decimal(&upper_adr)),
        format!("{:04b}", hex_to_decimal(&lower_adr))
    ])
}

fn parse_nop() -> String {
    String::from("0000000000000000")
}

fn parse_hlt() -> String {
    String::from("1111111111111111")
}

fn parse_int() -> String {
    String::from("0000111100001111")
}

fn parse_mov(z: &str, x:&str) -> String {
    String::from_iter([
        "0001",
        "0",
        &parse_dta_adr(&z),
        "0",
        &parse_dta_adr(&x),
        "0000"
    ])
}

fn parse_mvi(z: &str, val:&str) -> String {
    let val_res: u8 = val.trim().parse().unwrap();
    String::from_iter([
        "0001",
        "1",
        &parse_dta_adr(&z),
        &format!("{:08b}",val_res)
    ])

}

fn parse_arilog(opcode: &str, z: &str, x:&str, y:&str) -> String {
    let pads = if opcode == "ADD" {
        ("0", "0", "0")
    } else if opcode == "SUB" {
        ("0", "0", "1")
    } else if opcode == "MUL" {
        ("0", "1", "0")
    } else if opcode == "DIV" {
        ("0", "1", "1")
    } else if opcode == "NOT" {
        ("1", "0", "0")
    } else if opcode == "AND" {
        ("1", "0", "1")
    } else if opcode == "LOR" {
        ("1", "1", "0")
    } else if opcode == "XOR" {
        ("1", "1", "1")
    } else {
        panic!();
    };

    String::from_iter([
        "0010",
        pads.0,
        &parse_dta_adr(&z),
        pads.1,
        &parse_dta_adr(&x),
        pads.2,
        &parse_dta_adr(&y),

    ])
}

fn parse_mvab(opcode: &str, x:&str) -> String {
    let opr = if opcode == "MVA" {
        "0000"
    } else if opcode == "MVB" {
        "0001"
    } else {
        panic!()
    };

    String::from_iter([
        "0011",
        opr, 
        "0",
        &parse_dta_adr(&x),
        "0000"
    ])

}

fn parse_cmp(opcode:&str) -> String {
    let opr = if opcode == "CME" {
        "0000"
    } else if opcode == "CMN" {
        "0001"
    } else if opcode == "CML" {
        "0010"
    } else if opcode == "CMG" {
        "0011"
    } else if opcode == "CMF" {
        "0100"
    } else if opcode == "CMT" {
        "0101"
    } else {
        panic!()
    };

    String::from_iter([
        "0100",
        opr,
        "00000000"
    ])
}

fn parse_jmp(opcode: &str, mem: &str) -> String {
    let opr = if opcode == "JMP" {
        "0000"
    } else if opcode == "JMF" {
        "0001"
    } else if opcode == "JMT" {
        "0010"
    } else {
        panic!()
    };

    String::from_iter([
        "0101",
        opr, 
        &parse_mem_adr(&mem)
    ])
}

fn parse_get() -> String {
    String::from_iter([
        "0110",
        "0000",
        "0000",
        "0000"
    ])
}

fn parse_put(out_adr: &str, x_adr:&str) -> String {
    String::from_iter([
        "0110",
        "1",
        &parse_out_adr(&out_adr),
        "0",
        &parse_dta_adr(&x_adr),
        "0000"
    ])
}

fn parse_mvg(z_adr:&str) -> String {
    String::from_iter([
        "0111",
        "0",
        &parse_dta_adr(&z_adr),
        "00000000"
    ])
}

fn parse_psh(x_adr:&str) -> String {
    String::from_iter([
        "10001111",
        "0",
        &parse_dta_adr(&x_adr),
        "0000"
    ])
}

fn parse_pop(z_adr:&str) -> String {
    String::from_iter([
        "1000",
        "0",
        &parse_dta_adr(&z_adr),
        "00000000"
    ])
}

fn parse_png(val:&str) -> String {
    let val_res: u8 = val.trim().parse().unwrap();
    String::from_iter([
        "10010000",
        &format!("{:08b}", val_res)
    ])
}

fn read_source_code(file_path:&str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let contents_vec: Vec<&str> = contents.split("\n").collect();

    let mut source_code: Vec<String> = Vec::new();

    for content in contents_vec { 
        source_code.push(content.to_string());
    }

    source_code
    
}

fn write_assembly(file_path:&str, assembly: &str) {
    let file_result = File::create(file_path);
    match file_result {
        Ok(mut file) => {
            file.write(assembly.as_bytes()).expect("");
        },
        Err(_) => {}
    };

}




fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file_path_segs: Vec<&str> = file_path.split(".").collect();
    let file_name = file_path_segs.get(0).unwrap();
    let file_ext = file_path_segs.get(1).unwrap();

    if *file_ext != "pics" {
        panic!();
    }


    let instructions = read_source_code(&file_path);

    let mut assembly = String::new();

    for instruction in &instructions{
        let instruction_parts = parse_instruction(instruction);
        let opcode = instruction_parts.get(0).unwrap();
        let op1 = instruction_parts.get(1).unwrap_or(&"");
        let op2 = instruction_parts.get(2).unwrap_or(&"");
        let op3 = instruction_parts.get(3).unwrap_or(&"");


        let bin_instruction;

        
        if *opcode == "NOP" {
            bin_instruction = parse_nop();
        } else if *opcode == "HLT" {
            bin_instruction = parse_hlt();
        } else if *opcode == "MOV" {
            bin_instruction = parse_mov(&op1,&op2);
        } else if *opcode == "MVI" {
            bin_instruction = parse_mvi(&op1, &op2);
        } else if ["ADD", "SUB","MUL", "DIV", "NOT", "AND", "LOR", "XOR"].contains(&opcode) {
            bin_instruction = parse_arilog(&opcode,&op1,&op2,&op3);
        } else if ["MVA", "MVB"].contains(&opcode) {
            bin_instruction = parse_mvab(&opcode, &op1);
        } else if ["CME","CMN","CML","CMG","CMF","CMT"].contains(&opcode){
            bin_instruction = parse_cmp(&opcode);
        } else if ["JMP","JMT","JMF"].contains(&opcode) {
            bin_instruction = parse_jmp(&opcode, &op1);
        } else if *opcode == "INT" {
            bin_instruction = parse_int();
        } else if *opcode == "GET" {
            bin_instruction = parse_get();
        }  else if *opcode == "PUT" {
            bin_instruction = parse_put(&op1, &op2);
        } else if *opcode == "MVG" {
            bin_instruction = parse_mvg(&op1);
        } else if *opcode == "PSH" {
            bin_instruction = parse_psh(&op1);
        } else if *opcode == "POP" {
            bin_instruction = parse_pop(&op1);
        } else if *opcode == "PNG" {
            bin_instruction = parse_png(&op1);
        } else {
            panic!();
        }

        assembly.push_str(&bin_to_hex(&bin_instruction));
        assembly.push_str("\n");
    }

    write_assembly(&[&file_name,"out"].join("."), &assembly);
}
