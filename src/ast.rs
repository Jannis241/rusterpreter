use core::num;
use std::collections::btree_map::Values;

use crate::*;


pub fn eval_print(node : Node) -> EvalValue {
    let nodes_in_the_print_statement = node.value;
    let evaluated_nodes: Vec<EvalValue> = nodes_in_the_print_statement.iter().map(|node| node.eval()).collect();

    println!("{:?}",evaluated_nodes);

    EvalValue::None
}


pub fn eval_add(node: Node) -> EvalValue {
    // Todo: Support for floating point numbers
    // Todo: checken ob token None ist weil dann weiß man dass es zb ein plus nodei st mit mehreren ints und n
}

pub fn eval_int(node: Node) -> EvalValue {
    println!("node: {:?}", node);
    if let Some(token) = node.value[0].token.clone() {
        if let Ok(value) = token.value.parse::<i64>() {
            println!("evaluated int: {}", value);
            return EvalValue::Int(value);
        }
    }
    EvalValue::None
}

pub fn eval_multiply(node : Node) -> EvalValue {
    return  EvalValue::Int(0);

}
pub fn eval_divide(node : Node) -> EvalValue {
    return  EvalValue::Int(0);

}
pub fn eval_subtract(node : Node) -> EvalValue {

    return  EvalValue::Int(0);
}
