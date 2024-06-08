use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use std::str::FromStr;


enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    Undefined,
}


impl Register {
    fn parse(self) -> String {
        match self {
            Register::R0 => "0".to_string(),
            Register::R1 => "1".to_string(),
            Register::R2 => "2".to_string(),
            Register::R3 => "3".to_string(),
            Register::R4 => "4".to_string(),
            Register::R5 => "5".to_string(),
            Register::Undefined => "unsupported".to_string(),
        }
    }
}


impl FromStr for Register {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp = match s {
            "R0" => Register::R0,
            "R1" => Register::R1,
            "R2" => Register::R2,
            "R3" => Register::R3,
            "R4" => Register::R4,
            "R5" => Register::R5,
            _ => Register::Undefined
        };

        Ok(temp)
    }
}


#[derive(Debug)]
enum Command {
    Mov,
    Movi,
    Nop,
    Jump,
    Jumpi,
    Jz,
    Jnz,
    Add,
    Addi,
    And,
    Andi,
    Load,
    Loadi,
    Undefined,
}


impl Command {
    fn parse_command(&self) -> String {
        match self {
            Command::Mov => "0x001<RX>6<RD>00".to_string(),
            Command::Movi => "0x00168<RD><IMM>".to_string(),
            Command::Nop => "0x00300000".to_string(),
            Command::Jump => "0x011<RX>8000".to_string(),
            Command::Jumpi => "0x013283<IMM>".to_string(),
            Command::Jz => "0x023<RX>86<IMM>".to_string(),
            Command::Jnz => "0x033<RX>86<IMM>".to_string(),
            Command::Add => "0x001<RX><RY><RD>00".to_string(),
            Command::Addi => "0x001<RX>8<RD><IMM>".to_string(),
            Command::And => "0x000<RX><RY><RD>00".to_string(),
            Command::Andi => "0x000<RX>8<RD><IMM>".to_string(),
            Command::Load => "0x001<RX>8<RD*>00".to_string(),
            Command::Loadi => "0x00308<RD*><IMM>".to_string(),
            Command::Undefined => "unsupported".to_string()
        }
    }

    fn parse_inputs(&self) -> i8 {
        match self {
            Command::Mov => 2,
            Command::Movi => 2,
            Command::Nop => 0,
            Command::Jump => 1,
            Command::Jumpi => 1,
            Command::Jz => 2,
            Command::Jnz => 2,
            Command::Add => 3,
            Command::Addi => 3,
            Command::And => 3,
            Command::Andi => 3,
            Command::Load => 2,
            Command::Loadi => 2,
            Command::Undefined => -128
        }
    }
}


#[derive(Debug)]
struct Instruction {
    command: Command,
    data: String,
    no_inputs: i8,
}


impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let command = match instruction {
            "mov" => Command::Mov,
            "movi" => Command::Movi,
            "nop" => Command::Nop,
            "jump" => Command::Jump,
            "jumpi" => Command::Jumpi,
            "jz" => Command::Jz,
            "jnz" => Command::Jnz,
            "add" => Command::Add,
            "addi" => Command::Addi,
            "and" => Command::And,
            "andi" => Command::Andi,
            "load" => Command::Load,
            "loadi" => Command::Loadi,
            _ => Command::Undefined
        };

        let data = command.parse_command();
        let no_inputs = command.parse_inputs();

        Ok(Instruction { command, data, no_inputs })
    }
}


fn read_file(file_name: &str) -> Result<String, String> {
    let mut file = match OpenOptions::new().read(true).write(false).open(file_name) {
        Ok(file) => file,
        Err(error) => return Err(error.to_string())
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(read_bytes) => println!("Read {read_bytes} bytes"),
        Err(error) => return Err(error.to_string())
    }

    Ok(buffer)
}

fn open_file(file_name: &str) -> std::io::Result<File> {
    OpenOptions::new().read(false).write(true).create(true).open(file_name)
}

fn save_to_file(file_name: &str, data: &[u8]) -> Result<(), String> {
    let mut file = match open_file(file_name) {
        Ok(file) => file,
        Err(error) => return Err(error.to_string())
    };

    match file.write(data) {
        Ok(saved_bytes) => println!("Saved {saved_bytes} bytes"),
        Err(error) => return Err(error.to_string())
    };

    match file.sync_data() {
        Ok(_) => println!("Data synced"),
        Err(error) => return Err(error.to_string())
    }

    Ok(())
}

fn repair_register(reg: &str) -> String {
    let temp = (reg.parse::<u8>().unwrap() << 1) + 1;
    format!("{:x}", temp)
}


fn fill_instruction(instruction: &mut Instruction, input_data: &Vec<&str>) -> Result<(), String> {
    let new_data = match instruction.command {
        Command::Mov => {
            let rd = input_data.get(1).unwrap().to_uppercase().parse::<Register>().unwrap().parse();
            let rx = input_data.get(2).unwrap().to_uppercase().parse::<Register>().unwrap().parse();
            if rx == "unsupported" || rd == "unsupported" { return Err(String::from("Unsupported register")); }

            let temp_data = instruction.data.replace("<RD>", &rd);
            temp_data.replace("<RX>", &rx)
        }

        Command::Movi => {
            let rd = input_data.get(1).unwrap().to_uppercase().parse::<Register>().unwrap().parse();
            if rd == "unsupported" { return Err(String::from("Unsupported register")); }

            let imm = match input_data.get(2).unwrap().parse::<u8>() {
                Ok(imm) => imm,
                Err(error) => return Err(error.to_string())
            };

            let temp_data = instruction.data.replace("<RD>", &rd);
            temp_data.replace("<IMM>", &format!("{:02}", imm))
        },

        Command::Nop => {
            instruction.data.clone()
        },

        Command::Jump => {
            let rx = input_data.get(1).unwrap().to_uppercase().parse::<Register>().unwrap().parse();
            if rx == "unsupported" { return Err(String::from("Unsupported register")); }

            instruction.data.replace("<RX>", &rx)
        },

        Command::Jumpi => {
            let imm = match input_data.get(1).unwrap().parse::<u8>() {
                Ok(imm) => imm,
                Err(error) => return Err(error.to_string())
            };

            instruction.data.replace("<IMM>", &format!("{:02}", imm))
        },

        Command::Jz => {
            let rx = input_data.get(1).unwrap().to_uppercase().parse::<Register>().unwrap().parse();
            if rx == "unsupported" { return Err(String::from("Unsupported register")); }

            let imm = match input_data.get(2).unwrap().parse::<u8>() {
                Ok(imm) => imm,
                Err(error) => return Err(error.to_string())
            };

            let temp_data = instruction.data.replace("<RX>", &rx);
            temp_data.replace("<IMM>", &format!("{:02}", imm))
        },

        Command::Jnz => {
            let rx = input_data.get(1).unwrap().to_uppercase().parse::<Register>().unwrap().parse();
            if rx == "unsupported" { return Err(String::from("Unsupported register")); }

            let imm = match input_data.get(1).unwrap().parse::<u8>() {
                Ok(imm) => imm,
                Err(error) => return Err(error.to_string())
            };

            let temp_data = instruction.data.replace("<RX>", &rx);
            temp_data.replace("<IMM>", &format!("{:02}", imm))
        },

        Command::Add => {
            let rd = input_data.get(1).unwrap().parse::<Register>().unwrap().parse();
            let rx = input_data.get(2).unwrap().parse::<Register>().unwrap().parse();
            let ry = input_data.get(3).unwrap().parse::<Register>().unwrap().parse();
            if rd == "unsupported" || rx == "unsupported" || ry == "unsupported" { return Err(String::from("Unsupported register")); }

            let mut temp_data = instruction.data.replace("<RD>", &rd);
            temp_data = temp_data.replace("<RX>", &rx);
            temp_data.replace("<RY>", &ry)
        },

        Command::Addi => {
            let rd = input_data.get(1).unwrap().parse::<Register>().unwrap().parse();
            let rx = input_data.get(2).unwrap().parse::<Register>().unwrap().parse();
            if rd == "unsupported" || rx == "unsupported" { return Err(String::from("Unsupported register")); }

            let imm = match input_data.get(3).unwrap().parse::<u8>() {
                Ok(imm) => imm,
                Err(error) => return Err(error.to_string())
            };

            let mut temp_data = instruction.data.replace("<RD>", &rd);
            temp_data = temp_data.replace("<RX>", &rx);
            temp_data.replace("<IMM>", &format!("{:02}", imm))

        },

        Command::And => {
            let rd = input_data.get(1).unwrap().parse::<Register>().unwrap().parse();
            let rx = input_data.get(2).unwrap().parse::<Register>().unwrap().parse();
            let ry = input_data.get(3).unwrap().parse::<Register>().unwrap().parse();
            if rd == "unsupported" || rx == "unsupported" || ry == "unsupported" { return Err(String::from("Unsupported register")); }

            let mut temp_data = instruction.data.replace("<RD>", &rd);
            temp_data = temp_data.replace("<RX>", &rx);
            temp_data.replace("<RY>", &ry)
        },

        Command::Andi => {
            let rd = input_data.get(1).unwrap().parse::<Register>().unwrap().parse();
            let rx = input_data.get(2).unwrap().parse::<Register>().unwrap().parse();
            if rd == "unsupported" || rx == "unsupported" { return Err(String::from("Unsupported register")); }

            let imm = match input_data.get(3).unwrap().parse::<u8>() {
                Ok(imm) => imm,
                Err(error) => return Err(error.to_string())
            };

            let mut temp_data = instruction.data.replace("<RD>", &rd);
            temp_data = temp_data.replace("<RX>", &rx);
            temp_data.replace("<IMM>", &format!("{:02}", imm))

        },

        Command::Load => {
            let rd = input_data.get(1).unwrap().parse::<Register>().unwrap().parse();
            let rx = input_data.get(2).unwrap().parse::<Register>().unwrap().parse();
            if rd == "unsupported" || rx == "unsupported" { return Err(String::from("Unsupported register")); }
            let rd = repair_register(&rd);

            let temp_data = instruction.data.replace("<RD*>", &rd);
            temp_data.replace("<RX>", &rx)
        },

        Command::Loadi => {
            let rd = input_data.get(1).unwrap().parse::<Register>().unwrap().parse();
            if rd == "unsupported" { return Err(String::from("Unsupported register")); }
            let rd = repair_register(&rd);

            let imm = match input_data.get(2).unwrap().parse::<u8>() {
                Ok(imm) => imm,
                Err(error) => return Err(error.to_string())
            };

            let temp_data = instruction.data.replace("<RD*>", &rd);
            temp_data.replace("<IMM>", &format!("{:02}", imm))
        },

        Command::Undefined => return Err("Undefined command".to_string())
    };

    instruction.data = new_data;

    Ok(())
}


pub fn compile_asm2ms() -> Result<(), String> {
    let instr_lines = match read_file("program.asm") {
        Ok(var) => var,
        Err(error) => return Err(error.to_string())
    };

    let mut buffer: Vec<u8> = Vec::new();

    for line in instr_lines.lines() {
        let line = line.split(' ').collect::<Vec<&str>>();
        let line = line.iter().map(|x| { x.trim_matches(',') }).collect::<Vec<&str>>();

        let mut instruction = line.get(0).unwrap().to_string().parse::<Instruction>().unwrap();

        if instruction.no_inputs == (line.len() - 1) as i8 {
            match fill_instruction(&mut instruction, &line) {
                Ok(_) => {
                    println!("{instruction:?}");
                    let mut instruction_u8  = Vec::from(instruction.data.as_bytes());
                    buffer.append(&mut instruction_u8);
                    buffer.push(13); // \r
                    buffer.push(10); // \n
                },
                Err(error_message) => println!("{error_message}")
            };
        } else {
            match instruction.command {
                Command::Undefined => println!("Undefined command"),
                _ => println!("Wrong instruction arguments")
            }
        }
    }

    match save_to_file("program.ms", buffer.as_slice()) {
        Ok(_) => (),
        Err(error_message) => return Err(error_message)
    };

    Ok(())
}
