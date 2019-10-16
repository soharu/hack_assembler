use code;
use regex::Regex;

pub fn binary_code_from(lines: Vec<&str>) -> Vec<String> {
    let filtered_lines: Vec<&str> = remove_all_white_space_and_comments(lines);
    let result: Vec<String> = filtered_lines
        .iter()
        .map(|line| build_instruction(line).to_bin())
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
    if Some('@') == line.chars().nth(0) {
        let value = line[1..].parse::<i16>().unwrap();
        return Box::new(AddressingValueInstruction { value: value });
    }
    return Box::new(ComputeInstruction::new(line));
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
    fn test_build_instruction() {
        assert_eq!("0000000000000010", build_instruction("@2").to_bin());
        assert_eq!("0000000010000101", build_instruction("@133").to_bin());
        assert_eq!("1110110000010000", build_instruction("D=A").to_bin());
        assert_eq!("1110000010010000", build_instruction("D=D+A").to_bin());
        assert_eq!("1110001100001000", build_instruction("M=D").to_bin());
        assert_eq!("1110001100000001", build_instruction("D;JGT").to_bin());
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
}
