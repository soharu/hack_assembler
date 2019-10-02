fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Command {
    AType { value: i16 },
    CType { s: String }
}

fn build_command(line: &str) -> Command {
    if Some('@') == line.chars().nth(0) {
        let value = line[1..].parse::<i16>().unwrap();
        return Command::AType { value: value }
    } else {
        return Command::CType { s: line.to_string() }
    }
}

fn code_to_bin(code: &str) -> String {
    let command = build_command(code);
    match command {
        Command::AType { value } => format!("{:0>16b}", value),
        Command::CType { s } => s,
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
    fn test_code_to_bin() {
        assert_eq!("0000000000000010", code_to_bin("@2"));
        assert_eq!("0000000010000101", code_to_bin("@133"));
    }
}
