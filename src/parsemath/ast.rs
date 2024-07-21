use super::parser::Node;

pub fn eval(expr: Node) -> Result<f64, Box<dyn std::error::Error>> {

    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1, expr2) => Ok(eval(*expr1)? + 
            eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - 
            eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? *
            eval(*expr2)?),
        Divide(expr1, expr2) => Ok(eval(*expr1)? /
            eval(*expr2)?),
        Negative(expr1) => Ok(-(eval(*expr1)?)),
        Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)), 
    }
}