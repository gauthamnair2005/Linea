use std::collections::HashMap;
use linea_core::{Type, TypeContext, Value, Result, Error};
use linea_ast::{Program, Statement, Expression, BinaryOp, UnaryOp};

pub struct Executor {
    type_context: TypeContext,
    variables: HashMap<String, Value>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            type_context: TypeContext::new(),
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, program: &Program) -> Result<()> {
        for statement in &program.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::VarDeclaration { name, expr } => {
                let value = self.eval_expression(expr)?;
                let ty = value.to_type();
                self.type_context.declare(name.clone(), ty)?;
                self.variables.insert(name.clone(), value);
                Ok(())
            }
            Statement::VarUpdate { name, expr } => {
                if !self.variables.contains_key(name) {
                    return Err(Error::VariableNotFound(name.clone()));
                }
                let value = self.eval_expression(expr)?;
                let ty = value.to_type();
                
                if let Some(old_ty) = self.type_context.lookup(name) {
                    if !ty.can_coerce_to(&old_ty) && old_ty != ty {
                        return Err(Error::TypeError(format!(
                            "Cannot assign {} to variable of type {}",
                            ty.display_name(),
                            old_ty.display_name()
                        )));
                    }
                }
                
                self.variables.insert(name.clone(), value);
                Ok(())
            }
            Statement::Display(expr) => {
                let value = self.eval_expression(expr)?;
                println!("{}", value.to_string());
                Ok(())
            }
            Statement::For { var, start, end, body } => {
                let start_val = self.eval_expression(start)?.to_int()?;
                let end_val = self.eval_expression(end)?.to_int()?;
                
                for i in start_val..=end_val {
                    self.variables.insert(var.clone(), Value::Int(i));
                    for stmt in body {
                        self.execute_statement(stmt)?;
                    }
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                loop {
                    let cond_val = self.eval_expression(condition)?;
                    if !cond_val.to_bool() {
                        break;
                    }
                    for stmt in body {
                        self.execute_statement(stmt)?;
                    }
                }
                Ok(())
            }
            Statement::If { condition, then_body, else_body } => {
                let cond_val = self.eval_expression(condition)?;
                if cond_val.to_bool() {
                    for stmt in then_body {
                        self.execute_statement(stmt)?;
                    }
                } else if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        self.execute_statement(stmt)?;
                    }
                }
                Ok(())
            }
            Statement::Expression(expr) => {
                self.eval_expression(expr)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn eval_expression(&mut self, expression: &Expression) -> Result<Value> {
        match expression {
            Expression::Integer(n) => Ok(Value::Int(*n)),
            Expression::Float(f) => Ok(Value::Float(*f)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Bool(b) => Ok(Value::Bool(*b)),
            Expression::Identifier(name) => {
                self.variables.get(name)
                    .cloned()
                    .ok_or_else(|| Error::VariableNotFound(name.clone()))
            }
            Expression::Binary { left, op, right } => {
                let left_val = self.eval_expression(left)?;
                let right_val = self.eval_expression(right)?;
                self.eval_binary_op(&left_val, *op, &right_val)
            }
            Expression::Unary { op, expr } => {
                let val = self.eval_expression(expr)?;
                self.eval_unary_op(*op, &val)
            }
            Expression::Array(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval_expression(elem)?);
                }
                Ok(Value::Array(values))
            }
            Expression::TypeCast { expr, target_type } => {
                let val = self.eval_expression(expr)?;
                match target_type {
                    Type::Int => Ok(Value::Int(val.to_int()?)),
                    Type::Float => Ok(Value::Float(val.to_float()?)),
                    Type::String => Ok(Value::String(val.to_string())),
                    Type::Bool => Ok(Value::Bool(val.to_bool())),
                    _ => Err(Error::TypeError("Cannot cast to this type".to_string())),
                }
            }
            _ => Err(Error::RuntimeError("Unsupported expression".to_string())),
        }
    }

    fn eval_binary_op(&self, left: &Value, op: BinaryOp, right: &Value) -> Result<Value> {
        match op {
            BinaryOp::Add => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::String(a), _) => Ok(Value::String(format!("{}{}", a, right.to_string()))),
                (_, Value::String(b)) => Ok(Value::String(format!("{}{}", left.to_string(), b))),
                _ => Err(Error::InvalidOperation("Cannot add these types".to_string())),
            },
            BinaryOp::Subtract => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
                _ => Err(Error::InvalidOperation("Cannot subtract these types".to_string())),
            },
            BinaryOp::Multiply => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
                _ => Err(Error::InvalidOperation("Cannot multiply these types".to_string())),
            },
            BinaryOp::Divide => match (left, right) {
                (Value::Int(a), Value::Int(b)) => {
                    if *b == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    Ok(Value::Int(a / b))
                }
                (Value::Float(a), Value::Float(b)) => {
                    if *b == 0.0 {
                        return Err(Error::DivisionByZero);
                    }
                    Ok(Value::Float(a / b))
                }
                (Value::Int(a), Value::Float(b)) => {
                    if *b == 0.0 {
                        return Err(Error::DivisionByZero);
                    }
                    Ok(Value::Float(*a as f64 / b))
                }
                (Value::Float(a), Value::Int(b)) => {
                    if *b == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    Ok(Value::Float(a / *b as f64))
                }
                _ => Err(Error::InvalidOperation("Cannot divide these types".to_string())),
            },
            BinaryOp::Modulo => match (left, right) {
                (Value::Int(a), Value::Int(b)) => {
                    if *b == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    Ok(Value::Int(a % b))
                }
                _ => Err(Error::InvalidOperation("Modulo requires integers".to_string())),
            },
            BinaryOp::Power => match (left, right) {
                (Value::Int(a), Value::Int(b)) => {
                    if *b < 0 {
                        Ok(Value::Float((*a as f64).powf(*b as f64)))
                    } else {
                        Ok(Value::Int(a.pow(*b as u32)))
                    }
                }
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powf(*b as f64))),
                _ => Err(Error::InvalidOperation("Cannot power these types".to_string())),
            },
            BinaryOp::Equal => Ok(Value::Bool(left == right)),
            BinaryOp::NotEqual => Ok(Value::Bool(left != right)),
            BinaryOp::Less => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) < *b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a < (*b as f64))),
                _ => Err(Error::InvalidOperation("Cannot compare these types".to_string())),
            },
            BinaryOp::Greater => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) > *b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a > (*b as f64))),
                _ => Err(Error::InvalidOperation("Cannot compare these types".to_string())),
            },
            BinaryOp::LessEqual => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) <= *b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a <= (*b as f64))),
                _ => Err(Error::InvalidOperation("Cannot compare these types".to_string())),
            },
            BinaryOp::GreaterEqual => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) >= *b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a >= (*b as f64))),
                _ => Err(Error::InvalidOperation("Cannot compare these types".to_string())),
            },
            BinaryOp::And => Ok(Value::Bool(left.to_bool() && right.to_bool())),
            BinaryOp::Or => Ok(Value::Bool(left.to_bool() || right.to_bool())),
        }
    }

    fn eval_unary_op(&self, op: UnaryOp, val: &Value) -> Result<Value> {
        match op {
            UnaryOp::Negate => match val {
                Value::Int(n) => Ok(Value::Int(-n)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err(Error::InvalidOperation("Cannot negate this type".to_string())),
            },
            UnaryOp::Not => Ok(Value::Bool(!val.to_bool())),
        }
    }
}
