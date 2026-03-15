use linea_ast::{Program, Statement, Expression, BinaryOp, UnaryOp};
use linea_core::Result;


pub fn generate_rust_code(program: &Program) -> Result<String> {
    let mut generator = RustGenerator::new();
    generator.generate(program)
}

struct RustGenerator {
    code: String,
    indent_level: usize,
    variable_types: std::collections::HashMap<String, String>,
}

impl RustGenerator {
    fn new() -> Self {
        RustGenerator {
            code: String::new(),
            indent_level: 0,
            variable_types: std::collections::HashMap::new(),
        }
    }

    fn generate(&mut self, program: &Program) -> Result<String> {
        self.emit_line("fn main() {");
        self.indent_level += 1;

        for statement in &program.statements {
            self.generate_statement(statement)?;
        }

        self.indent_level -= 1;
        self.emit_line("}");

        let output = format!(
            "{}\n{}",
            "// Generated Linea Rust code\nuse std::io::Write;",
            self.code
        );
        Ok(output)
    }

    fn generate_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Import { module, items } => {
                // Generate module comments in the Rust code
                self.emit_line(&format!("// Import module: {} (items: {})", module, items.join(", ")));
                Ok(())
            }
            Statement::VarDeclaration { name, expr } => {
                let (rust_expr, type_name) = self.generate_expression(expr)?;
                self.variable_types.insert(name.clone(), type_name.clone());
                self.emit_line(&format!("let mut {} : {} = {};", name, type_name, rust_expr));
                Ok(())
            }
            Statement::VarUpdate { name, expr } => {
                let (rust_expr, _) = self.generate_expression(expr)?;
                self.emit_line(&format!("{} = {};", name, rust_expr));
                Ok(())
            }
            Statement::Display(expr) => {
                let (rust_expr, rust_ty) = self.generate_expression(expr)?;
                if rust_ty.starts_with("Vec") {
                    self.emit_line(&format!("println!(\"{{:?}}\", {});", rust_expr));
                } else {
                    self.emit_line(&format!("println!(\"{{}}\", {});", rust_expr));
                }
                Ok(())
            }
            Statement::For { var, start, end, body } => {
                let (start_expr, _) = self.generate_expression(start)?;
                let (end_expr, _) = self.generate_expression(end)?;
                self.emit_line(&format!("for {} in {}..={} {{", var, start_expr, end_expr));
                self.indent_level += 1;

                for stmt in body {
                    self.generate_statement(stmt)?;
                }

                self.indent_level -= 1;
                self.emit_line("}");
                Ok(())
            }
            Statement::While { condition, body } => {
                let (cond_expr, _) = self.generate_expression(condition)?;
                self.emit_line(&format!("while {} {{", cond_expr));
                self.indent_level += 1;

                for stmt in body {
                    self.generate_statement(stmt)?;
                }

                self.indent_level -= 1;
                self.emit_line("}");
                Ok(())
            }
            Statement::If { condition, then_body, else_body } => {
                let (cond_expr, _) = self.generate_expression(condition)?;
                self.emit_line(&format!("if {} {{", cond_expr));
                self.indent_level += 1;

                for stmt in then_body {
                    self.generate_statement(stmt)?;
                }

                self.indent_level -= 1;

                if let Some(else_stmts) = else_body {
                    self.emit_line("} else {");
                    self.indent_level += 1;

                    for stmt in else_stmts {
                        self.generate_statement(stmt)?;
                    }

                    self.indent_level -= 1;
                    self.emit_line("}");
                } else {
                    self.emit_line("}");
                }
                Ok(())
            }
            Statement::Expression(expr) => {
                let (rust_expr, _) = self.generate_expression(expr)?;
                self.emit_line(&format!("{};", rust_expr));
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn generate_expression(&mut self, expr: &Expression) -> Result<(String, String)> {
        match expr {
            Expression::Integer(n) => Ok((n.to_string(), "i64".to_string())),
            Expression::Float(f) => Ok((f.to_string(), "f64".to_string())),
            Expression::String(s) => Ok((format!("\"{}\".to_string()", s), "String".to_string())),
            Expression::Bool(b) => Ok((b.to_string(), "bool".to_string())),
            Expression::Identifier(name) => {
                let ty = self.variable_types.get(name).cloned().unwrap_or_else(|| "i64".to_string());
                Ok((name.clone(), ty))
            }
            Expression::Binary { left, op, right } => {
                let (left_expr, left_ty) = self.generate_expression(left)?;
                let (right_expr, right_ty) = self.generate_expression(right)?;

                match op {
                    BinaryOp::Add => {
                        if left_ty.starts_with("Vec") && right_ty.starts_with("Vec") {
                            // Array + Array
                            let elem_ty = if left_ty.starts_with("Vec<") && left_ty.ends_with(">") {
                                left_ty[4..left_ty.len()-1].to_string()
                            } else {
                                "i64".to_string()
                            };
                            let code = format!(
                                "{{ let mut result = Vec::new(); for (a, b) in {}.iter().zip({}.iter()) {{ result.push(a + b); }} result }}",
                                left_expr, right_expr
                            );
                            Ok((code, format!("Vec<{}>", elem_ty)))
                        } else if left_ty == "String" || right_ty == "String" {
                            let left_str = if left_ty == "String" {
                                format!("{}", left_expr)
                            } else if left_ty.starts_with("Vec") {
                                // For arrays, use debug format
                                format!("(format!(\"{{:?}}\", {}))", left_expr)
                            } else {
                                format!("(({}).to_string())", left_expr)
                            };
                            let right_str = if right_ty == "String" {
                                format!("{}", right_expr)
                            } else if right_ty.starts_with("Vec") {
                                // For arrays, use debug format
                                format!("(format!(\"{{:?}}\", {}))", right_expr)
                            } else {
                                format!("(({}).to_string())", right_expr)
                            };
                            Ok((format!("format!(\"{{}}{{}}\", {}, {})", left_str, right_str), "String".to_string()))
                        } else {
                            Ok((format!("({} + {})", left_expr, right_expr), left_ty.clone()))
                        }
                    }
                    BinaryOp::Subtract => {
                        if left_ty.starts_with("Vec") && right_ty.starts_with("Vec") {
                            let elem_ty = if left_ty.starts_with("Vec<") && left_ty.ends_with(">") {
                                left_ty[4..left_ty.len()-1].to_string()
                            } else {
                                "i64".to_string()
                            };
                            let code = format!(
                                "{{ let mut result = Vec::new(); for (a, b) in {}.iter().zip({}.iter()) {{ result.push(a - b); }} result }}",
                                left_expr, right_expr
                            );
                            Ok((code, format!("Vec<{}>", elem_ty)))
                        } else {
                            Ok((format!("({} - {})", left_expr, right_expr), left_ty.clone()))
                        }
                    }
                    BinaryOp::Multiply => {
                        if left_ty.starts_with("Vec") && right_ty.starts_with("Vec") {
                            // Array * Array
                            let elem_ty = if left_ty.starts_with("Vec<") && left_ty.ends_with(">") {
                                left_ty[4..left_ty.len()-1].to_string()
                            } else {
                                "i64".to_string()
                            };
                            let code = format!(
                                "{{ let mut result = Vec::new(); for (a, b) in {}.iter().zip({}.iter()) {{ result.push(a * b); }} result }}",
                                left_expr, right_expr
                            );
                            Ok((code, format!("Vec<{}>", elem_ty)))
                        } else if left_ty.starts_with("Vec") && !right_ty.starts_with("Vec") {
                            // Array * Scalar
                            let elem_ty = if left_ty.starts_with("Vec<") && left_ty.ends_with(">") {
                                left_ty[4..left_ty.len()-1].to_string()
                            } else {
                                "i64".to_string()
                            };
                            let code = format!(
                                "{{ let mut result = Vec::new(); for a in {}.iter() {{ result.push(a * {}); }} result }}",
                                left_expr, right_expr
                            );
                            Ok((code, format!("Vec<{}>", elem_ty)))
                        } else if !left_ty.starts_with("Vec") && right_ty.starts_with("Vec") {
                            // Scalar * Array
                            let elem_ty = if right_ty.starts_with("Vec<") && right_ty.ends_with(">") {
                                right_ty[4..right_ty.len()-1].to_string()
                            } else {
                                "i64".to_string()
                            };
                            let code = format!(
                                "{{ let mut result = Vec::new(); for b in {}.iter() {{ result.push({} * b); }} result }}",
                                right_expr, left_expr
                            );
                            Ok((code, format!("Vec<{}>", elem_ty)))
                        } else {
                            Ok((format!("({} * {})", left_expr, right_expr), left_ty.clone()))
                        }
                    }
                    BinaryOp::Divide => {
                        if left_ty.starts_with("Vec") && right_ty.starts_with("Vec") {
                            let elem_ty = if left_ty.starts_with("Vec<") && left_ty.ends_with(">") {
                                left_ty[4..left_ty.len()-1].to_string()
                            } else {
                                "i64".to_string()
                            };
                            let code = format!(
                                "{{ let mut result = Vec::new(); for (a, b) in {}.iter().zip({}.iter()) {{ if *b != 0 {{ result.push(a / b); }} }} result }}",
                                left_expr, right_expr
                            );
                            Ok((code, format!("Vec<{}>", elem_ty)))
                        } else {
                            Ok((format!("({} / {})", left_expr, right_expr), left_ty.clone()))
                        }
                    }
                    _ => {
                        let (op_str, result_ty) = match op {
                            BinaryOp::Modulo => ("%", left_ty.clone()),
                            BinaryOp::Power => return self.generate_power(&left_expr, &right_expr, &left_ty),
                            BinaryOp::Equal => ("==", "bool".to_string()),
                            BinaryOp::NotEqual => ("!=", "bool".to_string()),
                            BinaryOp::Less => ("<", "bool".to_string()),
                            BinaryOp::Greater => (">", "bool".to_string()),
                            BinaryOp::LessEqual => ("<=", "bool".to_string()),
                            BinaryOp::GreaterEqual => (">=", "bool".to_string()),
                            BinaryOp::And => ("&&", "bool".to_string()),
                            BinaryOp::Or => ("||", "bool".to_string()),
                            _ => unreachable!(),
                        };

                        let expr_str = format!("({} {} {})", left_expr, op_str, right_expr);
                        Ok((expr_str, result_ty))
                    }
                }
            }
            Expression::Unary { op, expr } => {
                let (inner_expr, ty) = self.generate_expression(expr)?;
                match op {
                    UnaryOp::Negate => Ok((format!("-({})", inner_expr), ty)),
                    UnaryOp::Not => Ok((format!("!({})", inner_expr), "bool".to_string())),
                }
            }
            Expression::Array(elements) => {
                let mut elem_exprs = Vec::new();
                let mut elem_type = "i64".to_string();
                for elem in elements {
                    let (expr, ty) = self.generate_expression(elem)?;
                    elem_exprs.push(expr);
                    elem_type = ty;
                }
                Ok((format!("vec![{}]", elem_exprs.join(", ")), format!("Vec<{}>", elem_type)))
            }
            Expression::Index { expr, index } => {
                let (expr_code, expr_ty) = self.generate_expression(expr)?;
                let (idx_code, _) = self.generate_expression(index)?;
                // Remove Vec< and > from type for element type
                let elem_type = if expr_ty.starts_with("Vec<") && expr_ty.ends_with(">") {
                    expr_ty[4..expr_ty.len()-1].to_string()
                } else {
                    "i64".to_string()
                };
                
                // For nested Vec types (Vec<Vec<...>>), use .clone()
                if elem_type.contains("Vec") {
                    Ok((format!("{}[{} as usize].clone()", expr_code, idx_code), elem_type))
                } else {
                    Ok((format!("{}[{} as usize]", expr_code, idx_code), elem_type))
                }
            }
            Expression::Slice { expr, start, end, step } => {
                let (expr_code, expr_ty) = self.generate_expression(expr)?;
                let start_code = if let Some(s) = start {
                    let (code, _) = self.generate_expression(s)?;
                    format!("{} as usize", code)
                } else {
                    "0".to_string()
                };
                let end_code = if let Some(e) = end {
                    let (code, _) = self.generate_expression(e)?;
                    format!("{} as usize", code)
                } else {
                    format!("{}.len()", expr_code)
                };
                
                // For now, simple slice without step (Rust doesn't support step in slicing)
                let elem_type = if expr_ty.starts_with("Vec<") && expr_ty.ends_with(">") {
                    expr_ty[4..expr_ty.len()-1].to_string()
                } else {
                    "i64".to_string()
                };
                
                Ok((format!("{}[{}..{}].to_vec()", expr_code, start_code, end_code), format!("Vec<{}>", elem_type)))
            }
            Expression::Call { func, args } => {
                if let Expression::Identifier(name) = &**func {
                    match name.as_str() {
                        "len" => {
                            if args.len() != 1 {
                                return Ok(("0".to_string(), "i64".to_string()));
                            }
                            let (expr_code, _) = self.generate_expression(&args[0])?;
                            Ok((format!("({}.len() as i64)", expr_code), "i64".to_string()))
                        }
                        "sum" => {
                            if args.len() != 1 {
                                return Ok(("0".to_string(), "i64".to_string()));
                            }
                            let (expr_code, _) = self.generate_expression(&args[0])?;
                            Ok((format!("({}.iter().sum::<i64>())", expr_code), "i64".to_string()))
                        }
                        "mean" => {
                            if args.len() != 1 {
                                return Ok(("0".to_string(), "f64".to_string()));
                            }
                            let (expr_code, _) = self.generate_expression(&args[0])?;
                            Ok((format!("({}.iter().map(|x| *x as f64).sum::<f64>() / {}.len() as f64)", expr_code, expr_code), "f64".to_string()))
                        }
                        "max" => {
                            if args.len() != 1 {
                                return Ok(("0".to_string(), "i64".to_string()));
                            }
                            let (expr_code, _) = self.generate_expression(&args[0])?;
                            Ok((format!("({}.iter().copied().max().unwrap_or(0))", expr_code), "i64".to_string()))
                        }
                        "min" => {
                            if args.len() != 1 {
                                return Ok(("0".to_string(), "i64".to_string()));
                            }
                            let (expr_code, _) = self.generate_expression(&args[0])?;
                            Ok((format!("({}.iter().copied().min().unwrap_or(0))", expr_code), "i64".to_string()))
                        }
                        "shape" => {
                            if args.len() != 1 {
                                return Ok(("vec![]".to_string(), "Vec<i64>".to_string()));
                            }
                            let (expr_code, _) = self.generate_expression(&args[0])?;
                            Ok((format!("vec![{}.len() as i64]", expr_code), "Vec<i64>".to_string()))
                        }
                        "asFloat" => {
                            if args.len() != 1 {
                                return Ok(("0.0".to_string(), "f64".to_string()));
                            }
                            let (expr_code, expr_ty) = self.generate_expression(&args[0])?;
                            if expr_ty.starts_with("Vec") {
                                Ok((format!("({}.iter().map(|x| *x as f64).collect())", expr_code), "Vec<f64>".to_string()))
                            } else {
                                Ok((format!("({} as f64)", expr_code), "f64".to_string()))
                            }
                        }
                        "asInt" => {
                            if args.len() != 1 {
                                return Ok(("0".to_string(), "i64".to_string()));
                            }
                            let (expr_code, expr_ty) = self.generate_expression(&args[0])?;
                            if expr_ty.starts_with("Vec") {
                                Ok((format!("({}.iter().map(|x| *x as i64).collect::<Vec<i64>>())", expr_code), "Vec<i64>".to_string()))
                            } else {
                                Ok((format!("({} as i64)", expr_code), "i64".to_string()))
                            }
                        }
                        "asString" => {
                            if args.len() != 1 {
                                return Ok(("\"\"".to_string(), "String".to_string()));
                            }
                            let (expr_code, _) = self.generate_expression(&args[0])?;
                            Ok((format!("({}.to_string())", expr_code), "String".to_string()))
                        }
                        _ => Ok(("0".to_string(), "i64".to_string())),
                    }
                } else {
                    Ok(("0".to_string(), "i64".to_string()))
                }
            }
            Expression::TypeCast { expr, target_type } => {
                let (inner_expr, inner_ty) = self.generate_expression(expr)?;
                match target_type {
                    linea_core::Type::Int => {
                        if inner_ty == "String" {
                            Ok((format!("({}.parse::<i64>().unwrap_or(0))", inner_expr), "i64".to_string()))
                        } else {
                            Ok((format!("({} as i64)", inner_expr), "i64".to_string()))
                        }
                    }
                    linea_core::Type::Float => {
                        if inner_ty == "String" {
                            Ok((format!("({}.parse::<f64>().unwrap_or(0.0))", inner_expr), "f64".to_string()))
                        } else {
                            Ok((format!("({} as f64)", inner_expr), "f64".to_string()))
                        }
                    }
                    linea_core::Type::String => {
                        Ok((format!("(({}).to_string())", inner_expr), "String".to_string()))
                    }
                    linea_core::Type::Bool => {
                        Ok((format!("({} != 0)", inner_expr), "bool".to_string()))
                    }
                    _ => Ok((inner_expr, inner_ty)),
                }
            }
            _ => Ok(("0".to_string(), "i64".to_string())),
        }
    }

    fn generate_power(&self, left: &str, right: &str, ty: &str) -> Result<(String, String)> {
        let func_name = match ty {
            "i64" => "pow",
            "f64" => "powf",
            _ => "pow",
        };
        Ok((format!("({}.{}({} as u32))", left, func_name, right), ty.to_string()))
    }

    fn emit_line(&mut self, line: &str) {
        let indent = "    ".repeat(self.indent_level);
        self.code.push_str(&format!("{}{}\n", indent, line));
    }
}

