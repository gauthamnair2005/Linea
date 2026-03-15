use crate::types::Type;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
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
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
