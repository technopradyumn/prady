use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::Value;

pub fn create_dsa_instance(name: &str, args: Vec<Value>) -> Option<Value> {
    let mut fields = HashMap::new();
    match name {
        "Map" | "HashMap" => {
            fields.insert("_keys".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("_values".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "Map".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "Set" | "HashSet" => {
            let initial = if let Some(Value::Array(arr)) = args.first() {
                arr.borrow().clone()
            } else {
                Vec::new()
            };
            let mut unique = Vec::new();
            for item in initial {
                if !unique.contains(&item) {
                    unique.push(item);
                }
            }
            let count = unique.len() as i64;
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(unique))));
            fields.insert("size".to_string(), Value::Int(count));
            Some(Value::Struct {
                name: "Set".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "Stack" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "Stack".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "Queue" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "Queue".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "Deque" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "Deque".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "MinHeap" | "PriorityQueue" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "MinHeap".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "MaxHeap" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "MaxHeap".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "LinkedList" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "LinkedList".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "DoublyLinkedList" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "DoublyLinkedList".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "BST" | "BinarySearchTree" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "BinarySearchTree".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "AVLTree" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "AVLTree".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "RedBlackTree" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "RedBlackTree".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "Trie" => {
            fields.insert("_words".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "Trie".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "Graph" => {
            fields.insert("_vertices".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("_edges".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            Some(Value::Struct {
                name: "Graph".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "LRUCache" => {
            let cap = match args.first() {
                Some(Value::Int(n)) => *n,
                _ => 10,
            };
            fields.insert("_capacity".to_string(), Value::Int(cap));
            fields.insert("_keys".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("_values".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "LRUCache".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "LFUCache" => {
            let cap = match args.first() {
                Some(Value::Int(n)) => *n,
                _ => 10,
            };
            fields.insert("_capacity".to_string(), Value::Int(cap));
            fields.insert("_keys".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("_values".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("_counts".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "LFUCache".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "CircularBuffer" => {
            let cap = match args.first() {
                Some(Value::Int(n)) => *n,
                _ => 8,
            };
            fields.insert("_capacity".to_string(), Value::Int(cap));
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            Some(Value::Struct {
                name: "CircularBuffer".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "BloomFilter" => {
            let size = match args.first() {
                Some(Value::Int(n)) => *n,
                _ => 64,
            };
            fields.insert("_size".to_string(), Value::Int(size));
            fields.insert("_bits".to_string(), Value::Array(Rc::new(RefCell::new(vec![Value::Bool(false); size as usize]))));
            Some(Value::Struct {
                name: "BloomFilter".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "DisjointSet" | "UnionFind" => {
            let size = match args.first() {
                Some(Value::Int(n)) => *n as usize,
                _ => 16,
            };
            let parent: Vec<Value> = (0..size).map(|i| Value::Int(i as i64)).collect();
            let rank: Vec<Value> = vec![Value::Int(0); size];
            fields.insert("_parent".to_string(), Value::Array(Rc::new(RefCell::new(parent))));
            fields.insert("_rank".to_string(), Value::Array(Rc::new(RefCell::new(rank))));
            fields.insert("count".to_string(), Value::Int(size as i64));
            Some(Value::Struct {
                name: "DisjointSet".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "SegmentTree" => {
            let leaf_vals = match args.first() {
                Some(Value::Array(arr)) => arr.borrow().clone(),
                _ => Vec::new(),
            };
            let mut tree = Vec::new();
            if !leaf_vals.is_empty() {
                let n = leaf_vals.len();
                tree.resize(2 * n, Value::Int(0));
                for (i, val) in leaf_vals.iter().enumerate() {
                    tree[n + i] = val.clone();
                }
                for i in (1..n).rev() {
                    let left = match &tree[2 * i] { Value::Int(x) => *x, _ => 0 };
                    let right = match &tree[2 * i + 1] { Value::Int(x) => *x, _ => 0 };
                    tree[i] = Value::Int(left + right);
                }
            }
            let n_leaf = leaf_vals.len() as i64;
            fields.insert("_n".to_string(), Value::Int(n_leaf));
            fields.insert("_tree".to_string(), Value::Array(Rc::new(RefCell::new(tree))));
            Some(Value::Struct {
                name: "SegmentTree".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "FenwickTree" | "BinaryIndexedTree" => {
            let size = match args.first() {
                Some(Value::Int(n)) => *n as usize,
                _ => 16,
            };
            let tree = vec![Value::Int(0); size + 1];
            fields.insert("_size".to_string(), Value::Int(size as i64));
            fields.insert("_tree".to_string(), Value::Array(Rc::new(RefCell::new(tree))));
            Some(Value::Struct {
                name: "FenwickTree".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "BitSet" => {
            let size = match args.first() {
                Some(Value::Int(n)) => *n as usize,
                _ => 64,
            };
            let bits = vec![Value::Bool(false); size];
            fields.insert("_size".to_string(), Value::Int(size as i64));
            fields.insert("_bits".to_string(), Value::Array(Rc::new(RefCell::new(bits))));
            Some(Value::Struct {
                name: "BitSet".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "SkipList" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "SkipList".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "Matrix" => {
            let rows = match args.first() { Some(Value::Int(r)) => *r as usize, _ => 3 };
            let cols = match args.get(1) { Some(Value::Int(c)) => *c as usize, _ => 3 };
            let def = args.get(2).cloned().unwrap_or(Value::Int(0));
            let data: Vec<Value> = (0..rows)
                .map(|_| Value::Array(Rc::new(RefCell::new(vec![def.clone(); cols]))))
                .collect();
            fields.insert("_rows".to_string(), Value::Int(rows as i64));
            fields.insert("_cols".to_string(), Value::Int(cols as i64));
            fields.insert("_data".to_string(), Value::Array(Rc::new(RefCell::new(data))));
            Some(Value::Struct {
                name: "Matrix".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "SparseMatrix" => {
            let rows = match args.first() { Some(Value::Int(r)) => *r, _ => 10 };
            let cols = match args.get(1) { Some(Value::Int(c)) => *c, _ => 10 };
            fields.insert("_rows".to_string(), Value::Int(rows));
            fields.insert("_cols".to_string(), Value::Int(cols));
            fields.insert("_entries".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            Some(Value::Struct {
                name: "SparseMatrix".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "TreeMap" => {
            fields.insert("_keys".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("_values".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "TreeMap".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        "TreeSet" => {
            fields.insert("_items".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            fields.insert("size".to_string(), Value::Int(0));
            Some(Value::Struct {
                name: "TreeSet".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            })
        }
        _ => None,
    }
}

pub fn dsa_to_array(struct_name: &str, fields: &Rc<RefCell<HashMap<String, Value>>>) -> Option<Vec<Value>> {
    let borrowed = fields.borrow();
    match struct_name {
        "Set" | "HashSet" | "Stack" | "Queue" | "Deque" | "MinHeap" | "MaxHeap" | "LinkedList"
        | "DoublyLinkedList" | "BinarySearchTree" | "AVLTree" | "RedBlackTree" | "TreeSet"
        | "CircularBuffer" | "SkipList" => {
            if let Some(Value::Array(arr)) = borrowed.get("_items") {
                return Some(arr.borrow().clone());
            }
            None
        }
        "Map" | "HashMap" | "TreeMap" => {
            if let (Some(Value::Array(keys)), Some(Value::Array(values))) =
                (borrowed.get("_keys"), borrowed.get("_values"))
            {
                let k_b = keys.borrow();
                let v_b = values.borrow();
                let mut entries = Vec::new();
                for i in 0..k_b.len() {
                    let val = v_b.get(i).cloned().unwrap_or(Value::Null);
                    entries.push(Value::Array(Rc::new(RefCell::new(vec![k_b[i].clone(), val]))));
                }
                return Some(entries);
            }
            None
        }
        "Trie" => {
            if let Some(Value::Array(words)) = borrowed.get("_words") {
                return Some(words.borrow().clone());
            }
            None
        }
        "Graph" => {
            if let Some(Value::Array(vertices)) = borrowed.get("_vertices") {
                return Some(vertices.borrow().clone());
            }
            None
        }
        _ => None,
    }
}

pub fn handle_dsa_method(
    struct_name: &str,
    fields: &Rc<RefCell<HashMap<String, Value>>>,
    method: &str,
    args: Vec<Value>,
) -> Result<Option<Value>, String> {
    match struct_name {
        "Map" | "HashMap" | "TreeMap" => {
            let mut f = fields.borrow_mut();
            let keys_arr = match f.get("_keys") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };
            let vals_arr = match f.get("_values") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "set" => {
                    let key = args.first().cloned().unwrap_or(Value::Null);
                    let val = args.get(1).cloned().unwrap_or(Value::Null);
                    let mut k_borrow = keys_arr.borrow_mut();
                    let mut v_borrow = vals_arr.borrow_mut();
                    if let Some(idx) = k_borrow.iter().position(|x| x == &key) {
                        v_borrow[idx] = val;
                    } else {
                        k_borrow.push(key);
                        v_borrow.push(val);
                        f.insert("size".to_string(), Value::Int(k_borrow.len() as i64));
                    }
                    Ok(Some(Value::Null))
                }
                "get" => {
                    let key = args.first().cloned().unwrap_or(Value::Null);
                    let k_borrow = keys_arr.borrow();
                    let v_borrow = vals_arr.borrow();
                    if let Some(idx) = k_borrow.iter().position(|x| x == &key) {
                        Ok(Some(v_borrow[idx].clone()))
                    } else {
                        Ok(Some(Value::Null))
                    }
                }
                "has" => {
                    let key = args.first().cloned().unwrap_or(Value::Null);
                    let k_borrow = keys_arr.borrow();
                    Ok(Some(Value::Bool(k_borrow.iter().any(|x| x == &key))))
                }
                "delete" => {
                    let key = args.first().cloned().unwrap_or(Value::Null);
                    let mut k_borrow = keys_arr.borrow_mut();
                    let mut v_borrow = vals_arr.borrow_mut();
                    if let Some(idx) = k_borrow.iter().position(|x| x == &key) {
                        k_borrow.remove(idx);
                        v_borrow.remove(idx);
                        f.insert("size".to_string(), Value::Int(k_borrow.len() as i64));
                        Ok(Some(Value::Bool(true)))
                    } else {
                        Ok(Some(Value::Bool(false)))
                    }
                }
                "clear" => {
                    keys_arr.borrow_mut().clear();
                    vals_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                "size" => Ok(Some(Value::Int(keys_arr.borrow().len() as i64))),
                "keys" => Ok(Some(Value::Array(Rc::new(RefCell::new(keys_arr.borrow().clone()))))),
                "values" => Ok(Some(Value::Array(Rc::new(RefCell::new(vals_arr.borrow().clone()))))),
                "entries" => {
                    let k_b = keys_arr.borrow();
                    let v_b = vals_arr.borrow();
                    let mut entries = Vec::new();
                    for i in 0..k_b.len() {
                        entries.push(Value::Array(Rc::new(RefCell::new(vec![k_b[i].clone(), v_b[i].clone()]))));
                    }
                    Ok(Some(Value::Array(Rc::new(RefCell::new(entries)))))
                }
                _ => Ok(None),
            }
        }

        "Set" | "HashSet" | "TreeSet" => {
            let mut f = fields.borrow_mut();
            let items_arr = match f.get("_items") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "add" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut borrow = items_arr.borrow_mut();
                    if !borrow.contains(&val) {
                        borrow.push(val);
                        f.insert("size".to_string(), Value::Int(borrow.len() as i64));
                    }
                    Ok(Some(Value::Null))
                }
                "has" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let borrow = items_arr.borrow();
                    Ok(Some(Value::Bool(borrow.contains(&val))))
                }
                "delete" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut borrow = items_arr.borrow_mut();
                    if let Some(pos) = borrow.iter().position(|x| x == &val) {
                        borrow.remove(pos);
                        f.insert("size".to_string(), Value::Int(borrow.len() as i64));
                        Ok(Some(Value::Bool(true)))
                    } else {
                        Ok(Some(Value::Bool(false)))
                    }
                }
                "clear" => {
                    items_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "values" | "toArray" => {
                    Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone())))))
                }
                "union" => {
                    let other_items = match args.first() {
                        Some(Value::Struct { fields: other_f, .. }) => {
                            if let Some(Value::Array(a)) = other_f.borrow().get("_items") {
                                a.borrow().clone()
                            } else {
                                Vec::new()
                            }
                        }
                        Some(Value::Array(a)) => a.borrow().clone(),
                        _ => Vec::new(),
                    };
                    let mut combined = items_arr.borrow().clone();
                    for item in other_items {
                        if !combined.contains(&item) {
                            combined.push(item);
                        }
                    }
                    let res_fields = Rc::new(RefCell::new(HashMap::from([
                        ("_items".to_string(), Value::Array(Rc::new(RefCell::new(combined.clone())))),
                        ("size".to_string(), Value::Int(combined.len() as i64)),
                    ])));
                    Ok(Some(Value::Struct { name: "Set".to_string(), fields: res_fields }))
                }
                "intersection" => {
                    let other_items = match args.first() {
                        Some(Value::Struct { fields: other_f, .. }) => {
                            if let Some(Value::Array(a)) = other_f.borrow().get("_items") {
                                a.borrow().clone()
                            } else {
                                Vec::new()
                            }
                        }
                        Some(Value::Array(a)) => a.borrow().clone(),
                        _ => Vec::new(),
                    };
                    let current = items_arr.borrow();
                    let common: Vec<Value> = current.iter().filter(|x| other_items.contains(x)).cloned().collect();
                    let res_fields = Rc::new(RefCell::new(HashMap::from([
                        ("_items".to_string(), Value::Array(Rc::new(RefCell::new(common.clone())))),
                        ("size".to_string(), Value::Int(common.len() as i64)),
                    ])));
                    Ok(Some(Value::Struct { name: "Set".to_string(), fields: res_fields }))
                }
                "difference" => {
                    let other_items = match args.first() {
                        Some(Value::Struct { fields: other_f, .. }) => {
                            if let Some(Value::Array(a)) = other_f.borrow().get("_items") {
                                a.borrow().clone()
                            } else {
                                Vec::new()
                            }
                        }
                        Some(Value::Array(a)) => a.borrow().clone(),
                        _ => Vec::new(),
                    };
                    let current = items_arr.borrow();
                    let diff: Vec<Value> = current.iter().filter(|x| !other_items.contains(x)).cloned().collect();
                    let res_fields = Rc::new(RefCell::new(HashMap::from([
                        ("_items".to_string(), Value::Array(Rc::new(RefCell::new(diff.clone())))),
                        ("size".to_string(), Value::Int(diff.len() as i64)),
                    ])));
                    Ok(Some(Value::Struct { name: "Set".to_string(), fields: res_fields }))
                }
                _ => Ok(None),
            }
        }

        "Stack" => {
            let mut f = fields.borrow_mut();
            let items_arr = match f.get("_items") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "push" => {
                    for arg in args {
                        items_arr.borrow_mut().push(arg);
                    }
                    f.insert("size".to_string(), Value::Int(items_arr.borrow().len() as i64));
                    Ok(Some(Value::Null))
                }
                "pop" => {
                    let val = items_arr.borrow_mut().pop().unwrap_or(Value::Null);
                    f.insert("size".to_string(), Value::Int(items_arr.borrow().len() as i64));
                    Ok(Some(val))
                }
                "peek" => Ok(Some(items_arr.borrow().last().cloned().unwrap_or(Value::Null))),
                "isEmpty" => Ok(Some(Value::Bool(items_arr.borrow().is_empty()))),
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "clear" => {
                    items_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                "toArray" => Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone()))))),
                _ => Ok(None),
            }
        }

        "Queue" => {
            let mut f = fields.borrow_mut();
            let items_arr = match f.get("_items") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "enqueue" | "push" => {
                    for arg in args {
                        items_arr.borrow_mut().push(arg);
                    }
                    f.insert("size".to_string(), Value::Int(items_arr.borrow().len() as i64));
                    Ok(Some(Value::Null))
                }
                "dequeue" | "pop" => {
                    let mut b = items_arr.borrow_mut();
                    let val = if b.is_empty() { Value::Null } else { b.remove(0) };
                    f.insert("size".to_string(), Value::Int(b.len() as i64));
                    Ok(Some(val))
                }
                "peek" => Ok(Some(items_arr.borrow().first().cloned().unwrap_or(Value::Null))),
                "isEmpty" => Ok(Some(Value::Bool(items_arr.borrow().is_empty()))),
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "clear" => {
                    items_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                "toArray" => Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone()))))),
                _ => Ok(None),
            }
        }

        "Deque" => {
            let mut f = fields.borrow_mut();
            let items_arr = match f.get("_items") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "pushFront" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    items_arr.borrow_mut().insert(0, val);
                    f.insert("size".to_string(), Value::Int(items_arr.borrow().len() as i64));
                    Ok(Some(Value::Null))
                }
                "pushBack" | "push" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    items_arr.borrow_mut().push(val);
                    f.insert("size".to_string(), Value::Int(items_arr.borrow().len() as i64));
                    Ok(Some(Value::Null))
                }
                "popFront" => {
                    let mut b = items_arr.borrow_mut();
                    let val = if b.is_empty() { Value::Null } else { b.remove(0) };
                    f.insert("size".to_string(), Value::Int(b.len() as i64));
                    Ok(Some(val))
                }
                "popBack" | "pop" => {
                    let val = items_arr.borrow_mut().pop().unwrap_or(Value::Null);
                    f.insert("size".to_string(), Value::Int(items_arr.borrow().len() as i64));
                    Ok(Some(val))
                }
                "peekFront" => Ok(Some(items_arr.borrow().first().cloned().unwrap_or(Value::Null))),
                "peekBack" => Ok(Some(items_arr.borrow().last().cloned().unwrap_or(Value::Null))),
                "isEmpty" => Ok(Some(Value::Bool(items_arr.borrow().is_empty()))),
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "clear" => {
                    items_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                "toArray" => Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone()))))),
                _ => Ok(None),
            }
        }

        "MinHeap" | "PriorityQueue" => {
            let mut f = fields.borrow_mut();
            let items_arr = match f.get("_items") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "insert" | "push" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut b = items_arr.borrow_mut();
                    b.push(val);
                    let mut idx = b.len() - 1;
                    while idx > 0 {
                        let parent = (idx - 1) / 2;
                        if val_less(&b[idx], &b[parent]) {
                            b.swap(idx, parent);
                            idx = parent;
                        } else {
                            break;
                        }
                    }
                    f.insert("size".to_string(), Value::Int(b.len() as i64));
                    Ok(Some(Value::Null))
                }
                "extractMin" | "pop" => {
                    let mut b = items_arr.borrow_mut();
                    if b.is_empty() {
                        return Ok(Some(Value::Null));
                    }
                    let min_val = b.swap_remove(0);
                    if !b.is_empty() {
                        let mut idx = 0;
                        loop {
                            let left = 2 * idx + 1;
                            let right = 2 * idx + 2;
                            let mut smallest = idx;
                            if left < b.len() && val_less(&b[left], &b[smallest]) {
                                smallest = left;
                            }
                            if right < b.len() && val_less(&b[right], &b[smallest]) {
                                smallest = right;
                            }
                            if smallest != idx {
                                b.swap(idx, smallest);
                                idx = smallest;
                            } else {
                                break;
                            }
                        }
                    }
                    f.insert("size".to_string(), Value::Int(b.len() as i64));
                    Ok(Some(min_val))
                }
                "peek" => Ok(Some(items_arr.borrow().first().cloned().unwrap_or(Value::Null))),
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "isEmpty" => Ok(Some(Value::Bool(items_arr.borrow().is_empty()))),
                "clear" => {
                    items_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                "toArray" => Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone()))))),
                _ => Ok(None),
            }
        }

        "MaxHeap" => {
            let mut f = fields.borrow_mut();
            let items_arr = match f.get("_items") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "insert" | "push" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut b = items_arr.borrow_mut();
                    b.push(val);
                    let mut idx = b.len() - 1;
                    while idx > 0 {
                        let parent = (idx - 1) / 2;
                        if val_greater(&b[idx], &b[parent]) {
                            b.swap(idx, parent);
                            idx = parent;
                        } else {
                            break;
                        }
                    }
                    f.insert("size".to_string(), Value::Int(b.len() as i64));
                    Ok(Some(Value::Null))
                }
                "extractMax" | "pop" => {
                    let mut b = items_arr.borrow_mut();
                    if b.is_empty() {
                        return Ok(Some(Value::Null));
                    }
                    let max_val = b.swap_remove(0);
                    if !b.is_empty() {
                        let mut idx = 0;
                        loop {
                            let left = 2 * idx + 1;
                            let right = 2 * idx + 2;
                            let mut largest = idx;
                            if left < b.len() && val_greater(&b[left], &b[largest]) {
                                largest = left;
                            }
                            if right < b.len() && val_greater(&b[right], &b[largest]) {
                                largest = right;
                            }
                            if largest != idx {
                                b.swap(idx, largest);
                                idx = largest;
                            } else {
                                break;
                            }
                        }
                    }
                    f.insert("size".to_string(), Value::Int(b.len() as i64));
                    Ok(Some(max_val))
                }
                "peek" => Ok(Some(items_arr.borrow().first().cloned().unwrap_or(Value::Null))),
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "isEmpty" => Ok(Some(Value::Bool(items_arr.borrow().is_empty()))),
                "clear" => {
                    items_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                "toArray" => Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone()))))),
                _ => Ok(None),
            }
        }

        "LinkedList" | "DoublyLinkedList" => {
            let mut f = fields.borrow_mut();
            let items_arr = match f.get("_items") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "append" | "push" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    items_arr.borrow_mut().push(val);
                    f.insert("size".to_string(), Value::Int(items_arr.borrow().len() as i64));
                    Ok(Some(Value::Null))
                }
                "prepend" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    items_arr.borrow_mut().insert(0, val);
                    f.insert("size".to_string(), Value::Int(items_arr.borrow().len() as i64));
                    Ok(Some(Value::Null))
                }
                "insertAt" => {
                    let idx = match args.first() { Some(Value::Int(i)) => *i as usize, _ => 0 };
                    let val = args.get(1).cloned().unwrap_or(Value::Null);
                    let mut b = items_arr.borrow_mut();
                    if idx <= b.len() {
                        b.insert(idx, val);
                        f.insert("size".to_string(), Value::Int(b.len() as i64));
                        Ok(Some(Value::Bool(true)))
                    } else {
                        Ok(Some(Value::Bool(false)))
                    }
                }
                "delete" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut b = items_arr.borrow_mut();
                    if let Some(pos) = b.iter().position(|x| x == &val) {
                        b.remove(pos);
                        f.insert("size".to_string(), Value::Int(b.len() as i64));
                        Ok(Some(Value::Bool(true)))
                    } else {
                        Ok(Some(Value::Bool(false)))
                    }
                }
                "deleteAt" => {
                    let idx = match args.first() { Some(Value::Int(i)) => *i as usize, _ => 0 };
                    let mut b = items_arr.borrow_mut();
                    if idx < b.len() {
                        let removed = b.remove(idx);
                        f.insert("size".to_string(), Value::Int(b.len() as i64));
                        Ok(Some(removed))
                    } else {
                        Ok(Some(Value::Null))
                    }
                }
                "get" => {
                    let idx = match args.first() { Some(Value::Int(i)) => *i as usize, _ => 0 };
                    let b = items_arr.borrow();
                    Ok(Some(b.get(idx).cloned().unwrap_or(Value::Null)))
                }
                "contains" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    Ok(Some(Value::Bool(items_arr.borrow().contains(&val))))
                }
                "reverse" => {
                    items_arr.borrow_mut().reverse();
                    Ok(Some(Value::Null))
                }
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "isEmpty" => Ok(Some(Value::Bool(items_arr.borrow().is_empty()))),
                "clear" => {
                    items_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                "toArray" => Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone()))))),
                _ => Ok(None),
            }
        }

        "BinarySearchTree" | "AVLTree" | "RedBlackTree" => {
            let mut f = fields.borrow_mut();
            let items_arr = match f.get("_items") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "insert" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut b = items_arr.borrow_mut();
                    if !b.contains(&val) {
                        b.push(val);
                        b.sort_by(val_cmp);
                        f.insert("size".to_string(), Value::Int(b.len() as i64));
                    }
                    Ok(Some(Value::Null))
                }
                "contains" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    Ok(Some(Value::Bool(items_arr.borrow().contains(&val))))
                }
                "remove" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut b = items_arr.borrow_mut();
                    if let Some(pos) = b.iter().position(|x| x == &val) {
                        b.remove(pos);
                        f.insert("size".to_string(), Value::Int(b.len() as i64));
                        Ok(Some(Value::Bool(true)))
                    } else {
                        Ok(Some(Value::Bool(false)))
                    }
                }
                "min" => Ok(Some(items_arr.borrow().first().cloned().unwrap_or(Value::Null))),
                "max" => Ok(Some(items_arr.borrow().last().cloned().unwrap_or(Value::Null))),
                "inorder" | "toArray" => {
                    Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone())))))
                }
                "preorder" => {
                    Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone())))))
                }
                "postorder" => {
                    let mut rev = items_arr.borrow().clone();
                    rev.reverse();
                    Ok(Some(Value::Array(Rc::new(RefCell::new(rev)))))
                }
                "height" => {
                    let n = items_arr.borrow().len();
                    let h = if n == 0 { 0 } else { (n as f64).log2().floor() as i64 + 1 };
                    Ok(Some(Value::Int(h)))
                }
                "isBalanced" => Ok(Some(Value::Bool(true))),
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "clear" => {
                    items_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                _ => Ok(None),
            }
        }

        "Trie" => {
            let mut f = fields.borrow_mut();
            let words_arr = match f.get("_words") {
                Some(Value::Array(a)) => Rc::clone(a),
                _ => return Ok(None),
            };

            match method {
                "insert" => {
                    let word = match args.first() {
                        Some(Value::String(s)) => s.clone(),
                        Some(other) => other.to_string(),
                        None => String::new(),
                    };
                    let mut b = words_arr.borrow_mut();
                    if !b.iter().any(|w| match w { Value::String(s) => s == &word, _ => false }) {
                        b.push(Value::String(word));
                        f.insert("size".to_string(), Value::Int(b.len() as i64));
                    }
                    Ok(Some(Value::Null))
                }
                "search" => {
                    let word = match args.first() {
                        Some(Value::String(s)) => s.clone(),
                        Some(other) => other.to_string(),
                        None => String::new(),
                    };
                    let b = words_arr.borrow();
                    let found = b.iter().any(|w| match w { Value::String(s) => s == &word, _ => false });
                    Ok(Some(Value::Bool(found)))
                }
                "startsWith" => {
                    let prefix = match args.first() {
                        Some(Value::String(s)) => s.clone(),
                        Some(other) => other.to_string(),
                        None => String::new(),
                    };
                    let b = words_arr.borrow();
                    let found = b.iter().any(|w| match w { Value::String(s) => s.starts_with(&prefix), _ => false });
                    Ok(Some(Value::Bool(found)))
                }
                "wordsWithPrefix" => {
                    let prefix = match args.first() {
                        Some(Value::String(s)) => s.clone(),
                        Some(other) => other.to_string(),
                        None => String::new(),
                    };
                    let b = words_arr.borrow();
                    let matches: Vec<Value> = b.iter().filter(|w| match w { Value::String(s) => s.starts_with(&prefix), _ => false }).cloned().collect();
                    Ok(Some(Value::Array(Rc::new(RefCell::new(matches)))))
                }
                "delete" => {
                    let word = match args.first() {
                        Some(Value::String(s)) => s.clone(),
                        Some(other) => other.to_string(),
                        None => String::new(),
                    };
                    let mut b = words_arr.borrow_mut();
                    if let Some(pos) = b.iter().position(|w| match w { Value::String(s) => s == &word, _ => false }) {
                        b.remove(pos);
                        f.insert("size".to_string(), Value::Int(b.len() as i64));
                        Ok(Some(Value::Bool(true)))
                    } else {
                        Ok(Some(Value::Bool(false)))
                    }
                }
                "size" => Ok(Some(Value::Int(words_arr.borrow().len() as i64))),
                "clear" => {
                    words_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                _ => Ok(None),
            }
        }

        "Graph" => {
            let f = fields.borrow();
            let v_arr = match f.get("_vertices") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };
            let e_arr = match f.get("_edges") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "addVertex" => {
                    let v = args.first().cloned().unwrap_or(Value::Null);
                    let mut vb = v_arr.borrow_mut();
                    if !vb.contains(&v) {
                        vb.push(v);
                    }
                    Ok(Some(Value::Null))
                }
                "addEdge" => {
                    let u = args.first().cloned().unwrap_or(Value::Null);
                    let v = args.get(1).cloned().unwrap_or(Value::Null);
                    let weight = args.get(2).cloned().unwrap_or(Value::Int(1));
                    {
                        let mut vb = v_arr.borrow_mut();
                        if !vb.contains(&u) { vb.push(u.clone()); }
                        if !vb.contains(&v) { vb.push(v.clone()); }
                    }
                    let edge = Value::Array(Rc::new(RefCell::new(vec![u, v, weight])));
                    e_arr.borrow_mut().push(edge);
                    Ok(Some(Value::Null))
                }
                "hasVertex" => {
                    let v = args.first().cloned().unwrap_or(Value::Null);
                    Ok(Some(Value::Bool(v_arr.borrow().contains(&v))))
                }
                "hasEdge" => {
                    let u = args.first().cloned().unwrap_or(Value::Null);
                    let v = args.get(1).cloned().unwrap_or(Value::Null);
                    let eb = e_arr.borrow();
                    let has = eb.iter().any(|edge| {
                        if let Value::Array(a) = edge {
                            let b = a.borrow();
                            b.len() >= 2 && b[0] == u && b[1] == v
                        } else {
                            false
                        }
                    });
                    Ok(Some(Value::Bool(has)))
                }
                "getVertices" => Ok(Some(Value::Array(Rc::new(RefCell::new(v_arr.borrow().clone()))))),
                "getNeighbors" => {
                    let u = args.first().cloned().unwrap_or(Value::Null);
                    let eb = e_arr.borrow();
                    let mut neighbors = Vec::new();
                    for edge in eb.iter() {
                        if let Value::Array(a) = edge {
                            let b = a.borrow();
                            if b.len() >= 2 && b[0] == u {
                                neighbors.push(b[1].clone());
                            }
                        }
                    }
                    Ok(Some(Value::Array(Rc::new(RefCell::new(neighbors)))))
                }
                "bfs" => {
                    let start = args.first().cloned().unwrap_or(Value::Null);
                    let mut visited = Vec::new();
                    let mut queue = vec![start.clone()];
                    visited.push(start);
                    let eb = e_arr.borrow();
                    while !queue.is_empty() {
                        let curr = queue.remove(0);
                        for edge in eb.iter() {
                            if let Value::Array(a) = edge {
                                let b = a.borrow();
                                if b.len() >= 2 && b[0] == curr && !visited.contains(&b[1]) {
                                    visited.push(b[1].clone());
                                    queue.push(b[1].clone());
                                }
                            }
                        }
                    }
                    Ok(Some(Value::Array(Rc::new(RefCell::new(visited)))))
                }
                "dfs" => {
                    let start = args.first().cloned().unwrap_or(Value::Null);
                    let mut visited = Vec::new();
                    let mut stack = vec![start];
                    let eb = e_arr.borrow();
                    while let Some(curr) = stack.pop() {
                        if !visited.contains(&curr) {
                            visited.push(curr.clone());
                            for edge in eb.iter().rev() {
                                if let Value::Array(a) = edge {
                                    let b = a.borrow();
                                    if b.len() >= 2 && b[0] == curr && !visited.contains(&b[1]) {
                                        stack.push(b[1].clone());
                                    }
                                }
                            }
                        }
                    }
                    Ok(Some(Value::Array(Rc::new(RefCell::new(visited)))))
                }
                "topologicalSort" => {
                    Ok(Some(Value::Array(Rc::new(RefCell::new(v_arr.borrow().clone())))))
                }
                "hasCycle" => Ok(Some(Value::Bool(false))),
                _ => Ok(None),
            }
        }

        "LRUCache" => {
            let mut f = fields.borrow_mut();
            let cap = match f.get("_capacity") { Some(Value::Int(c)) => *c as usize, _ => 10 };
            let keys_arr = match f.get("_keys") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };
            let vals_arr = match f.get("_values") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "put" => {
                    let key = args.first().cloned().unwrap_or(Value::Null);
                    let val = args.get(1).cloned().unwrap_or(Value::Null);
                    let mut kb = keys_arr.borrow_mut();
                    let mut vb = vals_arr.borrow_mut();
                    if let Some(pos) = kb.iter().position(|k| k == &key) {
                        kb.remove(pos);
                        vb.remove(pos);
                    } else if kb.len() >= cap {
                        kb.remove(0);
                        vb.remove(0);
                    }
                    kb.push(key);
                    vb.push(val);
                    f.insert("size".to_string(), Value::Int(kb.len() as i64));
                    Ok(Some(Value::Null))
                }
                "get" => {
                    let key = args.first().cloned().unwrap_or(Value::Null);
                    let mut kb = keys_arr.borrow_mut();
                    let mut vb = vals_arr.borrow_mut();
                    if let Some(pos) = kb.iter().position(|k| k == &key) {
                        let k = kb.remove(pos);
                        let v = vb.remove(pos);
                        kb.push(k);
                        vb.push(v.clone());
                        Ok(Some(v))
                    } else {
                        Ok(Some(Value::Null))
                    }
                }
                "has" => {
                    let key = args.first().cloned().unwrap_or(Value::Null);
                    Ok(Some(Value::Bool(keys_arr.borrow().contains(&key))))
                }
                "size" => Ok(Some(Value::Int(keys_arr.borrow().len() as i64))),
                "capacity" => Ok(Some(Value::Int(cap as i64))),
                "clear" => {
                    keys_arr.borrow_mut().clear();
                    vals_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                _ => Ok(None),
            }
        }

        "LFUCache" => {
            let mut f = fields.borrow_mut();
            let cap = match f.get("_capacity") { Some(Value::Int(c)) => *c as usize, _ => 10 };
            let keys_arr = match f.get("_keys") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };
            let vals_arr = match f.get("_values") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };
            let counts_arr = match f.get("_counts") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "put" => {
                    let key = args.first().cloned().unwrap_or(Value::Null);
                    let val = args.get(1).cloned().unwrap_or(Value::Null);
                    let mut kb = keys_arr.borrow_mut();
                    let mut vb = vals_arr.borrow_mut();
                    let mut cb = counts_arr.borrow_mut();
                    if let Some(pos) = kb.iter().position(|k| k == &key) {
                        vb[pos] = val;
                        if let Value::Int(cnt) = cb[pos] { cb[pos] = Value::Int(cnt + 1); }
                    } else {
                        if kb.len() >= cap && !kb.is_empty() {
                            let mut min_idx = 0;
                            let mut min_count = i64::MAX;
                            for (i, c) in cb.iter().enumerate() {
                                if let Value::Int(n) = c {
                                    if *n < min_count { min_count = *n; min_idx = i; }
                                }
                            }
                            kb.remove(min_idx);
                            vb.remove(min_idx);
                            cb.remove(min_idx);
                        }
                        kb.push(key);
                        vb.push(val);
                        cb.push(Value::Int(1));
                    }
                    f.insert("size".to_string(), Value::Int(kb.len() as i64));
                    Ok(Some(Value::Null))
                }
                "get" => {
                    let key = args.first().cloned().unwrap_or(Value::Null);
                    let kb = keys_arr.borrow();
                    let vb = vals_arr.borrow();
                    let mut cb = counts_arr.borrow_mut();
                    if let Some(pos) = kb.iter().position(|k| k == &key) {
                        if let Value::Int(cnt) = cb[pos] { cb[pos] = Value::Int(cnt + 1); }
                        Ok(Some(vb[pos].clone()))
                    } else {
                        Ok(Some(Value::Null))
                    }
                }
                "size" => Ok(Some(Value::Int(keys_arr.borrow().len() as i64))),
                "clear" => {
                    keys_arr.borrow_mut().clear();
                    vals_arr.borrow_mut().clear();
                    counts_arr.borrow_mut().clear();
                    f.insert("size".to_string(), Value::Int(0));
                    Ok(Some(Value::Null))
                }
                _ => Ok(None),
            }
        }

        "CircularBuffer" => {
            let f = fields.borrow();
            let cap = match f.get("_capacity") { Some(Value::Int(c)) => *c as usize, _ => 8 };
            let items_arr = match f.get("_items") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "push" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut b = items_arr.borrow_mut();
                    if b.len() >= cap {
                        b.remove(0);
                    }
                    b.push(val);
                    Ok(Some(Value::Null))
                }
                "pop" => {
                    let mut b = items_arr.borrow_mut();
                    Ok(Some(if b.is_empty() { Value::Null } else { b.remove(0) }))
                }
                "peek" => Ok(Some(items_arr.borrow().first().cloned().unwrap_or(Value::Null))),
                "isFull" => Ok(Some(Value::Bool(items_arr.borrow().len() >= cap))),
                "isEmpty" => Ok(Some(Value::Bool(items_arr.borrow().is_empty()))),
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "capacity" => Ok(Some(Value::Int(cap as i64))),
                "clear" => {
                    items_arr.borrow_mut().clear();
                    Ok(Some(Value::Null))
                }
                "toArray" => Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone()))))),
                _ => Ok(None),
            }
        }

        "BloomFilter" => {
            let f = fields.borrow();
            let size = match f.get("_size") { Some(Value::Int(s)) => *s as usize, _ => 64 };
            let bits_arr = match f.get("_bits") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "add" => {
                    let item_str = args.first().map(|v| v.to_display_string()).unwrap_or_default();
                    let hash1 = simple_hash(&item_str, 17) % size;
                    let hash2 = simple_hash(&item_str, 31) % size;
                    let mut b = bits_arr.borrow_mut();
                    b[hash1] = Value::Bool(true);
                    b[hash2] = Value::Bool(true);
                    Ok(Some(Value::Null))
                }
                "mightContain" => {
                    let item_str = args.first().map(|v| v.to_display_string()).unwrap_or_default();
                    let hash1 = simple_hash(&item_str, 17) % size;
                    let hash2 = simple_hash(&item_str, 31) % size;
                    let b = bits_arr.borrow();
                    let res = b[hash1] == Value::Bool(true) && b[hash2] == Value::Bool(true);
                    Ok(Some(Value::Bool(res)))
                }
                "clear" => {
                    let mut b = bits_arr.borrow_mut();
                    for x in b.iter_mut() { *x = Value::Bool(false); }
                    Ok(Some(Value::Null))
                }
                _ => Ok(None),
            }
        }

        "DisjointSet" => {
            let mut f = fields.borrow_mut();
            let parent_arr = match f.get("_parent") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };
            let rank_arr = match f.get("_rank") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "find" => {
                    let i = match args.first() { Some(Value::Int(x)) => *x as usize, _ => 0 };
                    let mut p_b = parent_arr.borrow_mut();
                    let root = dsu_find(&mut p_b, i);
                    Ok(Some(Value::Int(root as i64)))
                }
                "union" => {
                    let i = match args.first() { Some(Value::Int(x)) => *x as usize, _ => 0 };
                    let j = match args.get(1) { Some(Value::Int(x)) => *x as usize, _ => 0 };
                    let mut p_b = parent_arr.borrow_mut();
                    let mut r_b = rank_arr.borrow_mut();
                    let root_i = dsu_find(&mut p_b, i);
                    let root_j = dsu_find(&mut p_b, j);
                    if root_i != root_j {
                        let rank_i = match r_b[root_i] { Value::Int(r) => r, _ => 0 };
                        let rank_j = match r_b[root_j] { Value::Int(r) => r, _ => 0 };
                        if rank_i < rank_j {
                            p_b[root_i] = Value::Int(root_j as i64);
                        } else if rank_i > rank_j {
                            p_b[root_j] = Value::Int(root_i as i64);
                        } else {
                            p_b[root_j] = Value::Int(root_i as i64);
                            r_b[root_i] = Value::Int(rank_i + 1);
                        }
                        let cur_cnt = match f.get("count") {
                            Some(Value::Int(cnt)) => Some(*cnt),
                            _ => None,
                        };
                        if let Some(cnt) = cur_cnt {
                            f.insert("count".to_string(), Value::Int(cnt - 1));
                        }
                        Ok(Some(Value::Bool(true)))
                    } else {
                        Ok(Some(Value::Bool(false)))
                    }
                }
                "connected" => {
                    let i = match args.first() { Some(Value::Int(x)) => *x as usize, _ => 0 };
                    let j = match args.get(1) { Some(Value::Int(x)) => *x as usize, _ => 0 };
                    let mut p_b = parent_arr.borrow_mut();
                    let root_i = dsu_find(&mut p_b, i);
                    let root_j = dsu_find(&mut p_b, j);
                    Ok(Some(Value::Bool(root_i == root_j)))
                }
                "count" => Ok(Some(f.get("count").cloned().unwrap_or(Value::Int(0)))),
                _ => Ok(None),
            }
        }

        "SegmentTree" => {
            let f = fields.borrow();
            let n = match f.get("_n") { Some(Value::Int(x)) => *x as usize, _ => 0 };
            let tree_arr = match f.get("_tree") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "update" => {
                    let idx = match args.first() { Some(Value::Int(i)) => *i as usize, _ => 0 };
                    let val = match args.get(1) { Some(Value::Int(v)) => *v, _ => 0 };
                    let mut tb = tree_arr.borrow_mut();
                    if idx < n {
                        let mut pos = n + idx;
                        tb[pos] = Value::Int(val);
                        while pos > 1 {
                            pos /= 2;
                            let left = match tb[2 * pos] { Value::Int(x) => x, _ => 0 };
                            let right = match tb[2 * pos + 1] { Value::Int(x) => x, _ => 0 };
                            tb[pos] = Value::Int(left + right);
                        }
                    }
                    Ok(Some(Value::Null))
                }
                "query" => {
                    let mut l = match args.first() { Some(Value::Int(i)) => *i as usize + n, _ => n };
                    let mut r = match args.get(1) { Some(Value::Int(i)) => *i as usize + n + 1, _ => 2 * n };
                    let tb = tree_arr.borrow();
                    let mut sum = 0;
                    while l < r {
                        if l % 2 == 1 {
                            sum += match tb[l] { Value::Int(x) => x, _ => 0 };
                            l += 1;
                        }
                        if r % 2 == 1 {
                            r -= 1;
                            sum += match tb[r] { Value::Int(x) => x, _ => 0 };
                        }
                        l /= 2;
                        r /= 2;
                    }
                    Ok(Some(Value::Int(sum)))
                }
                _ => Ok(None),
            }
        }

        "FenwickTree" => {
            let f = fields.borrow();
            let size = match f.get("_size") { Some(Value::Int(s)) => *s as usize, _ => 16 };
            let tree_arr = match f.get("_tree") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "update" => {
                    let mut idx = match args.first() { Some(Value::Int(i)) => *i as usize + 1, _ => 1 };
                    let delta = match args.get(1) { Some(Value::Int(d)) => *d, _ => 0 };
                    let mut tb = tree_arr.borrow_mut();
                    while idx <= size {
                        let curr = match tb[idx] { Value::Int(x) => x, _ => 0 };
                        tb[idx] = Value::Int(curr + delta);
                        idx += idx & (!idx + 1);
                    }
                    Ok(Some(Value::Null))
                }
                "query" => {
                    let mut idx = match args.first() { Some(Value::Int(i)) => *i as usize + 1, _ => 0 };
                    let tb = tree_arr.borrow();
                    let mut sum = 0;
                    while idx > 0 {
                        sum += match tb[idx] { Value::Int(x) => x, _ => 0 };
                        idx -= idx & (!idx + 1);
                    }
                    Ok(Some(Value::Int(sum)))
                }
                "queryRange" => {
                    let l = match args.first() { Some(Value::Int(i)) => *i, _ => 0 };
                    let r = match args.get(1) { Some(Value::Int(i)) => *i, _ => 0 };
                    let q_r = {
                        let mut idx = (r + 1) as usize;
                        let tb = tree_arr.borrow();
                        let mut s = 0;
                        while idx > 0 { s += match tb[idx] { Value::Int(x) => x, _ => 0 }; idx -= idx & (!idx + 1); }
                        s
                    };
                    let q_l = if l > 0 {
                        let mut idx = l as usize;
                        let tb = tree_arr.borrow();
                        let mut s = 0;
                        while idx > 0 { s += match tb[idx] { Value::Int(x) => x, _ => 0 }; idx -= idx & (!idx + 1); }
                        s
                    } else { 0 };
                    Ok(Some(Value::Int(q_r - q_l)))
                }
                _ => Ok(None),
            }
        }

        "BitSet" => {
            let f = fields.borrow();
            let size = match f.get("_size") { Some(Value::Int(s)) => *s as usize, _ => 64 };
            let bits_arr = match f.get("_bits") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "set" => {
                    let idx = match args.first() { Some(Value::Int(i)) => *i as usize, _ => 0 };
                    let mut b = bits_arr.borrow_mut();
                    if idx < size { b[idx] = Value::Bool(true); }
                    Ok(Some(Value::Null))
                }
                "clear" => {
                    if let Some(Value::Int(idx)) = args.first() {
                        let mut b = bits_arr.borrow_mut();
                        if (*idx as usize) < size { b[*idx as usize] = Value::Bool(false); }
                    } else {
                        let mut b = bits_arr.borrow_mut();
                        for x in b.iter_mut() { *x = Value::Bool(false); }
                    }
                    Ok(Some(Value::Null))
                }
                "get" => {
                    let idx = match args.first() { Some(Value::Int(i)) => *i as usize, _ => 0 };
                    let b = bits_arr.borrow();
                    Ok(Some(b.get(idx).cloned().unwrap_or(Value::Bool(false))))
                }
                "toggle" => {
                    let idx = match args.first() { Some(Value::Int(i)) => *i as usize, _ => 0 };
                    let mut b = bits_arr.borrow_mut();
                    if idx < size {
                        let cur = b[idx] == Value::Bool(true);
                        b[idx] = Value::Bool(!cur);
                    }
                    Ok(Some(Value::Null))
                }
                "count" => {
                    let b = bits_arr.borrow();
                    let cnt = b.iter().filter(|x| **x == Value::Bool(true)).count();
                    Ok(Some(Value::Int(cnt as i64)))
                }
                "size" => Ok(Some(Value::Int(size as i64))),
                "all" => Ok(Some(Value::Bool(bits_arr.borrow().iter().all(|x| *x == Value::Bool(true))))),
                "any" => Ok(Some(Value::Bool(bits_arr.borrow().contains(&Value::Bool(true))))),
                "none" => Ok(Some(Value::Bool(bits_arr.borrow().iter().all(|x| *x == Value::Bool(false))))),
                _ => Ok(None),
            }
        }

        "SkipList" => {
            let mut f = fields.borrow_mut();
            let items_arr = match f.get("_items") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "insert" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut b = items_arr.borrow_mut();
                    b.push(val);
                    b.sort_by(val_cmp);
                    f.insert("size".to_string(), Value::Int(b.len() as i64));
                    Ok(Some(Value::Null))
                }
                "search" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    Ok(Some(Value::Bool(items_arr.borrow().contains(&val))))
                }
                "remove" => {
                    let val = args.first().cloned().unwrap_or(Value::Null);
                    let mut b = items_arr.borrow_mut();
                    if let Some(pos) = b.iter().position(|x| x == &val) {
                        b.remove(pos);
                        f.insert("size".to_string(), Value::Int(b.len() as i64));
                        Ok(Some(Value::Bool(true)))
                    } else {
                        Ok(Some(Value::Bool(false)))
                    }
                }
                "size" => Ok(Some(Value::Int(items_arr.borrow().len() as i64))),
                "toArray" => Ok(Some(Value::Array(Rc::new(RefCell::new(items_arr.borrow().clone()))))),
                _ => Ok(None),
            }
        }

        "Matrix" => {
            let f = fields.borrow();
            let rows = match f.get("_rows") { Some(Value::Int(r)) => *r as usize, _ => 0 };
            let cols = match f.get("_cols") { Some(Value::Int(c)) => *c as usize, _ => 0 };
            let data_arr = match f.get("_data") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "get" => {
                    let r = match args.first() { Some(Value::Int(x)) => *x as usize, _ => 0 };
                    let c = match args.get(1) { Some(Value::Int(x)) => *x as usize, _ => 0 };
                    let d = data_arr.borrow();
                    if r < d.len() {
                        if let Value::Array(row_arr) = &d[r] {
                            let row = row_arr.borrow();
                            if c < row.len() { return Ok(Some(row[c].clone())); }
                        }
                    }
                    Ok(Some(Value::Null))
                }
                "set" => {
                    let r = match args.first() { Some(Value::Int(x)) => *x as usize, _ => 0 };
                    let c = match args.get(1) { Some(Value::Int(x)) => *x as usize, _ => 0 };
                    let val = args.get(2).cloned().unwrap_or(Value::Null);
                    let d = data_arr.borrow();
                    if r < d.len() {
                        if let Value::Array(row_arr) = &d[r] {
                            let mut row = row_arr.borrow_mut();
                            if c < row.len() { row[c] = val; }
                        }
                    }
                    Ok(Some(Value::Null))
                }
                "rows" => Ok(Some(Value::Int(rows as i64))),
                "cols" => Ok(Some(Value::Int(cols as i64))),
                "toArray" => Ok(Some(Value::Array(Rc::new(RefCell::new(data_arr.borrow().clone()))))),
                _ => Ok(None),
            }
        }

        "SparseMatrix" => {
            let f = fields.borrow();
            let rows = match f.get("_rows") { Some(Value::Int(r)) => *r, _ => 10 };
            let cols = match f.get("_cols") { Some(Value::Int(c)) => *c, _ => 10 };
            let entries = match f.get("_entries") { Some(Value::Array(a)) => Rc::clone(a), _ => return Ok(None) };

            match method {
                "set" => {
                    let r = args.first().cloned().unwrap_or(Value::Int(0));
                    let c = args.get(1).cloned().unwrap_or(Value::Int(0));
                    let val = args.get(2).cloned().unwrap_or(Value::Int(0));
                    let mut eb = entries.borrow_mut();
                    let pos = eb.iter().position(|e| {
                        if let Value::Array(a) = e { let b = a.borrow(); b.len() >= 2 && b[0] == r && b[1] == c } else { false }
                    });
                    if let Some(idx) = pos {
                        eb[idx] = Value::Array(Rc::new(RefCell::new(vec![r, c, val])));
                    } else {
                        eb.push(Value::Array(Rc::new(RefCell::new(vec![r, c, val]))));
                    }
                    Ok(Some(Value::Null))
                }
                "get" => {
                    let r = args.first().cloned().unwrap_or(Value::Int(0));
                    let c = args.get(1).cloned().unwrap_or(Value::Int(0));
                    let eb = entries.borrow();
                    for e in eb.iter() {
                        if let Value::Array(a) = e {
                            let b = a.borrow();
                            if b.len() >= 3 && b[0] == r && b[1] == c { return Ok(Some(b[2].clone())); }
                        }
                    }
                    Ok(Some(Value::Int(0)))
                }
                "rows" => Ok(Some(Value::Int(rows))),
                "cols" => Ok(Some(Value::Int(cols))),
                "nonZeroCount" => Ok(Some(Value::Int(entries.borrow().len() as i64))),
                _ => Ok(None),
            }
        }

        _ => Ok(None),
    }
}

fn dsu_find(parent: &mut [Value], mut i: usize) -> usize {
    let mut root = i;
    while let Some(Value::Int(p)) = parent.get(root) {
        if *p as usize == root {
            break;
        }
        root = *p as usize;
    }
    // Path compression
    while let Some(Value::Int(p)) = parent.get(i) {
        if *p as usize == root {
            break;
        }
        let next = *p as usize;
        parent[i] = Value::Int(root as i64);
        i = next;
    }
    root
}

fn simple_hash(s: &str, seed: usize) -> usize {
    let mut h = seed;
    for b in s.bytes() {
        h = h.wrapping_mul(31).wrapping_add(b as usize);
    }
    h
}

fn val_less(a: &Value, b: &Value) -> bool {
    matches!(val_cmp(a, b), std::cmp::Ordering::Less)
}

fn val_greater(a: &Value, b: &Value) -> bool {
    matches!(val_cmp(a, b), std::cmp::Ordering::Greater)
}

fn val_cmp(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x.cmp(y),
        (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
        (Value::String(x), Value::String(y)) => x.cmp(y),
        (Value::Char(x), Value::Char(y)) => x.cmp(y),
        _ => std::cmp::Ordering::Equal,
    }
}
