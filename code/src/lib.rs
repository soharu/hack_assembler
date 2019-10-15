// Translate Hack assembly mnemonics into binary code

pub fn to_bin(dest: &str, comp: &str, jump: &str) -> String {
    let mut result = String::new();
    result.push_str("111");
    result.push_str(comp_to_bin(comp));
    result.push_str(dest_to_bin(dest));
    result.push_str(jump_to_bin(jump));
    return result;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bin() {
        assert_eq!("1110110000010000", to_bin("D", "A", ""));
        assert_eq!("1110000010010000", to_bin("D", "D+A", ""));
        assert_eq!("1110001100001000", to_bin("M", "D", ""));
        assert_eq!("1110001100000001", to_bin("", "D", "JGT"))
    }
}
