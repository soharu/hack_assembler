use code;
use regex::Regex;
use symbol_table;

pub fn binary_code_from(lines: Vec<&str>) -> Vec<String> {
    let filtered_lines: Vec<&str> = remove_all_white_space_and_comments(lines);
    let instructions: Vec<Box<dyn Instruction + 'static>> = filtered_lines
        .iter()
        .map(|line| build_instruction(line))
        .collect();
    let symbol_table = build_symbol_table(&instructions);
    let result: Vec<String> = instructions
        .iter()
        .map(|instruction| instruction.to_bin(&symbol_table))
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect();
    return result;
}

fn remove_all_white_space_and_comments(lines: Vec<&str>) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    for line in lines {
        let subs: Vec<&str> = line.split("//").collect();
        let trimmed = subs[0].trim();
        if !trimmed.is_empty() {
            result.push(trimmed);
        }
    }
    return result;
}

fn build_instruction(line: &str) -> Box<dyn Instruction + 'static> {
    match AddressingValueInstruction::new(line) {
        Some(instruction) => return Box::new(instruction),
        None => {}
    }

    match AddressingSymbolInstruction::new(line) {
        Some(instruction) => return Box::new(instruction),
        None => {}
    }

    match LabelInstruction::new(line) {
        Some(instruction) => return Box::new(instruction),
        None => {}
    }

    return Box::new(ComputeInstruction::new(line));
}

fn build_symbol_table(
    instructions: &Vec<Box<dyn Instruction + 'static>>,
) -> symbol_table::SymbolTable {
    let mut symbol_table = symbol_table::SymbolTable::new();

    // 1st Pass
    let mut rom_address = 0;
    for instruction in instructions.iter() {
        match instruction.instruction_type() {
            InstructionType::Label(label) => {
                symbol_table.add_entry(&label, rom_address);
            }
            _ => {
                rom_address += 1;
            }
        }
    }

    // 2nd Pass
    let mut ram_address = 16;
    for instruction in instructions.iter() {
        match instruction.instruction_type() {
            InstructionType::AddressSymbol(symbol) => {
                if symbol_table.contains(&symbol) == false {
                    symbol_table.add_entry(&symbol, ram_address);
                    ram_address += 1;
                }
            }
            _ => {}
        }
    }

    return symbol_table;
}

enum InstructionType {
    AddressValue,
    AddressSymbol(String),
    Label(String),
    Compute,
}

trait Instruction {
    fn to_bin(&self, symbol_table: &symbol_table::SymbolTable) -> Option<String>;
    fn instruction_type(&self) -> InstructionType;
}

#[derive(PartialEq, Debug)]
struct AddressingValueInstruction {
    value: i16,
}

impl AddressingValueInstruction {
    fn new(line: &str) -> Option<AddressingValueInstruction> {
        if Some('@') != line.chars().nth(0) {
            return None;
        }
        let value = &line[1..];
        match value.parse::<i16>() {
            Ok(value) => return Some(AddressingValueInstruction { value: value }),
            Err(_) => return None,
        };
    }
}

impl Instruction for AddressingValueInstruction {
    fn to_bin(&self, _symbol_table: &symbol_table::SymbolTable) -> Option<String> {
        return Some(format!("{:0>16b}", self.value));
    }

    fn instruction_type(&self) -> InstructionType {
        return InstructionType::AddressValue;
    }
}

#[derive(PartialEq, Debug)]
struct AddressingSymbolInstruction {
    symbol: String,
}

impl AddressingSymbolInstruction {
    fn new(line: &str) -> Option<AddressingSymbolInstruction> {
        if Some('@') != line.chars().nth(0) {
            return None;
        }
        let symbol = &line[1..];
        return Some(AddressingSymbolInstruction {
            symbol: symbol.into(),
        });
    }
}

impl Instruction for AddressingSymbolInstruction {
    fn to_bin(&self, symbol_table: &symbol_table::SymbolTable) -> Option<String> {
        match symbol_table.get_address(&self.symbol) {
            Some(value) => {
                return Some(format!("{:0>16b}", value));
            }
            None => {
                return None;
            }
        }
    }

    fn instruction_type(&self) -> InstructionType {
        return InstructionType::AddressSymbol(self.symbol.clone());
    }
}

#[derive(PartialEq, Debug)]
struct LabelInstruction {
    label: String,
}

impl LabelInstruction {
    fn new(line: &str) -> Option<LabelInstruction> {
        let re = Regex::new(r"\((?P<label>.*)\)").unwrap();
        match re.captures(line) {
            Some(captures) => {
                let label = captures.name("label").map_or("", |m| m.as_str());
                return Some(LabelInstruction {
                    label: label.into(),
                });
            }
            None => None,
        }
    }
}

impl Instruction for LabelInstruction {
    fn to_bin(&self, _symbol_table: &symbol_table::SymbolTable) -> Option<String> {
        return None;
    }

    fn instruction_type(&self) -> InstructionType {
        return InstructionType::Label(self.label.clone());
    }
}

#[derive(PartialEq, Debug)]
struct ComputeInstruction {
    dest: String,
    comp: String,
    jump: String,
}

impl ComputeInstruction {
    fn new(line: &str) -> ComputeInstruction {
        let re = Regex::new(r"^((?P<dest>[AMD]*)=)?(?P<comp>[^;]*)(;(?P<jump>\w{3}))?$").unwrap();
        let captures = re.captures(line).unwrap();
        let dest = captures
            .name("dest")
            .map_or("".into(), |m| m.as_str().into());
        let comp = captures
            .name("comp")
            .map_or("".into(), |m| m.as_str().into());
        let jump = captures
            .name("jump")
            .map_or("".into(), |m| m.as_str().into());
        return ComputeInstruction {
            dest: dest,
            comp: comp,
            jump: jump,
        };
    }
}

impl Instruction for ComputeInstruction {
    fn to_bin(&self, _symbol_table: &symbol_table::SymbolTable) -> Option<String> {
        return Some(code::to_bin(&self.dest, &self.comp, &self.jump));
    }

    fn instruction_type(&self) -> InstructionType {
        return InstructionType::Compute;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_all_white_space_and_comments() {
        let comments = vec![
            "// this",
            "\n\n",
            "// is",
            "// a comment.",
            "\n",
            "@2",
            "D=M",
            "    M=D+M  // i am here",
        ];
        let actual = remove_all_white_space_and_comments(comments);
        assert_eq!(actual, ["@2", "D=M", "M=D+M"]);
    }

    #[test]
    fn test_compute_instruction() {
        assert_eq!(
            ComputeInstruction {
                dest: "D".into(),
                comp: "A".into(),
                jump: "".into(),
            },
            ComputeInstruction::new("D=A")
        );
        assert_eq!(
            ComputeInstruction {
                dest: "AM".into(),
                comp: "M-1".into(),
                jump: "".into(),
            },
            ComputeInstruction::new("AM=M-1")
        );
        assert_eq!(
            ComputeInstruction {
                dest: "".into(),
                comp: "0".into(),
                jump: "JEQ".into(),
            },
            ComputeInstruction::new("0;JEQ")
        );
    }

    #[test]
    fn test_build_symbol_table() {
        let lines = vec!["@2", "@i", "@sum", "@R1"];
        let instructions: Vec<Box<dyn Instruction + 'static>> =
            lines.iter().map(|line| build_instruction(line)).collect();
        let symbol_table = build_symbol_table(&instructions);
        assert_eq!(Some(16), symbol_table.get_address("i"));
        assert_eq!(Some(17), symbol_table.get_address("sum"));
        assert_eq!(Some(1), symbol_table.get_address("R1"));
    }

    #[test]
    fn test_build_instruction() {
        let symbol_table = symbol_table::SymbolTable::new();
        assert_eq!(
            "0000000000000010",
            build_instruction("@2").to_bin(&symbol_table).unwrap()
        );
        assert_eq!(
            "0000000010000101",
            build_instruction("@133").to_bin(&symbol_table).unwrap()
        );
        assert_eq!(
            "1110110000010000",
            build_instruction("D=A").to_bin(&symbol_table).unwrap()
        );
        assert_eq!(
            "1110000010010000",
            build_instruction("D=D+A").to_bin(&symbol_table).unwrap()
        );
        assert_eq!(
            "1110001100001000",
            build_instruction("M=D").to_bin(&symbol_table).unwrap()
        );
        assert_eq!(
            "1110001100000001",
            build_instruction("D;JGT").to_bin(&symbol_table).unwrap()
        );
        assert_eq!(
            "0000000000000000",
            build_instruction("@SP").to_bin(&symbol_table).unwrap()
        );
        assert_eq!(
            "0000000000001111",
            build_instruction("@R15").to_bin(&symbol_table).unwrap()
        );
    }

    #[test]
    fn test_binary_code_from_assembly_code() {
        let lines = vec![
            "// this",
            "\n\n",
            "// is",
            "// a comment.",
            "\n",
            "@2",
            "D=A",
            "@3",
            "D=D+A",
            "@0",
            "M=D",
        ];
        let expected = vec![
            "0000000000000010",
            "1110110000010000",
            "0000000000000011",
            "1110000010010000",
            "0000000000000000",
            "1110001100001000",
        ];

        let actual = binary_code_from(lines);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_binary_code_from_assembly_code_with_symbol() {
        let lines = vec![
            "// this",
            "\n\n",
            "// is",
            "// a comment.",
            "\n",
            "@0",
            "D=M",
            "@LOOP",
            "D;JLE",
            "(LOOP)",
            "M=D",
            "@counter",
        ];
        let expected = vec![
            "0000000000000000", // @0
            "1111110000010000", // D=M
            "0000000000000100", // @LOOP
            "1110001100000110", // D;JLE
            "1110001100001000", // M=D
            "0000000000010000", // @counter
        ];

        let actual = binary_code_from(lines);
        assert_eq!(expected, actual);
    }
}
