pub(crate) struct Calculator {
    args: Vec<String>,
}

impl Calculator {
    const PLUS: &'static str = "+";

    const MINUS: &'static str = "-";

    const MULTIPLY: &'static str = "*";

    const DIVIDE: &'static str = "/";

    const EPSILON: f64 = 0.0000001;

    pub fn new(args: Vec<String>) -> Calculator {
        Calculator { args }
    }

    pub fn calculate(&self) -> f64 {
        let mut stack: Vec<f64> = Vec::new();

        self.args.iter().for_each(|token| {
            let token = token.as_str();

            match token {
                Calculator::PLUS => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap_or(0.0);

                    stack.push(a + b);
                }
                Calculator::MINUS => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap_or(2.0 * a);
                    stack.push(b - a);
                }
                Calculator::MULTIPLY => {
                    let a = stack.pop().unwrap();
                    println!("a: {}", a);
                    let b = stack.pop().unwrap_or(1.0);
                    stack.push(a * b);
                }
                Calculator::DIVIDE => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap_or(a * a);
                    stack.push(b / a);
                }
                _ => {
                    let num = token.parse::<f64>().unwrap_or_else(
                        |error| panic!("Invalid token: {}, {}", token, error)
                    );
                    stack.push(num);
                }
            }
        });

        self.round_result(stack.pop().unwrap())
    }

    fn round_result(&self, result: f64) -> f64 {
        let rounded = (result / Calculator::EPSILON).round() * Calculator::EPSILON;
        rounded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpn_calculator_is_able_to_divide() {
        let input = vec!["19".to_string(), "2".to_string(), "/".to_string()];
        assert_eq!(Calculator::new(input).calculate(), 9.5);
    }
}
