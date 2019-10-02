use regex::Regex;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
enum Command {
    AType {
        value: i16,
    },

    // dest=comp;jump
    CType {
        dest: String,
        comp: String,
        jump: String,
    },
}

fn build_command(line: &str) -> Command {
    if Some('@') == line.chars().nth(0) {
        let value = line[1..].parse::<i16>().unwrap();
        return Command::AType { value: value };
    }
    let re = Regex::new(r"^((?P<dest>[AMD]*)=)?(?P<comp>[^;]*)(;(?P<jump>\w{3}))?$").unwrap();
    let captures = re.captures(line).unwrap();
    return Command::CType {
        dest: captures
            .name("dest")
            .map_or("".to_string(), |m| m.as_str().to_string()),
        comp: captures["comp"].to_string(),
        jump: captures
            .name("jump")
            .map_or("".to_string(), |m| m.as_str().to_string()),
    };
}

fn dest_to_bin(dest: &str) -> &str {
    match dest {
        ""    => "000",
        "M"   => "001",
        "D"   => "010",
        "MD"  => "011",
        "A"   => "100",
        "AM"  => "101",
        "AD"  => "110",
        "AMD" => "111",
        _ => "",
    }
}

fn comp_to_bin(comp: &str) -> &str {
    match comp {
        "0"   => "0101010",
        "1"   => "0111111",
        "-1"  => "0111010",
        "D"   => "0001100",
        "A"   => "0110000",
        "!D"  => "0001101",
        "!A"  => "0110001",
        "-D"  => "0001111",
        "-A"  => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M"   => "1110000",
        "!M"  => "1110001",
        "-M"  => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => ""
    }
}

fn jump_to_bin(jump: &str) -> &str {
    match jump {
        ""    => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "",
    }
}

fn c_command_to_bin(dest: String, comp: String, jump: String) -> String {
    let mut result = String::new();
    result.push_str("111");
    result.push_str(comp_to_bin(&comp));
    result.push_str(dest_to_bin(&dest));
    result.push_str(jump_to_bin(&jump));
    return result;
}

fn code_to_bin(code: &str) -> String {
    let command = build_command(code);
    match command {
        Command::AType { value } => format!("{:0>16b}", value),
        Command::CType { dest, comp, jump } => c_command_to_bin(dest, comp, jump),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command_to_a_type() {
        assert_eq!(Command::AType { value: 2 }, build_command("@2"));
        assert_eq!(Command::AType { value: 133 }, build_command("@133"));
    }

    #[test]
    fn test_build_command_to_c_type() {
        assert_eq!(
            Command::CType {
                dest: "D".to_string(),
                comp: "A".to_string(),
                jump: "".to_string(),
            },
            build_command("D=A")
        );
        assert_eq!(
            Command::CType {
                dest: "AM".to_string(),
                comp: "M-1".to_string(),
                jump: "".to_string(),
            },
            build_command("AM=M-1")
        );
        assert_eq!(
            Command::CType {
                dest: "".to_string(),
                comp: "0".to_string(),
                jump: "JEQ".to_string(),
            },
            build_command("0;JEQ")
        );
    }

    #[test]
    fn test_code_to_bin() {
        assert_eq!("0000000000000010", code_to_bin("@2"));
        assert_eq!("0000000010000101", code_to_bin("@133"));
        assert_eq!("1110110000010000", code_to_bin("D=A"));
        assert_eq!("1110000010010000", code_to_bin("D=D+A"));
        assert_eq!("1110001100001000", code_to_bin("M=D"));
        assert_eq!("1110001100000001", code_to_bin("D;JGT"));
    }
}
