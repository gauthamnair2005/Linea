use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Array(Box<Type>),
    Function { params: Vec<Type>, return_type: Box<Type> },
    Unknown,
}

impl Type {
    pub fn can_coerce_to(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Int, Type::Float) => true,
            (Type::Int, Type::String) => true,
            (Type::Float, Type::String) => true,
            (a, b) if a == b => true,
            _ => false,
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::String => "string".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Array(inner) => format!("{}[]", inner.display_name()),
            Type::Function { params, return_type } => {
                let param_names = params.iter().map(|t| t.display_name()).collect::<Vec<_>>();
                format!("({}) -> {}", param_names.join(", "), return_type.display_name())
            }
            Type::Unknown => "unknown".to_string(),
        }
    }
}

pub struct TypeContext {
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeContext {
    pub fn new() -> Self {
        TypeContext {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn declare(&mut self, name: String, ty: Type) -> crate::error::Result<()> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                return Err(crate::error::Error::VariableAlreadyDeclared(name));
            }
            scope.insert(name, ty);
            Ok(())
        } else {
            Err(crate::error::Error::RuntimeError("No scope available".to_string()))
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
}
