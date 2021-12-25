
// Visitor design pattern

trait ExpressionVisitor {
    fn visit_unary(&mut self, c: &UnaryExpression);
    fn visit_binary(&mut self, r: &BinaryExpression);
}

trait Expression {
    fn accept<V: ExpressionVisitor>(&self, sv: &mut V);
}

#[derive(PartialEq, Debug, Clone)]
struct Token {
    lexeme: String,
}

#[derive(PartialEq, Debug, Clone)]
struct UnaryExpression {
    value: Token,
    operator: String,
}

#[derive(PartialEq, Debug, Clone)]
struct BinaryExpression {
    left: Token,
    operator: String,
    right: Token,
}

impl Expression for UnaryExpression {
    fn accept<V: ExpressionVisitor>(&self, sv: &mut V) {
        sv.visit_unary(self);
    }
}

impl Expression for BinaryExpression {
    fn accept<V: ExpressionVisitor>(&self, sv: &mut V) {
        sv.visit_binary(self);
    }
}

struct Interpreter {
}
 
impl ExpressionVisitor for Interpreter {
    fn visit_unary(&mut self, assign: &UnaryExpression) {
        println!("{} {}", assign.operator, assign.value.lexeme);
    }
    fn visit_binary(&mut self, binary: &BinaryExpression) {
        println!("{} {} {}", binary.left.lexeme, binary.operator, binary.right.lexeme);
    }
}

fn main() {
    let t1 = Token{lexeme: "c".into()};
    let left = Token{lexeme: "a".into()};
    let right = Token{lexeme: "b".into()};

    let assign = UnaryExpression {operator: "-".into(), value: t1};
    let binary= BinaryExpression {left: left, operator: "+".into(), right: right};

    let mut ev = Interpreter{};
    assign.accept(&mut ev);
    binary.accept(&mut ev);
}



