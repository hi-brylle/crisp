use std::collections::HashSet;

use crate::symbol_table::{self, Symbol, SymbolTable};



pub fn check_redeclarations(symbol_table: &SymbolTable) -> Vec<Symbol>  {
    let mut redeclared_symbols: Vec<Symbol> = vec![];
    let mut temp: HashSet<String> = HashSet::new();
    let symbol_table = &symbol_table.symbol_table;
    
    // Collect redeclared symbols, ignoring original declarations.
    for symbol in symbol_table {
        if !temp.insert(symbol.scope_address.clone()) {
            redeclared_symbols.push(symbol.clone());
        }
    }
    
    redeclared_symbols
}