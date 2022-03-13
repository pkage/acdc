use colored::Colorize;

use crate::stack::*;

pub type Operator = fn(&mut Stack) -> ();

pub struct Operation {
    pub arity: usize,
    pub name:  String,
    pub desc:  String,
    pub func:  Operator
}

/* --- OPERATOR DEFINITIONS --- */


fn operator_f(stack: &mut Stack) -> () {
    for i in (0..stack.size()).rev() {
        println!("{:3} | {}", i, format!("{}", stack.contents[i]).blue());
    }

    println!(" -> {} items", format!("{}", stack.size()).blue());
}

fn operator_p(stack: &mut Stack) -> () {
    println!("{}", format!("{}", stack.peek().unwrap()).blue());
}

fn operator_d(stack: &mut Stack) -> () {
    let num = stack.peek().unwrap();
    stack.push(num);
    println!(" -> added {} to stack", format!("{}", num).blue());
}

fn operator_c(stack: &mut Stack) -> () {
    println!(" -> dropped {} items (stack cleared).", format!("{}", stack.size()).blue());
    stack.contents = vec![];
}

fn operator_capital_p(stack: &mut Stack) -> () {
    operator_p(stack);
    println!(" -> popped {} item.", format!("{}", 1).blue());
    stack.pop().unwrap();
}

fn operator_plus(stack: &mut Stack) -> () {
    let o1 = stack.pop().unwrap();
    let o2 = stack.pop().unwrap();

    stack.push(o1 + o2);

}

fn operator_minus(stack: &mut Stack) -> () {
    let o1 = stack.pop().unwrap();
    let o2 = stack.pop().unwrap();

    stack.push(o1 - o2);
}

fn operator_multiply(stack: &mut Stack) -> () {
    let o1 = stack.pop().unwrap();
    let o2 = stack.pop().unwrap();

    stack.push(o1 * o2);
}

fn operator_divide(stack: &mut Stack) -> () {
    let divisor  = stack.pop().unwrap();
    let quotient = stack.pop().unwrap();

    if divisor == 0.0 {
        println!("{}", "warning: dividing by zero".yellow())
    }

    stack.push(quotient / divisor);
}

fn operator_power(stack: &mut Stack) -> () {
    let power = stack.pop().unwrap();
    let number = stack.pop().unwrap();

    stack.push(number.powf(power))
}

fn operator_root(stack: &mut Stack) -> () {
    let power = stack.pop().unwrap();
    let number = stack.pop().unwrap();

    stack.push(number.powf(1.0 / power))
}

fn operator_log(stack: &mut Stack) -> () {
    let base = stack.pop().unwrap();
    let number = stack.pop().unwrap();

    if base <= 0.0 {
        println!("{}", "warning: negative base for log!".yellow())
    }

    stack.push(number.log(base));
}

fn operator_const_pi(stack: &mut Stack) -> () {
    stack.push(std::f64::consts::PI)
}
fn operator_const_tau(stack: &mut Stack) -> () {
    stack.push(std::f64::consts::TAU)
}

fn operator_const_e(stack: &mut Stack) -> () {
    stack.push(std::f64::consts::E)
}

pub fn get_all_operations() -> Vec<Operation> {
    let op_l = Operation {
        arity: 0,
        name: "l".to_string(),
        desc: "print stack length".to_string(),
        func: |s: &mut Stack| println!("Stack has {} items", format!("{}", s.size()).blue())
    };

    let op_f = Operation {
        arity: 0,
        name: "f".to_string(),
        desc: "print stack".to_string(),
        func: operator_f
    };

    let op_p = Operation {
        arity: 1,
        name: "p".to_string(),
        desc: "peek at top of stack".to_string(),
        func: operator_p
    };

    let op_d = Operation {
        arity: 1,
        name: "d".to_string(),
        desc: "duplicate top of stack".to_string(),
        func: operator_d
    };

    let op_capital_p = Operation {
        arity: 1,
        name: "P".to_string(),
        desc: "pop top of stack".to_string(),
        func: operator_capital_p
    };

    let op_c = Operation {
        arity: 0,
        name: "c".to_string(),
        desc: "drop stack".to_string(),
        func: operator_c
    };

    let op_plus = Operation {
        arity: 2,
        name: "+".to_string(),
        desc: "add the top two elements, push onto stack".to_string(),
        func: operator_plus
    };

    let op_minus = Operation {
        arity: 2,
        name: "-".to_string(),
        desc: "subtract the top two elements, push onto stack".to_string(),
        func: operator_minus
    };

    let op_multiply = Operation {
        arity: 2,
        name: "*".to_string(),
        desc: "multiply the top two elements, push onto stack".to_string(),
        func: operator_multiply
    };

    let op_divide = Operation {
        arity: 2,
        name: "/".to_string(),
        desc: "divide the top two elements, push onto stack".to_string(),
        func: operator_divide
    };

    let op_power = Operation {
        arity: 2,
        name: "^".to_string(),
        desc: "raise the second element to the power of the first, push onto stack".to_string(),
        func: operator_power
    };

    let op_root = Operation {
        arity: 2,
        name: "root".to_string(),
        desc: "take the root of the second element (base of the first), push onto stack".to_string(),
        func: operator_root
    };

    let op_log = Operation {
        arity: 2,
        name: "log".to_string(),
        desc: "take the log of the second element (base of the first), push onto stack".to_string(),
        func: operator_log
    };

    let op_c_pi = Operation {
        arity: 0,
        name: "PI".to_string(),
        desc: "(constant) pi".to_string(),
        func: operator_const_pi
    };
    let op_c_tau = Operation {
        arity: 0,
        name: "TAU".to_string(),
        desc: "(constant) tau".to_string(),
        func: operator_const_tau
    };
    let op_c_e = Operation {
        arity: 0,
        name: "E".to_string(),
        desc: "(constant) E".to_string(),
        func: operator_const_e
    };

    return vec![
        op_l,
        op_f,
        op_p,
        op_d,
        op_capital_p,
        op_c,
        op_plus,
        op_minus,
        op_multiply,
        op_divide,
        op_power,
        op_root,
        op_log,
        op_c_pi,
        op_c_tau,
        op_c_e,
    ]
}
