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
            Statement::Import { module, items } => {
                // For now, just record the import (library loading would be here)
                // In a full implementation, this would load the .ln module file
                eprintln!("Note: Importing module '{}' with items: {}", module, items.join(", "));
                Ok(())
            }
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
                // Check for built-in functions
                match name.as_str() {
                    "len" | "sum" | "mean" | "max" | "min" | "shape" | "reshape" |
                    "transpose" | "flatten" | "asFloat" | "asInt" | "asString" => {
                        Ok(Value::String(name.clone())) // Return function name as placeholder
                    }
                    _ => self.variables.get(name)
                        .cloned()
                        .ok_or_else(|| Error::VariableNotFound(name.clone()))
                }
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
            Expression::Call { func, args } => {
                self.eval_function_call(func, args)
            }
            Expression::Array(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval_expression(elem)?);
                }
                Ok(Value::Array(values))
            }
            Expression::Index { expr, index } => {
                let arr_val = self.eval_expression(expr)?;
                let idx_val = self.eval_expression(index)?;
                let idx = idx_val.to_int()? as usize;
                
                match arr_val {
                    Value::Array(ref arr) => {
                        arr.get(idx)
                            .cloned()
                            .ok_or_else(|| Error::RuntimeError(format!("Index {} out of bounds", idx)))
                    }
                    Value::Matrix(ref mat) => {
                        mat.get(idx)
                            .map(|row| Value::Array(row.clone()))
                            .ok_or_else(|| Error::RuntimeError(format!("Index {} out of bounds", idx)))
                    }
                    Value::Tensor(ref tens) => {
                        tens.get(idx)
                            .map(|tensor| Value::Matrix(tensor.clone()))
                            .ok_or_else(|| Error::RuntimeError(format!("Index {} out of bounds", idx)))
                    }
                    _ => Err(Error::TypeError(format!("Cannot index {}", arr_val.type_name()))),
                }
            }
            Expression::Slice { expr, start, end, step } => {
                let arr_val = self.eval_expression(expr)?;
                let start_idx = if let Some(s) = start {
                    self.eval_expression(s)?.to_int()? as usize
                } else {
                    0
                };
                let end_idx = if let Some(e) = end {
                    self.eval_expression(e)?.to_int()? as usize
                } else {
                    match &arr_val {
                        Value::Array(a) => a.len(),
                        _ => 0,
                    }
                };
                let step_val = if let Some(s) = step {
                    self.eval_expression(s)?.to_int()? as usize
                } else {
                    1
                };
                
                match arr_val {
                    Value::Array(ref arr) => {
                        let sliced: Vec<Value> = arr.iter().enumerate()
                            .filter(|(i, _)| *i >= start_idx && *i < end_idx && (*i - start_idx) % step_val == 0)
                            .map(|(_, v)| v.clone())
                            .collect();
                        Ok(Value::Array(sliced))
                    }
                    _ => Err(Error::TypeError(format!("Cannot slice {}", arr_val.type_name()))),
                }
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

    fn eval_function_call(&mut self, func: &Expression, args: &[Expression]) -> Result<Value> {
        if let Expression::Identifier(name) = func {
            match name.as_str() {
                "len" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("len() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Array(ref a) => Ok(Value::Int(a.len() as i64)),
                        Value::Matrix(ref m) => Ok(Value::Int(m.len() as i64)),
                        Value::Tensor(ref t) => Ok(Value::Int(t.len() as i64)),
                        Value::String(ref s) => Ok(Value::Int(s.len() as i64)),
                        _ => Err(Error::TypeError("len() expects array, matrix, or string".to_string())),
                    }
                }
                "sum" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("sum() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Array(ref a) => {
                            let mut sum = 0i64;
                            for v in a {
                                sum += v.to_int()?;
                            }
                            Ok(Value::Int(sum))
                        }
                        _ => Err(Error::TypeError("sum() expects array".to_string())),
                    }
                }
                "mean" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("mean() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Array(ref a) => {
                            if a.is_empty() {
                                return Ok(Value::Float(0.0));
                            }
                            let mut sum = 0.0;
                            for v in a {
                                sum += v.to_float()?;
                            }
                            Ok(Value::Float(sum / a.len() as f64))
                        }
                        _ => Err(Error::TypeError("mean() expects array".to_string())),
                    }
                }
                "max" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("max() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Array(ref a) => {
                            if a.is_empty() {
                                return Err(Error::RuntimeError("max() of empty array".to_string()));
                            }
                            let mut max_val = a[0].to_float()?;
                            for v in a {
                                let f = v.to_float()?;
                                if f > max_val {
                                    max_val = f;
                                }
                            }
                            Ok(Value::Float(max_val))
                        }
                        _ => Err(Error::TypeError("max() expects array".to_string())),
                    }
                }
                "min" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("min() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Array(ref a) => {
                            if a.is_empty() {
                                return Err(Error::RuntimeError("min() of empty array".to_string()));
                            }
                            let mut min_val = a[0].to_float()?;
                            for v in a {
                                let f = v.to_float()?;
                                if f < min_val {
                                    min_val = f;
                                }
                            }
                            Ok(Value::Float(min_val))
                        }
                        _ => Err(Error::TypeError("min() expects array".to_string())),
                    }
                }
                "shape" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("shape() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Array(ref a) => {
                            Ok(Value::Array(vec![Value::Int(a.len() as i64)]))
                        }
                        Value::Matrix(ref m) => {
                            let rows = m.len() as i64;
                            let cols = if m.is_empty() { 0 } else { m[0].len() as i64 };
                            Ok(Value::Array(vec![Value::Int(rows), Value::Int(cols)]))
                        }
                        Value::Tensor(ref t) => {
                            let d1 = t.len() as i64;
                            let d2 = if t.is_empty() { 0 } else { t[0].len() as i64 };
                            let d3 = if t.is_empty() || t[0].is_empty() { 0 } else { t[0][0].len() as i64 };
                            Ok(Value::Array(vec![Value::Int(d1), Value::Int(d2), Value::Int(d3)]))
                        }
                        _ => Err(Error::TypeError("shape() expects array/matrix/tensor".to_string())),
                    }
                }
                "asFloat" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("asFloat() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Array(ref a) => {
                            let floats: Result<Vec<Value>> = a.iter()
                                .map(|v| Ok(Value::Float(v.to_float()?)))
                                .collect();
                            Ok(Value::Array(floats?))
                        }
                        _ => Ok(Value::Float(val.to_float()?)),
                    }
                }
                "asInt" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("asInt() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Array(ref a) => {
                            let ints: Result<Vec<Value>> = a.iter()
                                .map(|v| Ok(Value::Int(v.to_int()?)))
                                .collect();
                            Ok(Value::Array(ints?))
                        }
                        _ => Ok(Value::Int(val.to_int()?)),
                    }
                }
                "asString" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("asString() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    Ok(Value::String(val.to_string()))
                }
                _ => Err(Error::RuntimeError(format!("Unknown function: {}", name))),
            }
        } else {
            Err(Error::RuntimeError("Invalid function call".to_string()))
        }
    }

    fn eval_binary_op(&self, left: &Value, op: BinaryOp, right: &Value) -> Result<Value> {
        match op {
            BinaryOp::Add => match (left, right) {
                // Scalar addition
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::String(a), _) => Ok(Value::String(format!("{}{}", a, right.to_string()))),
                (_, Value::String(b)) => Ok(Value::String(format!("{}{}", left.to_string(), b))),
                // Array element-wise addition
                (Value::Array(a), Value::Array(b)) => {
                    if a.len() != b.len() {
                        return Err(Error::InvalidOperation("Arrays must have same length for element-wise operations".to_string()));
                    }
                    let result: Result<Vec<Value>> = a.iter().zip(b.iter())
                        .map(|(x, y)| self.eval_binary_op(x, op, y))
                        .collect();
                    Ok(Value::Array(result?))
                }
                // Scalar + Array (broadcast)
                (Value::Int(scalar), Value::Array(arr)) | (Value::Array(arr), Value::Int(scalar)) => {
                    let result: Result<Vec<Value>> = arr.iter()
                        .map(|v| match v {
                            Value::Int(n) => Ok(Value::Int(n + scalar)),
                            Value::Float(f) => Ok(Value::Float(f + *scalar as f64)),
                            _ => Err(Error::InvalidOperation("Cannot add scalar to non-numeric array".to_string())),
                        })
                        .collect();
                    Ok(Value::Array(result?))
                }
                _ => Err(Error::InvalidOperation("Cannot add these types".to_string())),
            },
            BinaryOp::Subtract => match (left, right) {
                // Scalar subtraction
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
                // Array element-wise subtraction
                (Value::Array(a), Value::Array(b)) => {
                    if a.len() != b.len() {
                        return Err(Error::InvalidOperation("Arrays must have same length".to_string()));
                    }
                    let result: Result<Vec<Value>> = a.iter().zip(b.iter())
                        .map(|(x, y)| self.eval_binary_op(x, op, y))
                        .collect();
                    Ok(Value::Array(result?))
                }
                _ => Err(Error::InvalidOperation("Cannot subtract these types".to_string())),
            },
            BinaryOp::Multiply => match (left, right) {
                // Scalar multiplication
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
                // Element-wise multiplication
                (Value::Array(a), Value::Array(b)) => {
                    if a.len() != b.len() {
                        return Err(Error::InvalidOperation("Arrays must have same length".to_string()));
                    }
                    let result: Result<Vec<Value>> = a.iter().zip(b.iter())
                        .map(|(x, y)| self.eval_binary_op(x, op, y))
                        .collect();
                    Ok(Value::Array(result?))
                }
                // Scalar * Array (broadcast)
                (Value::Int(scalar), Value::Array(arr)) | (Value::Array(arr), Value::Int(scalar)) => {
                    let result: Result<Vec<Value>> = arr.iter()
                        .map(|v| match v {
                            Value::Int(n) => Ok(Value::Int(n * scalar)),
                            Value::Float(f) => Ok(Value::Float(f * *scalar as f64)),
                            _ => Err(Error::InvalidOperation("Cannot multiply non-numeric array".to_string())),
                        })
                        .collect();
                    Ok(Value::Array(result?))
                }
                _ => Err(Error::InvalidOperation("Cannot multiply these types".to_string())),
            },
            BinaryOp::Divide => match (left, right) {
                // Scalar division
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
                // Array division
                (Value::Array(a), Value::Array(b)) => {
                    if a.len() != b.len() {
                        return Err(Error::InvalidOperation("Arrays must have same length".to_string()));
                    }
                    let result: Result<Vec<Value>> = a.iter().zip(b.iter())
                        .map(|(x, y)| self.eval_binary_op(x, op, y))
                        .collect();
                    Ok(Value::Array(result?))
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
