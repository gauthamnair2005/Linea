use linea_core::Type;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Import {
        module: String,
        items: Vec<String>,
    },
    VarDeclaration {
        name: String,
        expr: Expression,
    },
    VarUpdate {
        name: String,
        expr: Expression,
    },
    Display(Expression),
    For {
        var: String,
        start: Expression,
        end: Expression,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    FunctionDecl {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
    Expression(Expression),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        member: String,
    },
    Index {
        expr: Box<Expression>,
        index: Box<Expression>,
    },
    Array(Vec<Expression>),
    TypeCast {
        expr: Box<Expression>,
        target_type: Type,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}
