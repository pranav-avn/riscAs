use std::collections::HashMap;
#[derive(Debug)]
enum InstKind{
    RType {funct3: u8, funct7: u8},
    IType {funct3: u8},
    SType {funct3: u8},
    BType {funct3: u8},
    UType,
    JType,
}

use std::{fs::File, io::BufRead, io::BufReader};

struct InstFmt {
    opcode: u8,
    kind: InstKind,
}


fn get_arithmetic_instructions() -> HashMap<&'static str, InstFmt> {
    let mut map = HashMap::new();

    // R-type arithmetic instructions (opcode: 0b0110011)
    map.insert("add", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b000,
            funct7: 0b0000000,
        },
    });

    map.insert("sub", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b000,
            funct7: 0b0100000,
        },
    });

    map.insert("sll", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b001,
            funct7: 0b0000000,
        },
    });

    map.insert("slt", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b010,
            funct7: 0b0000000,
        },
    });

    map.insert("sltu", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b011,
            funct7: 0b0000000,
        },
    });

    map.insert("xor", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b100,
            funct7: 0b0000000,
        },
    });

    map.insert("srl", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b101,
            funct7: 0b0000000,
        },
    });

    map.insert("sra", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b101,
            funct7: 0b0100000,
        },
    });

    map.insert("or", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b110,
            funct7: 0b0000000,
        },
    });

    map.insert("and", InstFmt {
        opcode: 0b0110011,
        kind: InstKind::RType {
            funct3: 0b111,
            funct7: 0b0000000,
        },
    });

    // I-type arithmetic instructions (opcode: 0b0010011)
    map.insert("addi", InstFmt {
        opcode: 0b0010011,
        kind: InstKind::IType {
            funct3: 0b000,
        },
    });

    map.insert("slti", InstFmt {
        opcode: 0b0010011,
        kind: InstKind::IType {
            funct3: 0b010,
        },
    });

    map.insert("sltiu", InstFmt {
        opcode: 0b0010011,
        kind: InstKind::IType {
            funct3: 0b011,
        },
    });

    map.insert("xori", InstFmt {
        opcode: 0b0010011,
        kind: InstKind::IType {
            funct3: 0b100,
        },
    });

    map.insert("ori", InstFmt {
        opcode: 0b0010011,
        kind: InstKind::IType {
            funct3: 0b110,
        },
    });

    map.insert("andi", InstFmt {
        opcode: 0b0010011,
        kind: InstKind::IType {
            funct3: 0b111,
        },
    });

    map.insert("slli", InstFmt {
        opcode: 0b0010011,
        kind: InstKind::IType {
            funct3: 0b001,
        },
    });

    map.insert("srli", InstFmt {
        opcode: 0b0010011,
        kind: InstKind::IType {
            funct3: 0b101,
        },
    });

    map.insert("srai", InstFmt {
        opcode: 0b0010011,
        kind: InstKind::IType {
            funct3: 0b101,
        },
    });

    map
}

fn reg_to_u8(reg: &str) -> Option<u8> {
    let name = reg.trim_start_matches('x');
    name.parse::<u8>().ok().filter(|&r| r < 32)
}

fn encode_r_type(opcode: u8, funct3: u8, funct7: u8, rd: u8, rs1: u8, rs2: u8) -> u32 {
    ((funct7 as u32) << 25)
        | ((rs2 as u32) << 20)
        | ((rs1 as u32) << 15)
        | ((funct3 as u32) << 12)
        | ((rd as u32) << 7)
        | (opcode as u32)
}

fn encode_i_type(opcode: u8, funct3: u8, rd: u8, rs1: u8, imm: i32) -> u32 {
    let imm12 = (imm as u32) & 0b0000_1111_1111_1111; // 12-bit immediate
    (imm12 << 20)
        | ((rs1 as u32) << 15)
        | ((funct3 as u32) << 12)
        | ((rd as u32) << 7)
        | (opcode as u32)
}


pub fn asm_parser(contents: BufReader<File>) {
    let instr_map = get_arithmetic_instructions();

    for line_result in contents.lines() {
        match line_result {
            Ok(line) => {
                let trimmed = line.trim_start();
                if trimmed.starts_with('.') {
                    continue;
                }

                let tokens: Vec<String> = trimmed
                    .split('#').next().unwrap_or("")
                    .split_whitespace()
                    .map(|s| s.trim_end_matches(',').to_string())
                    .collect();

                if tokens.is_empty() {
                    continue;
                }

                let mnemonic = tokens[0].as_str();

                if let Some(instr) = instr_map.get(mnemonic) {
                    match &instr.kind {
                        InstKind::RType { funct3, funct7 } => {
                            if tokens.len() == 4 {
                                let rd = reg_to_u8(&tokens[1]);
                                let rs1 = reg_to_u8(&tokens[2]);
                                let rs2 = reg_to_u8(&tokens[3]);
                                if let (Some(rd), Some(rs1), Some(rs2)) = (rd, rs1, rs2) {
                                    let binary = encode_r_type(instr.opcode, *funct3, *funct7, rd, rs1, rs2);
                                    println!("{:032b}", binary);
                                } else {
                                    eprintln!("Invalid register in: {:?}", tokens);
                                }
                            } else {
                                eprintln!("Wrong number of operands for R-type: {:?}", tokens);
                            }
                        }

                        InstKind::IType { funct3 } => {
                            if tokens.len() == 4 {
                                let rd = reg_to_u8(&tokens[1]);
                                let rs1 = reg_to_u8(&tokens[2]);
                                let imm = tokens[3].parse::<i32>().ok();
                                if let (Some(rd), Some(rs1), Some(imm)) = (rd, rs1, imm) {
                                    let binary = encode_i_type(instr.opcode, *funct3, rd, rs1, imm);
                                    println!("{:032b}", binary);
                                } else {
                                    eprintln!("Invalid operands in I-type: {:?}", tokens);
                                }
                            } else {
                                eprintln!("Wrong number of operands for I-type: {:?}", tokens);
                            }
                        }

                        _ => {
                            println!("Instruction format not supported yet: {:?}", instr.kind);
                        }
                    }
                } else {
                    println!("Unknown mnemonic: {}", mnemonic);
                }
            }

            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}
