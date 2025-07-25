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

use std::{fs::File, io::BufRead, io::BufReader, io::Write};

struct InstFmt {
    opcode: u8,
    kind: InstKind,
}


fn get_instruction_map() -> HashMap<&'static str, InstFmt> {
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

    // S-Type instructions
    map.insert("sb",InstFmt {
            opcode: 0b0100011,
            kind: InstKind::SType { funct3: 0b000 },
        },
    );
    map.insert("sh",InstFmt {
            opcode: 0b0100011,
            kind: InstKind::SType { funct3: 0b001 },
        },
    );
    map.insert("sw",InstFmt {
            opcode: 0b0100011,
            kind: InstKind::SType { funct3: 0b010 },
        },
    );

    // B-Type (Branches)
    map.insert("beq",  InstFmt {
            opcode: 0b1100011,
            kind: InstKind::BType { funct3: 0b000 }, 
        },
    );
    map.insert("bne",  InstFmt {
            opcode: 0b1100011,
            kind: InstKind::BType { funct3: 0b001 },
        },
    );
    map.insert("blt",  InstFmt {
            opcode: 0b1100011,
            kind: InstKind::BType { funct3: 0b100 },
        },
    );
    map.insert("bge",  InstFmt {
            opcode: 0b1100011,
            kind: InstKind::BType { funct3: 0b101 },
        },
    );
    map.insert("bltu", InstFmt {
            opcode: 0b1100011,
            kind: InstKind::BType { funct3: 0b110 },
        },
    );
    map.insert("bgeu", InstFmt {
            opcode: 0b1100011,
            kind: InstKind::BType { funct3: 0b111 },
        },
    );

    // U-Type
    map.insert("lui",   InstFmt {
            opcode: 0b0110111,
            kind: InstKind::UType,
        },
    );
    map.insert("auipc", InstFmt {
            opcode: 0b0010111,
            kind: InstKind::UType,
        },
    );

    // J-Type
    map.insert("jal", InstFmt {
            opcode: 0b1101111,
            kind: InstKind::JType,
        },
    );

    map
}

fn reg_to_u8(reg: &str) -> Option<u8> {
    match reg {
        "zero" => Some(0),
        "ra" => Some(1),
        "sp" => Some(2),
        "gp" => Some(3),
        "tp" => Some(4),
        "t0" => Some(5),
        "t1" => Some(6),
        "t2" => Some(7),
        "s0" | "fp" => Some(8),
        "s1" => Some(9),
        "a0" => Some(10),
        "a1" => Some(11),
        "a2" => Some(12),
        "a3" => Some(13),
        "a4" => Some(14),
        "a5" => Some(15),
        "a6" => Some(16),
        "a7" => Some(17),
        "s2" => Some(18),
        "s3" => Some(19),
        "s4" => Some(20),
        "s5" => Some(21),
        "s6" => Some(22),
        "s7" => Some(23),
        "s8" => Some(24),
        "s9" => Some(25),
        "s10" => Some(26),
        "s11" => Some(27),
        "t3" => Some(28),
        "t4" => Some(29),
        "t5" => Some(30),
        "t6" => Some(31),
        _ if reg.starts_with('x') => {
            reg[1..].parse::<u8>().ok().filter(|&n| n < 32)
        }
        _ => None,
    }
}

fn parse_s_type_operands(rs2_token: &str, offset_base: &str) -> Option<(u8, u8, i32)> {
    //func to parse base+offset address
    let rs2 = reg_to_u8(rs2_token)?;
    let open_paren = offset_base.find('(')?;
    let close_paren = offset_base.find(')')?;

    let imm_str = &offset_base[..open_paren];
    let base_reg_str = &offset_base[open_paren + 1..close_paren];

    let imm = imm_str.trim().parse::<i32>().ok()?;
    let rs1 = reg_to_u8(base_reg_str.trim())?;

    Some((rs1, rs2, imm))
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

fn encode_s_type(opcode: u8, funct3: u8, rs1: u8, rs2: u8, imm: i32) -> u32 {
    let imm_u = imm as u32;
    let imm_4_0 = imm_u & 0b11111;            // bits 4:0
    let imm_11_5 = (imm_u >> 5) & 0b1111111;  // bits 11:5

    (imm_11_5 << 25)
        | ((rs2 as u32) << 20)
        | ((rs1 as u32) << 15)
        | ((funct3 as u32) << 12)
        | (imm_4_0 << 7)
        | (opcode as u32)
}

fn encode_b_type(opcode: u8, funct3: u8, rs1: u8, rs2: u8, offset: i32) -> u32 {
    let imm = offset as u32;

    let imm_12   = (imm >> 12) & 0x1;
    let imm_10_5 = (imm >> 5)  & 0x3F;
    let imm_4_1  = (imm >> 1)  & 0xF;
    let imm_11   = (imm >> 11) & 0x1;

    (imm_12 << 31)
        | (imm_10_5 << 25)
        | ((rs2 as u32) << 20)
        | ((rs1 as u32) << 15)
        | ((funct3 as u32) << 12)
        | (imm_4_1 << 8)
        | (imm_11 << 7)
        | (opcode as u32)
}

fn encode_j_type(opcode: u8, rd: u8, offset: i32) -> u32 {
    let imm = offset as u32;

    let imm_20 = (imm >> 20) & 0x1;
    let imm_10_1 = (imm >> 1) & 0x3FF;
    let imm_11 = (imm >> 11) & 0x1;
    let imm_19_12 = (imm >> 12) & 0xFF;

    (imm_20 << 31)
        | (imm_19_12 << 12)
        | (imm_11 << 20)
        | (imm_10_1 << 21)
        | ((rd as u32) << 7)
        | (opcode as u32)
}

fn encode_u_type(opcode: u8, rd: u8, imm: u32) -> u32 {
    (imm << 12) | ((rd as u32) << 7) | (opcode as u32)
}

fn resolve_labels(contents: BufReader<File>) -> (HashMap<String, u32>, Vec<(u32, String)>) {
    let mut label_map = HashMap::new();
    let mut instructions = Vec::new();
    let mut pc = 0u32;

    for line_result in contents.lines() {
        if let Ok(line) = line_result {
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.ends_with(':') {
                let label = trimmed.trim_end_matches(':').to_string();
                label_map.insert(label, pc);
            } else if trimmed.starts_with('.') {
                continue;
            } else {
                instructions.push((pc, trimmed.to_string()));
                pc += 4;
            }
        }
    }

    (label_map, instructions)
}


pub fn asm_parser(contents: BufReader<File>, output_file_name:String) {
    let (label_map, instructions) = resolve_labels(contents);
    
    let mut binary: Vec<u8> = Vec::new();
    
    for (pc, line) in instructions {
        let tokens: Vec<String> = line
            .split('#').next().unwrap_or("")
            .split_whitespace()
            .map(|t| t.trim_end_matches(',').to_string())
            .collect();

        if tokens.is_empty() {
            continue;
        }

        let mnemonic = &tokens[0];

        if let Some(instr) = get_instruction_map().get(mnemonic.as_str()) {
            let encoded: Option<u32> = match &instr.kind {
                InstKind::RType { funct3, funct7 } => {
                    if tokens.len() == 4 {
                        let rd = reg_to_u8(&tokens[1]);
                        let rs1 = reg_to_u8(&tokens[2]);
                        let rs2 = reg_to_u8(&tokens[3]);
                        if let (Some(rd), Some(rs1), Some(rs2)) = (rd, rs1, rs2) {
                            Some(encode_r_type(instr.opcode, *funct3, *funct7, rd, rs1, rs2))
                        } else {
                            eprintln!("Invalid register in: {:?}", tokens);
                            None
                        }
                    } else {
                        eprintln!("Wrong number of operands for R-type: {:?}", tokens);
                        None
                    }
                }

                InstKind::IType { funct3 } => {
                    if tokens.len() == 4 {
                        let rd = reg_to_u8(&tokens[1]);
                        let rs1 = reg_to_u8(&tokens[2]);
                        let imm = tokens[3].parse::<i32>().ok();
                        if let (Some(rd), Some(rs1), Some(imm)) = (rd, rs1, imm) {
                            Some(encode_i_type(instr.opcode, *funct3, rd, rs1, imm))
                        } else {
                            eprintln!("Invalid operands in I-type: {:?}", tokens);
                            None
                        }
                    } else {
                        eprintln!("Wrong number of operands for I-type: {:?}", tokens);
                        None
                    }
                }

                InstKind::SType { funct3 } => {
                    if tokens.len() == 3 {
                        if let Some((rs1, rs2, imm)) = parse_s_type_operands(&tokens[1], &tokens[2]) {
                            Some(encode_s_type(instr.opcode, *funct3, rs1, rs2, imm))
                        } else {
                            eprintln!("Invalid S-type operands: {:?}", tokens);
                            None
                        }
                    } else {
                        eprintln!("Wrong number of operands for S-type: {:?}", tokens);
                        None
                    }
                }

                InstKind::BType { funct3 } => {
                    if tokens.len() == 4 {
                        let rs1 = reg_to_u8(&tokens[1]);
                        let rs2 = reg_to_u8(&tokens[2]);
                        let label = &tokens[3];

                        if let (Some(rs1), Some(rs2)) = (rs1, rs2) {
                            if let Some(&target_addr) = label_map.get(label) {
                                let offset = target_addr as i32 - pc as i32;
                                Some(encode_b_type(instr.opcode, *funct3, rs1, rs2, offset))
                            } else {
                                eprintln!("Label '{}' not found", label);
                                None
                            }
                        } else {
                            eprintln!("Invalid registers in B-type: {:?}", tokens);
                            None
                        }
                    } else {
                        eprintln!("Wrong number of operands for B-type: {:?}", tokens);
                        None
                    }
                }

                InstKind::UType => {
                    if tokens.len() == 3 {
                        let rd = reg_to_u8(&tokens[1]);
                        let imm = tokens[2].parse::<u32>();

                        if let (Some(rd), Ok(imm)) = (rd, imm) {
                            if imm > 0xFFFFF {
                                eprintln!("Immediate too large for U-type: {}", imm);
                                None
                            } else {
                                Some(encode_u_type(instr.opcode, rd, imm))
                            }
                        } else {
                            eprintln!("Invalid U-type operands: {:?}", tokens);
                            None
                        }
                    } else {
                        eprintln!("U-type format: {} rd, imm", tokens[0]);
                        None
                    }
                }

                InstKind::JType => {
                    if tokens.len() == 3 {
                        let rd = reg_to_u8(&tokens[1]);
                        let label = &tokens[2];
                        if let Some(rd) = rd {
                            if let Some(&target_addr) = label_map.get(label) {
                                let offset = target_addr as i32 - pc as i32;
                                if offset % 2 != 0 {
                                    eprintln!("Unaligned offset for label '{}'", label);
                                    None
                                } else {
                                    Some(encode_j_type(instr.opcode, rd, offset))
                                }
                            } else {
                                eprintln!("Unknown label '{}'", label);
                                None
                            }
                        } else {
                            eprintln!("Invalid register '{}'", tokens[1]);
                            None
                        }
                    } else {
                        eprintln!("J-type format should be: jal rd, label");
                        None
                    }
                }
            };

            if let Some(instr_bin) = encoded{
                binary.extend(&instr_bin.to_le_bytes());
            }
        } else {
            println!("Unknown mnemonic: {}", mnemonic);
        }
    }

    let mut file = File::create(&output_file_name).expect("Failed to create program.bin");
    file.write_all(&binary).expect("Failed to write binary data");

    println!("Saved output to {}", output_file_name);
}
