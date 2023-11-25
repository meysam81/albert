// in every loop
//   if it is an operand => push to stack
//   if not
//     - take the last item in the stack (an operand) and push it in a temp vector
//     - take the next items in the stack until they are operand & higher priority than the current priority
//     - push the operand, push everything remaining in the temp vector

fn is_operand(elem: &str) -> bool {
  elem.parse::<i32>().is_ok()
}

fn is_higher_prio(coming: &str, current: &str) -> bool {
    let high: Vec<&str> = vec!["*", "/"];
    let low: Vec<&str> = vec!["-", "+"];

    high.contains(&coming) && low.contains(&current)
}

fn infix_to_prefix(args: Vec<String>) -> Vec<String> {
    let mut stack: Vec<String> = vec![];
    let operand = ["+", "-", "/", "*"];
    let _ = args
        .into_iter()
        .map(|elem| {
          let mut stack: Vec<String> = vec![];
          if is_operand(&elem) {
            stack.push(elem);
          } else {
            let mut partial: Vec<String> = vec![];
            partial.push(stack.pop().unwrap());

            let _: Vec<_> = partial.into_iter().rev().map(|e| stack.push(e)).collect();
          }
        })
        .collect::<Vec<_>>();
    stack
}

fn main() {
    let cmd_line_args = std::env::args().skip(1).collect::<Vec<String>>();
    println!("{:?}", cmd_line_args);
    println!("{:?}", infix_to_prefix(cmd_line_args));
}
