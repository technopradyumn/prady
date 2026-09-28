use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::env::Environment;

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    Null,
    Array(Rc<RefCell<Vec<Value>>>),
    Struct {
        name: String,
        fields: Rc<RefCell<HashMap<String, Value>>>,
    },
    Variant {
        enum_name: Option<String>,
        variant_name: String,
        payload: Vec<Value>,
    },
    Function {
        name: Option<String>,
        params: Vec<String>,
        body: prady_ast::Block,
        closure_env: Option<Rc<RefCell<Environment>>>,
    },
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0 && !f.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Char(c) => *c != '\0',
            Value::Array(arr) => !arr.borrow().is_empty(),
            Value::Struct { .. } => true,
            Value::Variant { .. } => true,
            Value::Function { .. } => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Bool(_) => "Bool",
            Value::Char(_) => "Char",
            Value::Null => "Null",
            Value::Array(_) => "Array",
            Value::Struct { name, .. } => match name.as_str() {
                "Map" => "Map",
                "Set" => "Set",
                "Stack" => "Stack",
                "Queue" => "Queue",
                "Deque" => "Deque",
                "MinHeap" | "PriorityQueue" => "MinHeap",
                "MaxHeap" => "MaxHeap",
                "LinkedList" => "LinkedList",
                "DoublyLinkedList" => "DoublyLinkedList",
                "BST" | "BinarySearchTree" => "BinarySearchTree",
                "AVLTree" => "AVLTree",
                "RedBlackTree" => "RedBlackTree",
                "Trie" => "Trie",
                "Graph" => "Graph",
                "LRUCache" => "LRUCache",
                "LFUCache" => "LFUCache",
                "CircularBuffer" => "CircularBuffer",
                "BloomFilter" => "BloomFilter",
                "DisjointSet" | "UnionFind" => "DisjointSet",
                "SegmentTree" => "SegmentTree",
                "FenwickTree" => "FenwickTree",
                "BitSet" => "BitSet",
                "SkipList" => "SkipList",
                "Matrix" => "Matrix",
                "SparseMatrix" => "SparseMatrix",
                "TreeMap" => "TreeMap",
                "TreeSet" => "TreeSet",
                _ => "Struct",
            },
            Value::Variant { .. } => "Variant",
            Value::Function { .. } => "Function",
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow(),
            (
                Value::Struct {
                    name: n1,
                    fields: f1,
                },
                Value::Struct {
                    name: n2,
                    fields: f2,
                },
            ) => n1 == n2 && (Rc::ptr_eq(f1, f2) || *f1.borrow() == *f2.borrow()),
            (
                Value::Variant {
                    enum_name: e1,
                    variant_name: v1,
                    payload: p1,
                },
                Value::Variant {
                    enum_name: e2,
                    variant_name: v2,
                    payload: p2,
                },
            ) => e1 == e2 && v1 == v2 && p1 == p2,
            (
                Value::Function {
                    name: n1,
                    params: p1,
                    ..
                },
                Value::Function {
                    name: n2,
                    params: p2,
                    ..
                },
            ) => n1 == n2 && p1 == p2,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Char(c) => write!(f, "{}", c),
            Value::Null => write!(f, "null"),
            Value::Array(items) => {
                let borrowed = items.borrow();
                write!(f, "[")?;
                for (i, val) in borrowed.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if let Value::String(s) = val {
                        write!(f, "\"{}\"", s)?;
                    } else {
                        write!(f, "{}", val)?;
                    }
                }
                write!(f, "]")
            }
            Value::Struct { name, fields } => {
                let borrowed = fields.borrow();
                write!(f, "{} {{ ", name)?;
                let mut first = true;
                for (k, v) in borrowed.iter() {
                    if k.starts_with('_') {
                        continue;
                    }
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, " }}")
            }
            Value::Variant {
                variant_name,
                payload,
                ..
            } => {
                if payload.is_empty() {
                    write!(f, "{}", variant_name)
                } else {
                    write!(f, "{}(", variant_name)?;
                    for (i, p) in payload.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", p)?;
                    }
                    write!(f, ")")
                }
            }
            Value::Function { name, params, .. } => {
                if let Some(n) = name {
                    write!(f, "<fn {}>", n)
                } else {
                    write!(f, "<fn ({})>", params.join(", "))
                }
            }
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            _ => write!(f, "{}", self),
        }
    }
}
