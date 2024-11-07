use crate::{ast2::*, Rule};

pub fn build_program_ast(pair: pest::iterators::Pair<Rule>) -> Program {
    debug_pair(&pair);

    todo!();
}

fn debug_pair(pair: &pest::iterators::Pair<Rule>) {
    println!("Rule:    {:?}", pair.as_rule());
    println!("Span:    {:?}", pair.as_span());
    println!("Text:    {}\n", pair.as_str());
}