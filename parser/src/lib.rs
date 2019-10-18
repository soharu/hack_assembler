use code;
use regex::Regex;
use symbol_table;

pub fn binary_code_from(lines: Vec<&str>) -> Vec<String> {
    let filtered_lines: Vec<&str> = remove_all_white_space_and_comments(lines);
    let symbol_table = build_symbol_table(&filtered_lines);
    let result: Vec<String> = filtered_lines
        .iter()
        .filter(|line| Some('(') != line.chars().nth(0))
        .map(|line| build_instruction(line, &symbol_table).to_bin())
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

fn build_symbol_table(lines: &Vec<&str>) -> symbol_table::SymbolTable {
    let mut symbol_table = symbol_table::SymbolTable::new();

    // 1st Pass
    let re = Regex::new(r"\((?P<label>.*)\)").unwrap();
    let mut address = 0;
    for line in lines.iter() {
        match re.captures(line) {
            Some(captures) => {
                let label = captures.name("label").map_or("", |m| m.as_str());
                if label.len() > 0 && symbol_table.contains(&label) == false {
                    symbol_table.add_entry(&label, address);
                }
            }
            None => {
                address += 1;
            }
        }
    }

    // 2nd Pass
    let mut address = 16;
    for line in lines.iter() {
        match extract_symbol_from(line) {
            Some(symbol) => {
                if symbol_table.contains(&symbol) == false {
                    symbol_table.add_entry(&symbol, address);
                    address += 1;
                }
            }
            None => {}
        }
    }

    return symbol_table;
}

fn extract_symbol_from(line: &str) -> Option<String> {
    if Some('@') != line.chars().nth(0) {
        return None;
    }

    let value_or_symbol = &line[1..];
    match value_or_symbol.parse::<i16>() {
        Ok(_) => None,
        Err(_) => {
            return Some(value_or_symbol.into());
        }
    }
}

fn build_instruction(
    line: &str,
    symbol_table: &symbol_table::SymbolTable,
) -> Box<dyn Instruction + 'static> {
    if Some('@') != line.chars().nth(0) {
        return Box::new(ComputeInstruction::new(line));
    }

    let value_or_symbol = &line[1..];
    match value_or_symbol.parse::<i16>() {
        Ok(value) => return Box::new(AddressingValueInstruction { value: value }),
        Err(_) => {
            let value = symbol_table.get_address(&value_or_symbol).unwrap();
            return Box::new(AddressingValueInstruction { value: value });
        }
    };
}

trait Instruction {
    fn to_bin(&self) -> String;
}

#[derive(PartialEq, Debug)]
struct AddressingValueInstruction {
    value: i16,
}

impl Instruction for AddressingValueInstruction {
    fn to_bin(&self) -> String {
        return format!("{:0>16b}", self.value);
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
    fn to_bin(&self) -> String {
        return code::to_bin(&self.dest, &self.comp, &self.jump);
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
        let symbol_table = build_symbol_table(&lines);
        assert_eq!(Some(16), symbol_table.get_address("i"));
        assert_eq!(Some(17), symbol_table.get_address("sum"));
        assert_eq!(Some(1), symbol_table.get_address("R1"));
    }

    #[test]
    fn test_extract_symbol_from_a_single_line() {
        assert_eq!(None, extract_symbol_from("@2"));
        assert_eq!(Some("SP".into()), extract_symbol_from("@SP"));
        assert_eq!(Some("i".into()), extract_symbol_from("@i"));
    }

    #[test]
    fn test_build_instruction() {
        let symbol_table = symbol_table::SymbolTable::new();
        assert_eq!(
            "0000000000000010",
            build_instruction("@2", &symbol_table).to_bin()
        );
        assert_eq!(
            "0000000010000101",
            build_instruction("@133", &symbol_table).to_bin()
        );
        assert_eq!(
            "1110110000010000",
            build_instruction("D=A", &symbol_table).to_bin()
        );
        assert_eq!(
            "1110000010010000",
            build_instruction("D=D+A", &symbol_table).to_bin()
        );
        assert_eq!(
            "1110001100001000",
            build_instruction("M=D", &symbol_table).to_bin()
        );
        assert_eq!(
            "1110001100000001",
            build_instruction("D;JGT", &symbol_table).to_bin()
        );
        assert_eq!(
            "0000000000000000",
            build_instruction("@SP", &symbol_table).to_bin()
        );
        assert_eq!(
            "0000000000001111",
            build_instruction("@R15", &symbol_table).to_bin()
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
