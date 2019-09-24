fn main() {
    println!("Hello, world!");
}

fn code_to_bin(code: &str) -> String {
    if Some('@') == code.chars().nth(0) {
        let value = code[1..].parse::<i16>().unwrap();
        return format!("{:0>16b}", value);
    } else {
        return code.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_to_bin() {
        assert_eq!("0000000000000010", code_to_bin("@2"));
        assert_eq!("0000000010000101", code_to_bin("@133"));
    }
}
