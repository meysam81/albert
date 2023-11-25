// in every loop
//   if it is an operand => push to stack
//   if not
//     - take the last item in the stack (an operand) and push it in a temp vector
//     - take the next items in the stack until they are operand & higher priority than the current priority
//     - push the operand, push everything remaining in the temp vector

fn is_operand(elem: &str) -> bool {
    elem.parse::<i32>().is_ok()
}

fn is_current_higher_prio(current: &str, coming: &str) -> bool {
    let high: Vec<&str> = vec!["*", "/"];
    let low: Vec<&str> = vec!["-", "+"];

    high.contains(&current) || (low.contains(&current) && low.contains(&coming))
}

fn do_calculate(operand1: &i32, operator: &str, operand2: &i32) -> i32 {
    match operator {
        "*" => operand1 * operand2,
        "/" => operand1 / operand2,
        "+" => operand1 + operand2,
        "-" => operand1 - operand2,
        &_ => unimplemented!(),
    }
}

fn infix_to_prefix(args: Vec<String>) -> Vec<String> {
    let mut stack: Vec<String> = vec![];
    let mut output: Vec<String> = vec![];

    args.into_iter().rev().for_each(|op| {
        // println!("{:?} {:?} {:?}", &stack, &output, &op);
        if is_operand(&op) {
            output.push(op);
        } else {
            while stack.len() != 0 && !is_current_higher_prio(&op, stack.last().unwrap()) {
                // println!("{:?} {:?}", &stack, &output);
                output.push(stack.pop().unwrap());
            }
            stack.push(op);
        }
    });

    stack.into_iter().for_each(|operand| output.push(operand));

    output.into_iter().rev().collect()
}

fn calculate_result(prefix_notation: Vec<String>) -> i32 {
    let mut stack: Vec<_> = vec![];
    prefix_notation.into_iter().rev().for_each(|op| {
        if is_operand(&op) {
            stack.push(op);
        } else {
            let operand1 = stack.pop().unwrap().parse::<i32>().unwrap();
            let operand2 = stack.pop().unwrap().parse::<i32>().unwrap();
            stack.push(do_calculate(&operand1, &op, &operand2).to_string())
        }
    });

    stack[0].parse::<i32>().unwrap()
}

fn main() {
    let cmd_line_args = std::env::args().skip(1).collect::<Vec<String>>();
    println!("{:?}", cmd_line_args);
    println!("{:?}", calculate_result(infix_to_prefix(cmd_line_args)));
}

#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn basic_arithmetic() {
        // 3 + 4 * 5 / 6
        // + 3 / * 4 5 6
        let inp: Vec<String> = "3 + 4 * 5 / 6".split(" ").collect();
        let rv = calculate_result(inp);
        assert_eq!(rv, 6);
    }
}
