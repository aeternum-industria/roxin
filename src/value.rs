use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i128),
    Float(f64),
    Bool(bool),
    Record(Vec<(String, Value)>),
}

impl Value {
    pub fn kind_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::Record(_) => "record",
        }
    }

    pub fn kind_phrase(&self) -> &'static str {
        match self {
            Value::Int(_) => "an int",
            Value::Float(_) => "a float",
            Value::Bool(_) => "a bool",
            Value::Record(_) => "a record",
        }
    }

    pub fn field(&self, name: &str) -> Option<&Value> {
        match self {
            Value::Record(fields) => fields.iter().find(|(n, _)| n == name).map(|(_, v)| v),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(x) => write!(f, "{x}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Record(fields) => {
                if fields.is_empty() {
                    return write!(f, "{{}}");
                }

                write!(f, "{{ ")?;

                for (i, (name, value)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{name}: {value}")?;
                }

                write!(f, " }}")
            }
        }
    }
}
