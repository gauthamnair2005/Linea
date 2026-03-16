use std::collections::HashMap;
use linea_core::{Type, TypeContext, Value, Result, Error};
use linea_ast::{Program, Statement, Expression, BinaryOp, UnaryOp};
use linea_ast::lexer::Lexer;
use linea_ast::parser::Parser;

mod compute;

// Graphics State Structures
#[derive(Clone, Debug)]
pub struct ChartConfig {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub series: Vec<Series>,
}

#[derive(Clone, Debug)]
pub enum Series {
    Line { x: Vec<f64>, y: Vec<f64>, label: String, color: String },
    Scatter { x: Vec<f64>, y: Vec<f64>, label: String, color: String },
    Bar { labels: Vec<String>, values: Vec<f64>, label: String, color: String },
}

impl ChartConfig {
    pub fn new() -> Self {
        ChartConfig {
            title: "Chart".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            series: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDef {
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

pub struct Executor {
    type_context: TypeContext,
    scopes: Vec<HashMap<String, Value>>,
    functions: HashMap<String, FunctionDef>,
    chart_config: ChartConfig,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            type_context: TypeContext::new(),
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            chart_config: ChartConfig::new(),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.type_context.push_scope();
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
        self.type_context.pop_scope();
    }

    fn declare_variable(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    fn update_variable(&mut self, name: String, value: Value) -> Result<()> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return Ok(());
            }
        }
        Err(Error::VariableNotFound(name))
    }

    fn get_variable(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
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
                let paths = vec![
                    format!("{}.ln", module),
                    format!("libs/{}.ln", module),
                    format!("../libs/{}.ln", module),
                ];
                
                let mut source = None;
                for path in &paths {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        source = Some(content);
                        break;
                    }
                }
                
                if let Some(content) = source {
                    let lexer = Lexer::new(&content);
                    let tokens = lexer.tokenize()?;
                    let mut parser = Parser::new(tokens);
                    let program = parser.parse()?;
                    
                    for stmt in program.statements {
                        self.execute_statement(&stmt)?;
                    }
                } else {
                    // Ignore missing built-in modules for now as they are simulated
                    match module.as_str() {
                         "math" | "strings" | "csv" | "excel" | "graphics" => {},
                         _ => return Err(Error::RuntimeError(format!("Module '{}' not found in paths: {:?}", module, paths))),
                    }
                }
                Ok(())
            }
            Statement::VarDeclaration { name, expr } => {
                let value = self.eval_expression(expr)?;
                let ty = value.to_type();
                self.type_context.declare(name.clone(), ty)?;
                self.declare_variable(name.clone(), value);
                Ok(())
            }
            Statement::VarUpdate { name, expr } => {
                if self.get_variable(name).is_none() {
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
                
                self.update_variable(name.clone(), value)?;
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
                    self.declare_variable(var.clone(), Value::Int(i));
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
            Statement::FunctionDecl { name, params, return_type, body } => {
                let func_def = FunctionDef {
                    params: params.clone(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                };
                self.functions.insert(name.clone(), func_def);
                Ok(())
            }
            Statement::Return(expr_opt) => {
                let val = if let Some(expr) = expr_opt {
                    self.eval_expression(expr)?
                } else {
                    Value::Bool(false)
                };
                Err(Error::Return(val))
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
                    _ => self.get_variable(name)
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
                
                // Auto-detect Matrix (2D) and Tensor (3D) structure
                if !values.is_empty() {
                    // Check if elements are arrays (potential Matrix)
                    if let Value::Array(_) = values[0] {
                        let is_matrix_structure = values.iter().all(|v| matches!(v, Value::Array(_)));
                        
                        if is_matrix_structure {
                            let rows: Vec<Vec<Value>> = values.into_iter().map(|v| {
                                if let Value::Array(a) = v { a } else { unreachable!() }
                            }).collect();
                            
                            // Check if elements are matrices (potential Tensor)
                            // Actually, since we process recursively, inner arrays would have already been converted to Matrix/Tensor?
                            // No, because strict evaluation happens bottom-up.
                            // But wait, if inner elements were [1, 2], they became Value::Array.
                            // So [[1, 2]] becomes Array(Array). We convert to Matrix.
                            // If we have [[[1]], [[2]]]:
                            // Inner [1] -> Array.
                            // Middle [[1]] -> Matrix? No, because inner is Array.
                            // So [[1]] becomes Matrix containing arrays? No, Matrix contains Vec<Value>.
                            // Value::Matrix(Vec<Vec<Value>>).
                            // So [[1]] -> Matrix([[1]]).
                            
                            // What about 3D? [[[1]]]
                            // Inner [1] -> Array.
                            // Middle [[1]] -> Matrix([[1]])?
                            // Outer [[[1]]] -> Array(Matrix).
                            // We should convert Array(Matrix) to Tensor.
                            
                            // Let's check for Tensor structure first (Array of Matrix)
                            /*
                            if let Value::Matrix(_) = values[0] { // This won't happen because values[0] is from recursive eval which returns Array/Matrix/Tensor
                                // If recursive eval returns Matrix, then we have Array of Matrix -> Tensor
                            }
                            */
                            
                            // Let's rely on what we have.
                            // If we just converted `values` (which are Arrays) to `rows` (Vec<Vec<Value>>),
                            // we have a Matrix structure.
                            
                            // Now check if it's actually 3D (Tensor)
                            if !rows.is_empty() && !rows[0].is_empty() {
                                if let Value::Array(_) = rows[0][0] {
                                    // It looks like 3D: Vec<Vec<Array>>
                                    // We need to flatten it to Vec<Vec<Vec<Value>>>
                                    let is_tensor = rows.iter().all(|row| row.iter().all(|cell| matches!(cell, Value::Array(_))));
                                    
                                    if is_tensor {
                                         let cubes: Vec<Vec<Vec<Value>>> = rows.into_iter().map(|row| {
                                             row.into_iter().map(|cell| {
                                                 if let Value::Array(a) = cell { a } else { unreachable!() }
                                             }).collect()
                                         }).collect();
                                         return Ok(Value::Tensor(cubes));
                                    }
                                }
                            }
                            
                            return Ok(Value::Matrix(rows));
                        }
                        
                        // Check if elements are Matrices (explicitly constructed or returned from functions)
                        if let Value::Matrix(_) = values[0] {
                             let is_tensor_from_matrices = values.iter().all(|v| matches!(v, Value::Matrix(_)));
                             if is_tensor_from_matrices {
                                 let cubes: Vec<Vec<Vec<Value>>> = values.into_iter().map(|v| {
                                     if let Value::Matrix(m) = v { m } else { unreachable!() }
                                 }).collect();
                                 return Ok(Value::Tensor(cubes));
                             }
                        }
                    }
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
                        Value::Matrix(m) => m.len(),
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
                    Value::Matrix(ref rows) => {
                        let sliced: Vec<Vec<Value>> = rows.iter().enumerate()
                            .filter(|(i, _)| *i >= start_idx && *i < end_idx && (*i - start_idx) % step_val == 0)
                            .map(|(_, v)| v.clone())
                            .collect();
                        Ok(Value::Matrix(sliced))
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
                "compute::device" => {
                    Ok(Value::String(compute::device()))
                }
                "compute::type" => {
                    Ok(Value::String(compute::device_type()))
                }
                "compute::matmul" => {
                    if args.len() != 2 { return Err(Error::InvalidOperation("matmul requires 2 arguments".to_string())); }
                    let a = self.eval_expression(&args[0])?;
                    let b = self.eval_expression(&args[1])?;

                    let a_mat: Vec<Vec<f64>> = if let Value::Matrix(m) = &a {
                         m.iter().map(|row| row.iter().map(|v| match v {
                            Value::Int(i) => *i as f64,
                            Value::Float(f) => *f,
                            _ => 0.0
                        }).collect()).collect()
                    } else {
                        return Err(Error::TypeError(format!("Expected Matrix for argument 1, got {:?}", a.type_name())));
                    };

                    let b_mat: Vec<Vec<f64>> = if let Value::Matrix(m) = &b {
                        m.iter().map(|row| row.iter().map(|v| match v {
                            Value::Int(i) => *i as f64,
                            Value::Float(f) => *f,
                            _ => 0.0
                        }).collect()).collect()
                    } else {
                        return Err(Error::TypeError(format!("Expected Matrix for argument 2, got {:?}", b.type_name())));
                    };

                    let result = compute::matmul(&a_mat, &b_mat);
                    
                    let result_val = result.into_iter().map(|row| {
                        row.into_iter().map(|v| Value::Float(v)).collect()
                    }).collect();

                    Ok(Value::Matrix(result_val))
                }
                "compute::add" | "compute::sub" | "compute::mul" | "compute::div" => {
                    let op = if name.contains("add") { "add" }
                             else if name.contains("sub") { "sub" }
                             else if name.contains("mul") { "mul" }
                             else { "div" };

                    if args.len() != 2 { return Err(Error::InvalidOperation("element-wise op requires 2 arguments".to_string())); }
                    let a_val = self.eval_expression(&args[0])?;
                    let b_val = self.eval_expression(&args[1])?;

                    match (a_val, b_val) {
                        (Value::Array(a), Value::Array(b)) => {
                            let a_f64: Vec<f64> = a.iter().map(|v| match v {
                                Value::Int(i) => *i as f64,
                                Value::Float(f) => *f,
                                _ => 0.0,
                            }).collect();
                            let b_f64: Vec<f64> = b.iter().map(|v| match v {
                                Value::Int(i) => *i as f64,
                                Value::Float(f) => *f,
                                _ => 0.0,
                            }).collect();
                            
                            let res = compute::element_wise(&a_f64, &b_f64, op);
                            let res_val = res.into_iter().map(Value::Float).collect();
                            Ok(Value::Array(res_val))
                        }
                        (Value::Matrix(a), Value::Matrix(b)) => {
                            let rows_a = a.len();
                            let cols_a = if rows_a > 0 { a[0].len() } else { 0 };
                            let rows_b = b.len();
                            let cols_b = if rows_b > 0 { b[0].len() } else { 0 };
                            
                            // Check for broadcasting opportunity (B is 1xN, A is MxN)
                            if rows_b == 1 && rows_a > 1 && cols_a == cols_b {
                                let a_flat: Vec<f64> = a.iter().flat_map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0))).collect();
                                let b_flat: Vec<f64> = b[0].iter().map(|v| v.to_float().unwrap_or(0.0)).collect();
                                
                                let res = compute::broadcast_op_flat(&a_flat, &b_flat, rows_a, cols_a, op);
                                
                                let mut res_matrix = Vec::new();
                                for i in 0..rows_a {
                                    let start = i * cols_a;
                                    let end = start + cols_a;
                                    if end <= res.len() {
                                        let row_slice = &res[start..end];
                                        res_matrix.push(row_slice.iter().map(|&v| Value::Float(v)).collect());
                                    }
                                }
                                return Ok(Value::Matrix(res_matrix));
                            }
                            
                            // Simple broadcasting: if b has 1 row, repeat it rows_a times
                            let b_expanded = if rows_b == 1 && rows_a > 1 && cols_a == cols_b {
                                (0..rows_a).map(|_| b[0].clone()).collect()
                            } else {
                                b.clone()
                            };
                            
                            // Check dimensions
                            if rows_a != b_expanded.len() || (rows_a > 0 && cols_a != b_expanded[0].len()) {
                                 return Err(Error::InvalidOperation(format!("Matrix shape mismatch: {:?} vs {:?}", (rows_a, cols_a), (b_expanded.len(), if b_expanded.is_empty() {0} else {b_expanded[0].len()}))));
                            }
                            
                            let a_flat: Vec<f64> = a.iter().flat_map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0))).collect();
                            let b_flat: Vec<f64> = b_expanded.iter().flat_map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0))).collect();
                            
                            let res = compute::element_wise(&a_flat, &b_flat, op);
                            
                            let mut res_matrix = Vec::new();
                            for i in 0..rows_a {
                                let start = i * cols_a;
                                let end = start + cols_a;
                                if end <= res.len() {
                                    let row_slice = &res[start..end];
                                    res_matrix.push(row_slice.iter().map(|&v| Value::Float(v)).collect());
                                }
                            }
                            Ok(Value::Matrix(res_matrix))
                        }
                        (Value::Matrix(a), Value::Int(i)) => {
                            let scalar = i as f64;
                            let rows_a = a.len();
                            let cols_a = if rows_a > 0 { a[0].len() } else { 0 };
                            
                            let a_flat: Vec<f64> = a.iter().flat_map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0))).collect();
                            let b_flat = vec![scalar; a_flat.len()];
                            
                            let res = compute::element_wise(&a_flat, &b_flat, op);
                            
                            let mut res_matrix = Vec::new();
                            for idx in 0..rows_a {
                                let start = idx * cols_a;
                                let end = start + cols_a;
                                if end <= res.len() {
                                    let row_slice = &res[start..end];
                                    res_matrix.push(row_slice.iter().map(|&v| Value::Float(v)).collect());
                                }
                            }
                            Ok(Value::Matrix(res_matrix))
                        }
                        (Value::Matrix(a), Value::Float(f)) => {
                            let scalar = f;
                            let rows_a = a.len();
                            let cols_a = if rows_a > 0 { a[0].len() } else { 0 };
                            
                            let a_flat: Vec<f64> = a.iter().flat_map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0))).collect();
                            let b_flat = vec![scalar; a_flat.len()];
                            
                            let res = compute::element_wise(&a_flat, &b_flat, op);
                            
                            let mut res_matrix = Vec::new();
                            for idx in 0..rows_a {
                                let start = idx * cols_a;
                                let end = start + cols_a;
                                if end <= res.len() {
                                    let row_slice = &res[start..end];
                                    res_matrix.push(row_slice.iter().map(|&v| Value::Float(v)).collect());
                                }
                            }
                            Ok(Value::Matrix(res_matrix))
                        }
                        (Value::Int(i), Value::Matrix(b)) => {
                            let scalar = i as f64;
                            let rows_b = b.len();
                            let cols_b = if rows_b > 0 { b[0].len() } else { 0 };
                            
                            let b_flat: Vec<f64> = b.iter().flat_map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0))).collect();
                            let a_flat = vec![scalar; b_flat.len()];
                            
                            let res = compute::element_wise(&a_flat, &b_flat, op);
                            
                            let mut res_matrix = Vec::new();
                            for idx in 0..rows_b {
                                let start = idx * cols_b;
                                let end = start + cols_b;
                                if end <= res.len() {
                                    let row_slice = &res[start..end];
                                    res_matrix.push(row_slice.iter().map(|&v| Value::Float(v)).collect());
                                }
                            }
                            Ok(Value::Matrix(res_matrix))
                        }
                        (Value::Float(f), Value::Matrix(b)) => {
                            let scalar = f;
                            let rows_b = b.len();
                            let cols_b = if rows_b > 0 { b[0].len() } else { 0 };
                            
                            let b_flat: Vec<f64> = b.iter().flat_map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0))).collect();
                            let a_flat = vec![scalar; b_flat.len()];
                            
                            let res = compute::element_wise(&a_flat, &b_flat, op);
                            
                            let mut res_matrix = Vec::new();
                            for idx in 0..rows_b {
                                let start = idx * cols_b;
                                let end = start + cols_b;
                                if end <= res.len() {
                                    let row_slice = &res[start..end];
                                    res_matrix.push(row_slice.iter().map(|&v| Value::Float(v)).collect());
                                }
                            }
                            Ok(Value::Matrix(res_matrix))
                        }
                        _ => Err(Error::TypeError("Element-wise operations require arrays or matrices (or scalar broadcast)".to_string())),
                    }
                }
                "compute::random" => {
                    if args.len() != 2 { return Err(Error::InvalidOperation("compute::random(rows, cols) requires 2 arguments".to_string())); }
                    let rows = self.eval_expression(&args[0])?.to_int()? as usize;
                    let cols = self.eval_expression(&args[1])?.to_int()? as usize;
                    let mat = compute::random(rows, cols);
                    let result_val = mat.into_iter().map(|row| row.into_iter().map(Value::Float).collect()).collect();
                    Ok(Value::Matrix(result_val))
                }
                "compute::one_hot" => {
                    if args.len() != 2 { return Err(Error::InvalidOperation("compute::one_hot(labels, classes) requires 2 arguments".to_string())); }
                    let labels_val = self.eval_expression(&args[0])?;
                    let classes = self.eval_expression(&args[1])?.to_int()? as usize;
                    
                    let labels: Vec<f64> = match labels_val {
                        Value::Array(arr) => {
                            arr.iter().map(|v| v.to_float().unwrap_or(0.0)).collect()
                        }
                        _ => return Err(Error::TypeError("Labels must be an array".to_string())),
                    };
                    
                    let mat = compute::one_hot(&labels, classes);
                    let result_val = mat.into_iter().map(|row| row.into_iter().map(Value::Float).collect()).collect();
                    Ok(Value::Matrix(result_val))
                }
                "compute::transpose" | "compute::exp" | "compute::log" | "compute::relu" | "compute::sigmoid" | "compute::tanh" | "compute::sum_columns" | "compute::softmax" => {
                     if args.len() != 1 { return Err(Error::InvalidOperation("Unary compute op requires 1 argument".to_string())); }
                     let arg = self.eval_expression(&args[0])?;
                     
                     let mat: Vec<Vec<f64>> = if let Value::Matrix(m) = &arg {
                         m.iter().map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0)).collect()).collect()
                     } else {
                         return Err(Error::TypeError("Argument must be a matrix".to_string()));
                     };

                     let res = match name.as_str() {
                         "compute::transpose" => compute::transpose(&mat),
                         "compute::sum_columns" => compute::sum_columns(&mat),
                         "compute::softmax" => compute::softmax(&mat),
                         "compute::exp" => compute::exp(&mat),
                         "compute::log" => compute::log(&mat),
                         "compute::relu" => compute::relu(&mat),
                         "compute::sigmoid" => compute::sigmoid(&mat),
                         "compute::tanh" => compute::tanh(&mat),
                         _ => unreachable!(),
                     };
                     
                     let result_val = res.into_iter().map(|row| row.into_iter().map(Value::Float).collect()).collect();
                     Ok(Value::Matrix(result_val))
                }
                "compute::cross_entropy" => {
                    if args.len() != 2 { return Err(Error::InvalidOperation("cross_entropy requires 2 arguments".to_string())); }
                    let a = self.eval_expression(&args[0])?;
                    let b = self.eval_expression(&args[1])?;

                    let a_mat: Vec<Vec<f64>> = if let Value::Matrix(m) = &a {
                        m.iter().map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0)).collect()).collect()
                    } else {
                         return Err(Error::TypeError("Argument 1 must be a matrix".to_string()));
                    };
                    let b_mat: Vec<Vec<f64>> = if let Value::Matrix(m) = &b {
                        m.iter().map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0)).collect()).collect()
                    } else {
                         return Err(Error::TypeError("Argument 2 must be a matrix".to_string()));
                    };
                    
                    let loss = compute::cross_entropy(&a_mat, &b_mat);
                    Ok(Value::Float(loss))
                }
                "compute::sum" | "compute::max" | "compute::argmax" => {
                     if args.len() != 1 { return Err(Error::InvalidOperation("Reduction op requires 1 argument".to_string())); }
                     let arg = self.eval_expression(&args[0])?;
                     
                     let mat: Vec<Vec<f64>> = if let Value::Matrix(m) = &arg {
                         m.iter().map(|row| row.iter().map(|v| v.to_float().unwrap_or(0.0)).collect()).collect()
                     } else {
                         return Err(Error::TypeError("Argument must be a matrix".to_string()));
                     };

                     let val = match name.as_str() {
                         "compute::sum" => compute::sum(&mat),
                         "compute::max" => compute::max(&mat),
                         "compute::argmax" => compute::argmax(&mat),
                         _ => unreachable!(),
                     };
                     
                     Ok(Value::Float(val))
                }
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
                "sin" => {
                    if args.len() != 1 { return Err(Error::RuntimeError("sin() expects 1 argument".to_string())); }
                    let val = self.eval_expression(&args[0])?.to_float()?;
                    Ok(Value::Float(val.sin()))
                }
                "cos" => {
                    if args.len() != 1 { return Err(Error::RuntimeError("cos() expects 1 argument".to_string())); }
                    let val = self.eval_expression(&args[0])?.to_float()?;
                    Ok(Value::Float(val.cos()))
                }
                "append" => {
                    if args.len() != 2 { return Err(Error::RuntimeError("append() expects 2 arguments".to_string())); }
                    let arr_val = self.eval_expression(&args[0])?;
                    let val = self.eval_expression(&args[1])?;
                    
                    match arr_val {
                        Value::Array(mut a) => {
                             a.push(val);
                             Ok(Value::Array(a))
                        }
                        Value::Matrix(mut m) => {
                             if let Value::Array(row) = val {
                                 m.push(row);
                                 Ok(Value::Matrix(m))
                             } else {
                                 Err(Error::TypeError("append() to Matrix expects array row".to_string()))
                             }
                        }
                        _ => Err(Error::TypeError(format!("append() expects array or matrix, got {}", arr_val.type_name()))),
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
                // CSV LIBRARY FUNCTIONS
                // HTTP Functions
                "http::get" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("http::get() expects 1 argument".to_string()));
                    }
                    let url = match self.eval_expression(&args[0])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("http::get() expects string URL".to_string())),
                    };
                    
                    match reqwest::blocking::get(&url) {
                        Ok(resp) => {
                            let status = resp.status().as_u16() as i64;
                            let ok = resp.status().is_success();
                            let body = resp.text().unwrap_or_default();
                            Ok(Value::Array(vec![
                                Value::String(status.to_string()),
                                Value::String(ok.to_string()),
                                Value::String(body)
                            ]))
                        }
                        Err(e) => Err(Error::RuntimeError(format!("HTTP Request failed: {}", e))),
                    }
                }
                "http::post" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("http::post() expects 2 arguments".to_string()));
                    }
                    let url = match self.eval_expression(&args[0])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("http::post() expects string URL".to_string())),
                    };
                    let body = match self.eval_expression(&args[1])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("http::post() expects string body".to_string())),
                    };
                    
                    let client = reqwest::blocking::Client::new();
                    match client.post(&url).body(body).send() {
                         Ok(resp) => {
                            let status = resp.status().as_u16() as i64;
                            let ok = resp.status().is_success();
                            let body_text = resp.text().unwrap_or_default();
                            Ok(Value::Array(vec![
                                Value::String(status.to_string()),
                                Value::String(ok.to_string()),
                                Value::String(body_text)
                            ]))
                        }
                        Err(e) => Err(Error::RuntimeError(format!("HTTP Request failed: {}", e))),
                    }
                }
                "http::request" => {
                    if args.len() != 4 {
                        return Err(Error::RuntimeError("http::request() expects 4 arguments".to_string()));
                    }
                    let method_str = match self.eval_expression(&args[0])? {
                        Value::String(s) => s.to_uppercase(),
                        _ => return Err(Error::TypeError("Method must be string".to_string())),
                    };
                    let url = match self.eval_expression(&args[1])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("URL must be string".to_string())),
                    };
                    let headers_str = match self.eval_expression(&args[2])? {
                        Value::String(s) => s,
                        _ => "{}".to_string(),
                    };
                    let body = match self.eval_expression(&args[3])? {
                        Value::String(s) => s,
                        _ => "".to_string(),
                    };
                    
                    let client = reqwest::blocking::Client::new();
                    let method = match method_str.as_str() {
                        "GET" => reqwest::Method::GET,
                        "POST" => reqwest::Method::POST,
                        "PUT" => reqwest::Method::PUT,
                        "DELETE" => reqwest::Method::DELETE,
                        _ => return Err(Error::RuntimeError(format!("Unsupported method: {}", method_str))),
                    };
                    
                    let mut req = client.request(method, &url);
                    
                    if !headers_str.is_empty() {
                         if let Ok(json) = serde_json::from_str::<std::collections::HashMap<String, String>>(&headers_str) {
                             for (k, v) in json {
                                 req = req.header(k, v);
                             }
                         }
                    }
                    
                    req = req.body(body);
                    
                    match req.send() {
                        Ok(resp) => {
                            let status = resp.status().as_u16() as i64;
                            let ok = resp.status().is_success();
                            let body_text = resp.text().unwrap_or_default();
                            Ok(Value::Array(vec![
                                Value::String(status.to_string()),
                                Value::String(ok.to_string()),
                                Value::String(body_text)
                            ]))
                        }
                        Err(e) => Err(Error::RuntimeError(format!("HTTP Request failed: {}", e))),
                    }
                }
                "http::download" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("http::download() expects 2 arguments".to_string()));
                    }
                    let url = match self.eval_expression(&args[0])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("URL must be string".to_string())),
                    };
                    let path = match self.eval_expression(&args[1])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("Path must be string".to_string())),
                    };
                    
                    match reqwest::blocking::get(&url) {
                         Ok(mut resp) => {
                             let mut file = std::fs::File::create(&path)
                                 .map_err(|e| Error::RuntimeError(format!("Failed to create file: {}", e)))?;
                             resp.copy_to(&mut file)
                                 .map_err(|e| Error::RuntimeError(format!("Failed to write content: {}", e)))?;
                             Ok(Value::Bool(true))
                         }
                         Err(_) => Ok(Value::Bool(false)),
                    }
                }
                "csv::read" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("csv::read() expects 1 argument".to_string()));
                    }
                    let path = match self.eval_expression(&args[0])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("Path must be string".to_string())),
                    };
                    
                    let content = std::fs::read_to_string(&path)
                        .map_err(|e| Error::RuntimeError(format!("Failed to read file: {}", e)))?;
                    
                    let rows: Vec<Vec<Value>> = content.lines()
                        .map(|line| {
                            line.split(',')
                                .map(|cell| Value::String(cell.trim().to_string()))
                                .collect()
                        })
                        .collect();
                    Ok(Value::Matrix(rows))
                }
                "csv::parse" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("csv::parse() expects 1 argument".to_string()));
                    }
                    let csv_text = self.eval_expression(&args[0])?.to_string();
                    let rows: Vec<Value> = csv_text.lines()
                        .map(|line| {
                            let cells: Vec<Value> = line.split(',')
                                .map(|cell| Value::String(cell.trim().to_string()))
                                .collect();
                            Value::Array(cells)
                        })
                        .collect();
                    Ok(Value::Matrix(rows.into_iter()
                        .map(|v| if let Value::Array(cells) = v { cells } else { vec![] })
                        .collect()))
                }
                "csv::stringify" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("csv::stringify() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Matrix(rows) => {
                            let csv_lines: Vec<String> = rows.iter()
                                .map(|row| {
                                    row.iter()
                                        .map(|cell| cell.to_string())
                                        .collect::<Vec<_>>()
                                        .join(",")
                                })
                                .collect();
                            Ok(Value::String(csv_lines.join("\n")))
                        }
                        _ => Err(Error::TypeError("csv::stringify() expects 2D array".to_string())),
                    }
                }
                "csv::headers" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("csv::headers() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Matrix(rows) => {
                            if rows.is_empty() {
                                Ok(Value::Array(vec![]))
                            } else {
                                Ok(Value::Array(rows[0].clone()))
                            }
                        }
                        _ => Err(Error::TypeError("csv::headers() expects 2D array".to_string())),
                    }
                }
                "csv::rows" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("csv::rows() expects 1 argument".to_string()));
                    }
                    let val = self.eval_expression(&args[0])?;
                    match val {
                        Value::Matrix(mut rows) => {
                            if !rows.is_empty() {
                                rows.remove(0);
                            }
                            Ok(Value::Matrix(rows))
                        }
                        _ => Err(Error::TypeError("csv::rows() expects 2D array".to_string())),
                    }
                }
                "csv::getColumn" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("csv::getColumn() expects 2 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let col_name = self.eval_expression(&args[1])?.to_string();
                    
                    match data {
                        Value::Matrix(rows) => {
                            if rows.is_empty() {
                                return Ok(Value::Array(vec![]));
                            }
                            
                            let headers = &rows[0];
                            let col_idx = headers.iter().position(|h| h.to_string() == col_name);
                            
                            match col_idx {
                                Some(idx) => {
                                    let column: Vec<Value> = rows.iter().skip(1)
                                        .map(|row| {
                                            if idx < row.len() {
                                                row[idx].clone()
                                            } else {
                                                Value::String("".to_string())
                                            }
                                        })
                                        .collect();
                                    Ok(Value::Array(column))
                                }
                                None => Ok(Value::Array(vec![])),
                            }
                        }
                        _ => Err(Error::TypeError("csv::getColumn() expects 2D array".to_string())),
                    }
                }
                "csv::filter" => {
                    if args.len() != 3 {
                        return Err(Error::RuntimeError("csv::filter() expects 3 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let col_name = self.eval_expression(&args[1])?.to_string();
                    let filter_val = self.eval_expression(&args[2])?.to_string();
                    
                    match data {
                        Value::Matrix(rows) => {
                            if rows.is_empty() {
                                return Ok(Value::Matrix(vec![]));
                            }
                            
                            let headers = rows[0].clone();
                            let col_idx = headers.iter().position(|h| h.to_string() == col_name);
                            
                            match col_idx {
                                Some(idx) => {
                                    let mut result = vec![headers];
                                    for row in rows.iter().skip(1) {
                                        if idx < row.len() && row[idx].to_string() == filter_val {
                                            result.push(row.clone());
                                        }
                                    }
                                    Ok(Value::Matrix(result))
                                }
                                None => Ok(Value::Matrix(vec![headers])),
                            }
                        }
                        _ => Err(Error::TypeError("csv::filter() expects 2D array".to_string())),
                    }
                }
                "csv::sort" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("csv::sort() expects 2 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let col_name = self.eval_expression(&args[1])?.to_string();
                    
                    match data {
                        Value::Matrix(mut rows) => {
                            if rows.is_empty() {
                                return Ok(Value::Matrix(rows));
                            }
                            
                            let headers = rows[0].clone();
                            let col_idx = headers.iter().position(|h| h.to_string() == col_name);
                            
                            match col_idx {
                                Some(idx) => {
                                    let mut data_rows = rows.drain(1..).collect::<Vec<_>>();
                                    data_rows.sort_by(|a, b| {
                                        let a_val = if idx < a.len() { a[idx].to_string() } else { "".to_string() };
                                        let b_val = if idx < b.len() { b[idx].to_string() } else { "".to_string() };
                                        a_val.cmp(&b_val)
                                    });
                                    
                                    let mut result = vec![headers];
                                    result.extend(data_rows);
                                    Ok(Value::Matrix(result))
                                }
                                None => {
                                    rows.insert(0, headers);
                                    Ok(Value::Matrix(rows))
                                }
                            }
                        }
                        _ => Err(Error::TypeError("csv::sort() expects 2D array".to_string())),
                    }
                }
                "csv::unique" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("csv::unique() expects 2 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let col_name = self.eval_expression(&args[1])?.to_string();
                    
                    match data {
                        Value::Matrix(rows) => {
                            if rows.is_empty() {
                                return Ok(Value::Array(vec![]));
                            }
                            
                            let headers = &rows[0];
                            let col_idx = headers.iter().position(|h| h.to_string() == col_name);
                            
                            match col_idx {
                                Some(idx) => {
                                    let mut unique_vals = vec![];
                                    let mut seen = std::collections::HashSet::new();
                                    
                                    for row in rows.iter().skip(1) {
                                        if idx < row.len() {
                                            let val_str = row[idx].to_string();
                                            if !seen.contains(&val_str) {
                                                seen.insert(val_str);
                                                unique_vals.push(row[idx].clone());
                                            }
                                        }
                                    }
                                    Ok(Value::Array(unique_vals))
                                }
                                None => Ok(Value::Array(vec![])),
                            }
                        }
                        _ => Err(Error::TypeError("csv::unique() expects 2D array".to_string())),
                    }
                }
                "csv::stats" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("csv::stats() expects 2 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let col_name = self.eval_expression(&args[1])?.to_string();
                    
                    match data {
                        Value::Matrix(rows) => {
                            if rows.is_empty() {
                                return Ok(Value::Array(vec![]));
                            }
                            
                            let headers = &rows[0];
                            let col_idx = headers.iter().position(|h| h.to_string() == col_name);
                            
                            match col_idx {
                                Some(idx) => {
                                    let mut values = vec![];
                                    for row in rows.iter().skip(1) {
                                        if idx < row.len() {
                                            if let Ok(num) = row[idx].to_float() {
                                                values.push(num);
                                            }
                                        }
                                    }
                                    
                                    if values.is_empty() {
                                        return Ok(Value::Array(vec![]));
                                    }
                                    
                                    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
                                    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                                    let mean = values.iter().sum::<f64>() / values.len() as f64;
                                    let count = values.len() as i64;
                                    let sum = values.iter().sum::<f64>();
                                    
                                    Ok(Value::Array(vec![
                                        Value::Float(min),
                                        Value::Float(max),
                                        Value::Float(mean),
                                        Value::Int(count),
                                        Value::Float(sum),
                                    ]))
                                }
                                None => Ok(Value::Array(vec![])),
                            }
                        }
                        _ => Err(Error::TypeError("csv::stats() expects 2D array".to_string())),
                    }
                }
                "csv::min" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("csv::min() expects 2 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let col_name = self.eval_expression(&args[1])?.to_string();
                    
                    match data {
                        Value::Matrix(rows) => {
                            if rows.is_empty() {
                                return Err(Error::RuntimeError("Empty CSV data".to_string()));
                            }
                            
                            let headers = &rows[0];
                            let col_idx = headers.iter().position(|h| h.to_string() == col_name);
                            
                            match col_idx {
                                Some(idx) => {
                                    let mut min_val = f64::INFINITY;
                                    for row in rows.iter().skip(1) {
                                        if idx < row.len() {
                                            if let Ok(num) = row[idx].to_float() {
                                                if num < min_val {
                                                    min_val = num;
                                                }
                                            }
                                        }
                                    }
                                    Ok(Value::Float(min_val))
                                }
                                None => Err(Error::RuntimeError("Column not found".to_string())),
                            }
                        }
                        _ => Err(Error::TypeError("csv::min() expects 2D array".to_string())),
                    }
                }
                "csv::max" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("csv::max() expects 2 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let col_name = self.eval_expression(&args[1])?.to_string();
                    
                    match data {
                        Value::Matrix(rows) => {
                            if rows.is_empty() {
                                return Err(Error::RuntimeError("Empty CSV data".to_string()));
                            }
                            
                            let headers = &rows[0];
                            let col_idx = headers.iter().position(|h| h.to_string() == col_name);
                            
                            match col_idx {
                                Some(idx) => {
                                    let mut max_val = f64::NEG_INFINITY;
                                    for row in rows.iter().skip(1) {
                                        if idx < row.len() {
                                            if let Ok(num) = row[idx].to_float() {
                                                if num > max_val {
                                                    max_val = num;
                                                }
                                            }
                                        }
                                    }
                                    Ok(Value::Float(max_val))
                                }
                                None => Err(Error::RuntimeError("Column not found".to_string())),
                            }
                        }
                        _ => Err(Error::TypeError("csv::max() expects 2D array".to_string())),
                    }
                }
                "csv::mean" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("csv::mean() expects 2 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let col_name = self.eval_expression(&args[1])?.to_string();
                    
                    match data {
                        Value::Matrix(rows) => {
                            if rows.is_empty() {
                                return Err(Error::RuntimeError("Empty CSV data".to_string()));
                            }
                            
                            let headers = &rows[0];
                            let col_idx = headers.iter().position(|h| h.to_string() == col_name);
                            
                            match col_idx {
                                Some(idx) => {
                                    let mut sum = 0.0;
                                    let mut count = 0;
                                    for row in rows.iter().skip(1) {
                                        if idx < row.len() {
                                            if let Ok(num) = row[idx].to_float() {
                                                sum += num;
                                                count += 1;
                                            }
                                        }
                                    }
                                    if count == 0 {
                                        return Err(Error::RuntimeError("No numeric values in column".to_string()));
                                    }
                                    Ok(Value::Float(sum / count as f64))
                                }
                                None => Err(Error::RuntimeError("Column not found".to_string())),
                            }
                        }
                        _ => Err(Error::TypeError("csv::mean() expects 2D array".to_string())),
                    }
                }
                "csv::rowCount" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("csv::rowCount() expects 1 argument".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    match data {
                        Value::Matrix(rows) => {
                            let count = if rows.is_empty() { 0 } else { rows.len() - 1 };
                            Ok(Value::Int(count as i64))
                        }
                        _ => Err(Error::TypeError("csv::rowCount() expects 2D array".to_string())),
                    }
                }
                "csv::columnCount" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("csv::columnCount() expects 1 argument".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    match data {
                        Value::Matrix(rows) => {
                            let count = if rows.is_empty() { 0 } else { rows[0].len() };
                            Ok(Value::Int(count as i64))
                        }
                        _ => Err(Error::TypeError("csv::columnCount() expects 2D array".to_string())),
                    }
                }
                "csv::select" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("csv::select() expects 2 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let col_names_val = self.eval_expression(&args[1])?;
                    
                    match (data, col_names_val) {
                        (Value::Matrix(rows), Value::Array(col_names)) => {
                            if rows.is_empty() {
                                return Ok(Value::Matrix(vec![]));
                            }
                            
                            let headers = &rows[0];
                            let col_indices: Vec<usize> = col_names.iter()
                                .filter_map(|name| {
                                    headers.iter().position(|h| h.to_string() == name.to_string())
                                })
                                .collect();
                            
                            if col_indices.is_empty() {
                                return Ok(Value::Matrix(vec![]));
                            }
                            
                            let mut result = vec![];
                            for row in rows {
                                let selected: Vec<Value> = col_indices.iter()
                                    .map(|&idx| {
                                        if idx < row.len() {
                                            row[idx].clone()
                                        } else {
                                            Value::String("".to_string())
                                        }
                                    })
                                    .collect();
                                result.push(selected);
                            }
                            Ok(Value::Matrix(result))
                        }
                        _ => Err(Error::TypeError("csv::select() expects 2D array and column array".to_string())),
                    }
                }
                "csv::removeDuplicates" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("csv::removeDuplicates() expects 1 argument".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    match data {
                        Value::Matrix(rows) => {
                            if rows.is_empty() {
                                return Ok(Value::Matrix(vec![]));
                            }
                            
                            let mut result = vec![rows[0].clone()];
                            let mut seen = std::collections::HashSet::new();
                            
                            for row in rows.iter().skip(1) {
                                let row_str = format!("{:?}", row);
                                if !seen.contains(&row_str) {
                                    seen.insert(row_str);
                                    result.push(row.clone());
                                }
                            }
                            Ok(Value::Matrix(result))
                        }
                        _ => Err(Error::TypeError("csv::removeDuplicates() expects 2D array".to_string())),
                    }
                }
                "csv::addRow" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("csv::addRow() expects 2 arguments".to_string()));
                    }
                    let data = self.eval_expression(&args[0])?;
                    let new_row = self.eval_expression(&args[1])?;
                    
                    match (data, new_row) {
                        (Value::Matrix(mut rows), Value::Array(row_values)) => {
                            rows.push(row_values);
                            Ok(Value::Matrix(rows))
                        }
                        _ => Err(Error::TypeError("csv::addRow() expects 2D array and row array".to_string())),
                    }
                }
                // Markdown Functions
                "markdown::parse" | "markdown::toHtml" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("markdown::parse() expects 1 argument".to_string()));
                    }
                    let md_text = match self.eval_expression(&args[0])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("Markdown text must be string".to_string())),
                    };
                    let options = comrak::ComrakOptions::default();
                    let html = comrak::markdown_to_html(&md_text, &options);
                    Ok(Value::String(html))
                }
                // Excel Functions
                "excel::read" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("excel::read() expects 1 argument".to_string()));
                    }
                    let path = match self.eval_expression(&args[0])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("File path must be string".to_string())),
                    };
                    
                    use calamine::{Reader, Xlsx, open_workbook, Data as ExcelData};
                    let workbook_result: std::result::Result<Xlsx<_>, calamine::XlsxError> = open_workbook(&path);
                    let mut workbook = match workbook_result {
                        Ok(wb) => wb,
                        Err(e) => return Err(Error::RuntimeError(format!("Failed to open workbook: {}", e))),
                    };
                    
                    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
                        let mut rows: Vec<Vec<Value>> = Vec::new();
                        for row in range.rows() {
                                let cells: Vec<Value> = row.iter()
                                    .map(|cell| match cell {
                                        ExcelData::String(s) => Value::String(s.to_string()),
                                        ExcelData::Float(f) => Value::Float(*f),
                                        ExcelData::Int(i) => Value::Int(*i),
                                        ExcelData::Bool(b) => Value::Bool(*b),
                                        ExcelData::Empty => Value::String("".to_string()),
                                        ExcelData::DateTime(d) => Value::String(d.to_string()),
                                        ExcelData::Error(e) => Value::String(format!("{:?}", e)),
                                        ExcelData::DateTimeIso(d) => Value::String(d.clone()),
                                        ExcelData::DurationIso(d) => Value::String(d.clone()),
                                    })
                                    .collect();
                                rows.push(cells);
                        }
                        Ok(Value::Matrix(rows))
                    } else {
                        Ok(Value::Matrix(vec![]))
                    }
                }
                "excel::write" => {
                    if args.len() != 2 {
                        return Err(Error::RuntimeError("excel::write() expects 2 arguments".to_string()));
                    }
                    let path_val = self.eval_expression(&args[0])?;
                    let path = match path_val {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("File path must be string".to_string())),
                    };
                    let data_val = self.eval_expression(&args[1])?;
                    let data = match data_val {
                        Value::Matrix(m) => m,
                        Value::Array(a) => {
                             let mut matrix = Vec::new();
                             for row in a {
                                 if let Value::Array(cells) = row {
                                     matrix.push(cells);
                                 } else {
                                     // Handle scalar in array by wrapping in array? No, skip or error.
                                     // For simplicity, just skip non-array rows
                                 }
                             }
                             matrix
                        }
                        _ => return Err(Error::TypeError("Data must be a matrix/2D array".to_string())),
                    };
                    
                    use rust_xlsxwriter::{Workbook};
                    let mut workbook = Workbook::new();
                    let worksheet = workbook.add_worksheet();
                    
                    for (row_idx, row_data) in data.iter().enumerate() {
                        for (col_idx, cell_val) in row_data.iter().enumerate() {
                            match cell_val {
                                Value::String(s) => { let _ = worksheet.write_string(row_idx as u32, col_idx as u16, s); },
                                Value::Int(i) => { let _ = worksheet.write_number(row_idx as u32, col_idx as u16, *i as f64); },
                                Value::Float(f) => { let _ = worksheet.write_number(row_idx as u32, col_idx as u16, *f); },
                                Value::Bool(b) => { let _ = worksheet.write_boolean(row_idx as u32, col_idx as u16, *b); },
                                _ => { let _ = worksheet.write_string(row_idx as u32, col_idx as u16, &cell_val.to_string()); },
                            };
                        }
                    }
                    
                    match workbook.save(&path) {
                        Ok(_) => Ok(Value::Bool(true)),
                        Err(_) => Ok(Value::Bool(false)),
                    }
                }
                // Graphics Functions
                "graphics::title" => {
                     if args.len() != 1 {
                         return Err(Error::RuntimeError("graphics::title() expects 1 argument".to_string()));
                     }
                     if let Value::String(t) = self.eval_expression(&args[0])? {
                         self.chart_config.title = t;
                     }
                     Ok(Value::Bool(true))
                }
                "graphics::plot" => {
                    if args.len() < 2 {
                        return Err(Error::RuntimeError("graphics::plot() expects at least x and y arrays".to_string()));
                    }
                    let x_val = self.eval_expression(&args[0])?;
                    let y_val = self.eval_expression(&args[1])?;
                    
                    let x: Vec<f64> = match x_val {
                        Value::Array(arr) => arr.iter().map(|v| match v {
                            Value::Int(i) => *i as f64,
                            Value::Float(f) => *f,
                            _ => 0.0,
                        }).collect(),
                        _ => return Err(Error::TypeError("X must be array".to_string())),
                    };
                    let y: Vec<f64> = match y_val {
                        Value::Array(arr) => arr.iter().map(|v| match v {
                            Value::Int(i) => *i as f64,
                            Value::Float(f) => *f,
                            _ => 0.0,
                        }).collect(),
                        _ => return Err(Error::TypeError("Y must be array".to_string())),
                    };
                    
                    let label = if args.len() > 2 {
                        match self.eval_expression(&args[2])? { Value::String(s) => s, _ => "Series".to_string() }
                    } else { "Series".to_string() };

                    let color = if args.len() > 3 {
                         match self.eval_expression(&args[3])? { Value::String(s) => s, _ => "blue".to_string() }
                    } else { "blue".to_string() };

                    self.chart_config.series.push(Series::Line { x, y, label, color });
                    Ok(Value::Bool(true))
                }
                "graphics::scatter" => {
                    if args.len() < 2 {
                        return Err(Error::RuntimeError("graphics::scatter() expects at least x and y arrays".to_string()));
                    }
                    let x_val = self.eval_expression(&args[0])?;
                    let y_val = self.eval_expression(&args[1])?;
                    
                    let x: Vec<f64> = match x_val {
                        Value::Array(arr) => arr.iter().map(|v| match v {
                            Value::Int(i) => *i as f64,
                            Value::Float(f) => *f,
                            _ => 0.0,
                        }).collect(),
                        _ => return Err(Error::TypeError("X must be array".to_string())),
                    };
                    let y: Vec<f64> = match y_val {
                        Value::Array(arr) => arr.iter().map(|v| match v {
                            Value::Int(i) => *i as f64,
                            Value::Float(f) => *f,
                            _ => 0.0,
                        }).collect(),
                        _ => return Err(Error::TypeError("Y must be array".to_string())),
                    };
                    
                    let label = if args.len() > 2 {
                        match self.eval_expression(&args[2])? { Value::String(s) => s, _ => "Series".to_string() }
                    } else { "Series".to_string() };

                    let color = if args.len() > 3 {
                         match self.eval_expression(&args[3])? { Value::String(s) => s, _ => "red".to_string() }
                    } else { "red".to_string() };

                    self.chart_config.series.push(Series::Scatter { x, y, label, color });
                    Ok(Value::Bool(true))
                }
                "graphics::bar" => {
                    if args.len() < 2 {
                        return Err(Error::RuntimeError("graphics::bar() expects labels and values arrays".to_string()));
                    }
                    let labels_val = self.eval_expression(&args[0])?;
                    let values_val = self.eval_expression(&args[1])?;
                    
                    let labels: Vec<String> = match labels_val {
                        Value::Array(arr) => arr.iter().map(|v| v.to_string()).collect(),
                        _ => return Err(Error::TypeError("Labels must be array".to_string())),
                    };
                    let values: Vec<f64> = match values_val {
                        Value::Array(arr) => arr.iter().map(|v| match v {
                            Value::Int(i) => *i as f64,
                            Value::Float(f) => *f,
                            _ => 0.0,
                        }).collect(),
                        _ => return Err(Error::TypeError("Values must be array".to_string())),
                    };
                    
                    let label = if args.len() > 2 {
                        match self.eval_expression(&args[2])? { Value::String(s) => s, _ => "Data".to_string() }
                    } else { "Data".to_string() };
                    
                    let color = if args.len() > 3 {
                         match self.eval_expression(&args[3])? { Value::String(s) => s, _ => "green".to_string() }
                    } else { "green".to_string() };

                    self.chart_config.series.push(Series::Bar { labels, values, label, color });
                    Ok(Value::Bool(true))
                }
                "graphics::save" => {
                    if args.len() != 1 {
                        return Err(Error::RuntimeError("graphics::save() expects filename".to_string()));
                    }
                    let filename = match self.eval_expression(&args[0])? {
                        Value::String(s) => s,
                        _ => return Err(Error::TypeError("Filename must be string".to_string())),
                    };
                    
                    use plotters::prelude::*;
                    let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
                    root.fill(&WHITE).map_err(|e| Error::RuntimeError(format!("Drawing error: {:?}", e)))?;
                    
                    // Determine ranges
                    let mut x_min = f64::INFINITY;
                    let mut x_max = f64::NEG_INFINITY;
                    let mut y_min = f64::INFINITY;
                    let mut y_max = f64::NEG_INFINITY;
                    
                    for s in &self.chart_config.series {
                        match s {
                            Series::Line { x, y, .. } | Series::Scatter { x, y, .. } => {
                                for val in x { if *val < x_min { x_min = *val; } if *val > x_max { x_max = *val; } }
                                for val in y { if *val < y_min { y_min = *val; } if *val > y_max { y_max = *val; } }
                            },
                            Series::Bar { values, .. } => {
                                x_min = 0.0;
                                x_max = values.len() as f64;
                                y_min = 0.0;
                                for val in values { if *val > y_max { y_max = *val; } }
                            }
                        }
                    }
                    
                    if x_min == f64::INFINITY { x_min = 0.0; x_max = 10.0; }
                    if y_min == f64::INFINITY { y_min = 0.0; y_max = 10.0; }
                    
                    // Add padding
                    let x_range = x_max - x_min;
                    let y_range = y_max - y_min;
                    x_min -= x_range * 0.1;
                    x_max += x_range * 0.1;
                    y_min -= y_range * 0.1;
                    y_max += y_range * 0.1;

                    let mut chart = ChartBuilder::on(&root)
                        //.caption(&self.chart_config.title, ("sans-serif", 40).into_font())
                        .margin(5)
                        //.x_label_area_size(30)
                        //.y_label_area_size(30)
                        .build_cartesian_2d(x_min..x_max, y_min..y_max)
                        .map_err(|e| Error::RuntimeError(format!("Chart build error: {:?}", e)))?;

                    chart.configure_mesh().draw().map_err(|e| Error::RuntimeError(format!("Mesh error: {:?}", e)))?;

                    for s in &self.chart_config.series {
                        let color = match s {
                            Series::Line { color, .. } => color,
                            Series::Scatter { color, .. } => color,
                            Series::Bar { color, .. } => color,
                        };
                        
                        let plot_color = match color.as_str() {
                            "red" => RED,
                            "blue" => BLUE,
                            "green" => GREEN,
                            "yellow" => YELLOW,
                            "black" => BLACK,
                            "cyan" => CYAN,
                            "magenta" => MAGENTA,
                            _ => BLUE,
                        };

                        match s {
                            Series::Line { x, y, label, .. } => {
                                let points: Vec<(f64, f64)> = x.iter().zip(y.iter()).map(|(a, b)| (*a, *b)).collect();
                                chart.draw_series(LineSeries::new(points, plot_color.stroke_width(2)))
                                    .map_err(|e| Error::RuntimeError(format!("Draw error: {:?}", e)))?;
                                    //.label(label)
                                    //.legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], plot_color.filled()));
                            },
                            Series::Scatter { x, y, label, .. } => {
                                let points: Vec<(f64, f64)> = x.iter().zip(y.iter()).map(|(a, b)| (*a, *b)).collect();
                                chart.draw_series(PointSeries::of_element(
                                    points,
                                    5,
                                    plot_color.filled(),
                                    &|c, s, st| {
                                        return EmptyElement::at(c) + Circle::new((0,0), s, st.filled());
                                    },
                                ))
                                .map_err(|e| Error::RuntimeError(format!("Draw error: {:?}", e)))?;
                                //.label(label)
                                //.legend(move |(x, y)| Circle::new((x + 10, y), 5, plot_color.filled()));
                            },
                            Series::Bar { labels, values, label, .. } => {
                                // Simplified bar handling for 2D cartesian using Rectangles
                                let bars: Vec<(f64, f64)> = values.iter().enumerate().map(|(i, v)| (i as f64, *v)).collect();
                                chart.draw_series(
                                    bars.iter().map(|(x, y)| {
                                        Rectangle::new([(*x - 0.4, 0.0), (*x + 0.4, *y)], plot_color.filled())
                                    })
                                ).map_err(|e| Error::RuntimeError(format!("Draw error: {:?}", e)))?;
                                //.label(label)
                                //.legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], plot_color.filled()));
                            }
                        }
                    }
                    
                    /*chart.configure_series_labels()
                        .background_style(&WHITE.mix(0.8))
                        .border_style(&BLACK)
                        .draw()
                        .map_err(|e| Error::RuntimeError(format!("Legend error: {:?}", e)))?;*/
                        
                    // Reset chart config after save
                    self.chart_config = ChartConfig::new();
                    
                    Ok(Value::Bool(true))
                }
                _ => {
                    if let Some(func_def) = self.functions.get(name).cloned() {
                        self.execute_user_function(func_def, args)
                    } else {
                        Err(Error::RuntimeError(format!("Unknown function: {}", name)))
                    }
                }
            }
        } else {
            Err(Error::RuntimeError("Invalid function call".to_string()))
        }
    }

    fn execute_user_function(&mut self, func_def: FunctionDef, args: &[Expression]) -> Result<Value> {
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expression(arg)?);
        }
        
        if arg_values.len() != func_def.params.len() {
             return Err(Error::RuntimeError(format!("Function expects {} arguments, got {}", func_def.params.len(), arg_values.len())));
        }
        
        self.push_scope();
        
        for ((name, _ty), val) in func_def.params.iter().zip(arg_values.into_iter()) {
            self.declare_variable(name.clone(), val);
        }
        
        let mut result = Value::Bool(false);
        for stmt in &func_def.body {
            match self.execute_statement(stmt) {
                Ok(_) => {},
                Err(Error::Return(val)) => {
                    result = val;
                    break;
                },
                Err(e) => {
                    self.pop_scope();
                    return Err(e);
                }
            }
        }
        
        self.pop_scope();
        Ok(result)
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
