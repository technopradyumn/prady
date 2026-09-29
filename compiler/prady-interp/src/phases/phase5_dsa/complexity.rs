// Phase 5 — Formal DSA Complexity Guarantees & Contract Verification
// Provides mathematical bounds (Big-O) for each data structure operation in the standard library.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BigO {
    O1,           // Constant
    OLogN,        // Logarithmic
    ON,           // Linear
    ONLogN,       // Linearithmic
    ON2,          // Quadratic
    OAlphaN,      // Inverse Ackermann (Disjoint Set)
}

impl std::fmt::Display for BigO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BigO::O1 => write!(f, "O(1)"),
            BigO::OLogN => write!(f, "O(log N)"),
            BigO::ON => write!(f, "O(N)"),
            BigO::ONLogN => write!(f, "O(N log N)"),
            BigO::ON2 => write!(f, "O(N^2)"),
            BigO::OAlphaN => write!(f, "O(α(N))"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperationComplexity {
    pub operation: &'static str,
    pub time_average: BigO,
    pub time_worst: BigO,
    pub space: BigO,
}

#[derive(Debug, Clone)]
pub struct DsaContract {
    pub name: &'static str,
    pub description: &'static str,
    pub operations: Vec<OperationComplexity>,
}

pub fn get_dsa_specifications() -> Vec<DsaContract> {
    vec![
        DsaContract {
            name: "Vector",
            description: "Contiguous resizable array with amortized O(1) push and O(1) random access",
            operations: vec![
                OperationComplexity { operation: "get(index)", time_average: BigO::O1, time_worst: BigO::O1, space: BigO::O1 },
                OperationComplexity { operation: "push(item)", time_average: BigO::O1, time_worst: BigO::ON, space: BigO::O1 },
                OperationComplexity { operation: "pop()", time_average: BigO::O1, time_worst: BigO::O1, space: BigO::O1 },
                OperationComplexity { operation: "insert(index, item)", time_average: BigO::ON, time_worst: BigO::ON, space: BigO::O1 },
            ],
        },
        DsaContract {
            name: "HashMap",
            description: "Hash table with open addressing / robin-hood hashing",
            operations: vec![
                OperationComplexity { operation: "get(key)", time_average: BigO::O1, time_worst: BigO::ON, space: BigO::O1 },
                OperationComplexity { operation: "insert(key, value)", time_average: BigO::O1, time_worst: BigO::ON, space: BigO::O1 },
                OperationComplexity { operation: "remove(key)", time_average: BigO::O1, time_worst: BigO::ON, space: BigO::O1 },
            ],
        },
        DsaContract {
            name: "AVLTree / RedBlackTree",
            description: "Self-balancing binary search tree with strict O(log N) height bound",
            operations: vec![
                OperationComplexity { operation: "search(key)", time_average: BigO::OLogN, time_worst: BigO::OLogN, space: BigO::O1 },
                OperationComplexity { operation: "insert(key, value)", time_average: BigO::OLogN, time_worst: BigO::OLogN, space: BigO::OLogN },
                OperationComplexity { operation: "delete(key)", time_average: BigO::OLogN, time_worst: BigO::OLogN, space: BigO::OLogN },
            ],
        },
        DsaContract {
            name: "PriorityQueue / BinaryHeap",
            description: "Binary min/max heap backed by array",
            operations: vec![
                OperationComplexity { operation: "peek()", time_average: BigO::O1, time_worst: BigO::O1, space: BigO::O1 },
                OperationComplexity { operation: "push(item)", time_average: BigO::OLogN, time_worst: BigO::OLogN, space: BigO::O1 },
                OperationComplexity { operation: "pop()", time_average: BigO::OLogN, time_worst: BigO::OLogN, space: BigO::O1 },
            ],
        },
        DsaContract {
            name: "DisjointSetUnion",
            description: "Union-Find structure with union by rank and path compression",
            operations: vec![
                OperationComplexity { operation: "find(x)", time_average: BigO::OAlphaN, time_worst: BigO::OAlphaN, space: BigO::O1 },
                OperationComplexity { operation: "union(x, y)", time_average: BigO::OAlphaN, time_worst: BigO::OAlphaN, space: BigO::O1 },
            ],
        },
        DsaContract {
            name: "LRUCache",
            description: "Doubly linked list paired with hash table for O(1) evictions",
            operations: vec![
                OperationComplexity { operation: "get(key)", time_average: BigO::O1, time_worst: BigO::O1, space: BigO::O1 },
                OperationComplexity { operation: "put(key, value)", time_average: BigO::O1, time_worst: BigO::O1, space: BigO::O1 },
            ],
        },
        DsaContract {
            name: "Trie",
            description: "Prefix tree for strings with O(K) lookup where K is string length",
            operations: vec![
                OperationComplexity { operation: "insert(word)", time_average: BigO::ON, time_worst: BigO::ON, space: BigO::ON },
                OperationComplexity { operation: "search(word)", time_average: BigO::ON, time_worst: BigO::ON, space: BigO::O1 },
                OperationComplexity { operation: "starts_with(prefix)", time_average: BigO::ON, time_worst: BigO::ON, space: BigO::O1 },
            ],
        },
        DsaContract {
            name: "Graph",
            description: "Adjacency list representation supporting BFS, DFS, Dijkstra, A*, Tarjan SCC",
            operations: vec![
                OperationComplexity { operation: "add_edge(u, v)", time_average: BigO::O1, time_worst: BigO::O1, space: BigO::O1 },
                OperationComplexity { operation: "dijkstra(src)", time_average: BigO::ONLogN, time_worst: BigO::ONLogN, space: BigO::ON },
            ],
        },
    ]
}
