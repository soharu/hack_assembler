use std::collections::HashMap;

pub struct SymbolTable {
    symbol_map: HashMap<String, i16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut symbol_table = SymbolTable {
            symbol_map: HashMap::new(),
        };
        symbol_table.add_predefined_symbols();
        symbol_table
    }

    fn add_predefined_symbols(&mut self) {
        self.add_entry("SP", 0);
        self.add_entry("LCL", 1);
        self.add_entry("ARG", 2);
        self.add_entry("THIS", 3);
        self.add_entry("THAT", 4);

        for i in 0..16 {
            let register = format!("R{}", i);
            self.add_entry(&register, i);
        }

        self.add_entry("SCREEN", 16384);
        self.add_entry("KBD", 24576);
    }

    pub fn add_entry(&mut self, symbol: &str, address: i16) {
        self.symbol_map.insert(String::from(symbol), address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.symbol_map.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<i16> {
        self.symbol_map
            .get(symbol)
            .map_or(None, |v| Some(v.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_entry("EXIT", 0);

        assert_eq!(true, symbol_table.contains("EXIT"));
        assert_eq!(Some(0), symbol_table.get_address("EXIT"));

        assert_eq!(false, symbol_table.contains("ABC"));
        assert_eq!(None, symbol_table.get_address("ABC"));
    }

    #[test]
    fn test_symbol_table_contains_predefined_symbols() {
        let symbol_table = SymbolTable::new();
        assert_eq!(Some(0), symbol_table.get_address("SP"));
        assert_eq!(Some(1), symbol_table.get_address("LCL"));
        assert_eq!(Some(2), symbol_table.get_address("ARG"));
        assert_eq!(Some(3), symbol_table.get_address("THIS"));
        assert_eq!(Some(4), symbol_table.get_address("THAT"));

        assert_eq!(Some(0), symbol_table.get_address("R0"));
        assert_eq!(Some(1), symbol_table.get_address("R1"));
        assert_eq!(Some(2), symbol_table.get_address("R2"));
        assert_eq!(Some(3), symbol_table.get_address("R3"));
        assert_eq!(Some(4), symbol_table.get_address("R4"));
        assert_eq!(Some(5), symbol_table.get_address("R5"));
        assert_eq!(Some(6), symbol_table.get_address("R6"));
        assert_eq!(Some(7), symbol_table.get_address("R7"));
        assert_eq!(Some(8), symbol_table.get_address("R8"));
        assert_eq!(Some(9), symbol_table.get_address("R9"));
        assert_eq!(Some(10), symbol_table.get_address("R10"));
        assert_eq!(Some(11), symbol_table.get_address("R11"));
        assert_eq!(Some(12), symbol_table.get_address("R12"));
        assert_eq!(Some(13), symbol_table.get_address("R13"));
        assert_eq!(Some(14), symbol_table.get_address("R14"));
        assert_eq!(Some(15), symbol_table.get_address("R15"));

        assert_eq!(Some(16384), symbol_table.get_address("SCREEN"));
        assert_eq!(Some(24576), symbol_table.get_address("KBD"));
    }
}
