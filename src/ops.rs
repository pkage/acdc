use colored::Colorize;

use crate::stack::*;
use core::ops::*;
use std::fmt::Display;

pub type Operator<T> = fn(&mut Stack<T>) -> Option<T>;

pub struct Operation<T> {
    pub arity: usize,
    pub name:  String,
    pub desc:  String,
    pub func:  Operator<T>
}

/* --- OPERATOR DEFINITIONS --- */


fn operator_f<T>(stack: &mut Stack<T>) -> Option<T> where T: Copy + Display {
    for i in (0..stack.size()).rev() {
        println!("{:3} | {}", i, format!("{}", stack.contents[i]).blue());
    }

    println!(" -> {} items", format!("{}", stack.size()).blue());
    return None
}

fn operator_p<T>(stack: &mut Stack<T>) -> Option<T> where T: Copy + Display {
    println!("{}", format!("{}", stack.peek().unwrap()).blue());
    return None
}

fn operator_c<T>(stack: &mut Stack<T>) -> Option<T> where T: Copy {
    println!("stack cleared (dropped {} items).", format!("{}", stack.size()).blue());
    stack.contents = vec![];
    return None
}

fn operator_plus<T>(stack: &mut Stack<T>) -> Option<T> where T: Copy, T: Add, T: Add<Output = T> {
    let o1 = stack.pop().unwrap();
    let o2 = stack.pop().unwrap();

    stack.push(o1 + o2);

    return None
}

fn operator_minus<T>(stack: &mut Stack<T>) -> Option<T> where T: Copy, T: Sub, T: Sub<Output = T> {
    let o1 = stack.pop().unwrap();
    let o2 = stack.pop().unwrap();

    stack.push(o1 - o2);

    return None
}

fn operator_multiply<T>(stack: &mut Stack<T>) -> Option<T> where T: Copy, T: Mul, T: Mul<Output = T> {
    let o1 = stack.pop().unwrap();
    let o2 = stack.pop().unwrap();

    stack.push(o1 * o2);

    return None
}

fn operator_divide<T>(stack: &mut Stack<T>) -> Option<T> where T: Copy, T: Div, T: Div<Output = T>, T: PartialEq {
    let divisor  = stack.pop().unwrap();
    let dividend = stack.pop().unwrap();

    stack.push(dividend / divisor);

    return None
}

pub fn get_all_operations<T: 
        Copy + 
        Display + 
        PartialEq +
        Mul + 
        Mul<Output = T> + 
        Div + 
        Div<Output = T> + 
        Sub + 
        Sub<Output = T> +
        Add + 
        Add<Output = T> 
>() -> Vec<Operation<T>> {
    let op_l = Operation::<T> {
        arity: 0,
        name: "l".to_string(),
        desc: "print stack length".to_string(),
        func: |s: &mut Stack<T>| { println!("Stack has {} items", format!("{}", s.size()).blue()); None }
    };

    let op_f = Operation::<T> {
        arity: 0,
        name: "f".to_string(),
        desc: "print stack".to_string(),
        func: operator_f
    };

    let op_p = Operation::<T> {
        arity: 1,
        name: "p".to_string(),
        desc: "peek at top of stack".to_string(),
        func: operator_p
    };

    let op_c = Operation::<T> {
        arity: 0,
        name: "c".to_string(),
        desc: "drop stack".to_string(),
        func: operator_c
    };

    let op_plus = Operation::<T> {
        arity: 2,
        name: "+".to_string(),
        desc: "add the top two elements, push onto stack".to_string(),
        func: operator_plus
    };

    let op_minus = Operation::<T> {
        arity: 2,
        name: "-".to_string(),
        desc: "subtract the top two elements, push onto stack".to_string(),
        func: operator_minus
    };

    let op_multiply = Operation::<T> {
        arity: 2,
        name: "*".to_string(),
        desc: "multiply the top two elements, push onto stack".to_string(),
        func: operator_multiply
    };

    let op_divide = Operation::<T> {
        arity: 2,
        name: "/".to_string(),
        desc: "divide the top two elements, push onto stack".to_string(),
        func: operator_divide
    };

    return vec![
        op_l,
        op_f,
        op_p,
        op_c,
        op_plus,
        op_minus,
        op_multiply,
        op_divide
    ]
}
