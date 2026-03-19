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
        type_annotation: Option<String>,
        expr: Expression,
    },
    ObjDeclaration {
        name: String,
        class_name: String,
        constructor: Expression,
    },
    VarUpdate {
        name: String,
        expr: Expression,
    },
    Assignment {
        target: Expression,
        expr: Expression,
    },
    Display(Expression),
    For {
        var: String,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
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
    Switch {
        expr: Expression,
        cases: Vec<(Expression, Vec<Statement>)>,
        default: Option<Vec<Statement>>,
    },
    FunctionDecl {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    },
    ClassDecl {
        name: String,
        super_class: Option<String>,
        body: Vec<Statement>,
    },
    MacroDecl {
        name: String,
        params: Vec<String>,
        body: Expression,
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
    Lambda {
        params: Vec<String>,
        body: Box<Expression>,
    },
    MacroCall {
        name: String,
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
    Slice {
        expr: Box<Expression>,
        start: Option<Box<Expression>>,
        end: Option<Box<Expression>>,
        step: Option<Box<Expression>>,
    },
    Array(Vec<Expression>),
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        step: Option<Box<Expression>>,
    },
    TypeCast {
        expr: Box<Expression>,
        target_type: Type,
    },
    Ternary {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    IfExpression {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
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
    AddressOf,  // & operator for pointers
    Dereference, // * operator for pointers
}
