use crate::types::Type;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),              // 1D: [1, 2, 3]
    Matrix(Vec<Vec<Value>>),        // 2D: [[1, 2], [3, 4]]
    Tensor(Vec<Vec<Vec<Value>>>),   // 3D: [[[1, 2]], [[3, 4]]]
    Null,
}

impl Value {
    pub fn to_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Bool(_) => Type::Bool,
            Value::Array(elements) => {
                if elements.is_empty() {
                    Type::Array(Box::new(Type::Unknown))
                } else {
                    Type::Array(Box::new(elements[0].to_type()))
                }
            }
            Value::Matrix(rows) => {
                if rows.is_empty() || rows[0].is_empty() {
                    Type::Matrix(Box::new(Type::Unknown))
                } else {
                    Type::Matrix(Box::new(rows[0][0].to_type()))
                }
            }
            Value::Tensor(tensors) => {
                if tensors.is_empty() || tensors[0].is_empty() || tensors[0][0].is_empty() {
                    Type::Tensor(Box::new(Type::Unknown))
                } else {
                    Type::Tensor(Box::new(tensors[0][0][0].to_type()))
                }
            }
            Value::Null => Type::Unknown,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Array(elements) => {
                let strs: Vec<String> = elements.iter().map(|v| v.to_string()).collect();
                format!("[{}]", strs.join(", "))
            }
            Value::Matrix(rows) => {
                let row_strs: Vec<String> = rows.iter().map(|row| {
                    let elem_strs: Vec<String> = row.iter().map(|v| v.to_string()).collect();
                    format!("[{}]", elem_strs.join(", "))
                }).collect();
                format!("[{}]", row_strs.join(", "))
            }
            Value::Tensor(tensors) => {
                let tensor_strs: Vec<String> = tensors.iter().map(|tensor| {
                    let row_strs: Vec<String> = tensor.iter().map(|row| {
                        let elem_strs: Vec<String> = row.iter().map(|v| v.to_string()).collect();
                        format!("[{}]", elem_strs.join(", "))
                    }).collect();
                    format!("[{}]", row_strs.join(", "))
                }).collect();
                format!("[{}]", tensor_strs.join(", "))
            }
            Value::Null => "null".to_string(),
        }
    }

    pub fn to_int(&self) -> crate::error::Result<i64> {
        match self {
            Value::Int(n) => Ok(*n),
            Value::Float(f) => Ok(*f as i64),
            Value::String(s) => s.parse::<i64>()
                .map_err(|_| crate::error::Error::TypeError(format!("Cannot convert '{}' to int", s))),
            Value::Bool(b) => Ok(if *b { 1 } else { 0 }),
            _ => Err(crate::error::Error::TypeError(format!("Cannot convert {} to int", self.type_name()))),
        }
    }

    pub fn to_float(&self) -> crate::error::Result<f64> {
        match self {
            Value::Int(n) => Ok(*n as f64),
            Value::Float(f) => Ok(*f),
            Value::String(s) => s.parse::<f64>()
                .map_err(|_| crate::error::Error::TypeError(format!("Cannot convert '{}' to float", s))),
            Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
            _ => Err(crate::error::Error::TypeError(format!("Cannot convert {} to float", self.type_name()))),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Bool(b) => *b,
            Value::Array(a) => !a.is_empty(),
            Value::Matrix(m) => !m.is_empty(),
            Value::Tensor(t) => !t.is_empty(),
            Value::Null => false,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Array(_) => "array",
            Value::Matrix(_) => "matrix",
            Value::Tensor(_) => "tensor",
            Value::Null => "null",
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < 1e-10,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => {
                if a.len() != b.len() { return false; }
                a.iter().zip(b.iter()).all(|(x, y)| x == y)
            }
            (Value::Matrix(a), Value::Matrix(b)) => {
                if a.len() != b.len() { return false; }
                a.iter().zip(b.iter()).all(|(row_a, row_b)| {
                    if row_a.len() != row_b.len() { return false; }
                    row_a.iter().zip(row_b.iter()).all(|(x, y)| x == y)
                })
            }
            (Value::Tensor(a), Value::Tensor(b)) => {
                if a.len() != b.len() { return false; }
                a.iter().zip(b.iter()).all(|(mat_a, mat_b)| {
                    if mat_a.len() != mat_b.len() { return false; }
                    mat_a.iter().zip(mat_b.iter()).all(|(row_a, row_b)| {
                        if row_a.len() != row_b.len() { return false; }
                        row_a.iter().zip(row_b.iter()).all(|(x, y)| x == y)
                    })
                })
            }
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
