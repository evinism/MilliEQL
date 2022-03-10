use pest::iterators::Pair;

use crate::{Result, Rule};

mod array;
mod expr;
mod function;
mod infix;
mod object;
mod prefix;
mod value;

pub fn eval(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    expr::eval(pair, context)
}