# Prady Language v1.0.0 (General Availability - GA) Release Notes & Architecture Summary

## 1. Release Overview

Prady has officially reached **v1.0.0 General Availability (Phase 10: GA)**. This major milestone brings full TypeScript-inspired language ergonomics, an enterprise-grade standard library containing all 28 production data structures, compile-time architecture-as-code enforcement, a built-in package manager CLI (`prady add`), and a redesigned documentation website modeled directly on the **classical MDN Web Docs UI reference**.

---

## 2. Compiler & Language Features Added

### A. TypeScript-Style `switch`, `case`, and `default`
- **Lexer Tokens**: `TokenKind::Switch`, `TokenKind::Case`, `TokenKind::Default`.
- **AST Nodes**: `Expr::Switch { condition, cases, default_case, span }` and `SwitchCase { value, body, span }`.
- **Parser & Semantics**: Evaluates target expressions and executes matching case bodies with exact equality, fall-through prevention, and fallback to `default:`. Trailing semicolons after blocks are fully optional.

```prady
let status: Int = 200;
switch (status) {
    case 200:
        print("Status 200: OK");
        break;
    case 404:
        print("Status 404: Not Found");
        break;
    default:
        print("Unknown status code");
}
```

### B. Iteration: `for..of`, `for..in`, and `foreach`
- **Tokens**: `TokenKind::ForEach`, `TokenKind::Of`, `TokenKind::In`.
- **Evaluation**: Unified loop execution across arrays, strings, sets, maps, queues, stacks, heaps, and tree structures.

```prady
let fruits = ["Apple", "Orange", "Mango"];
for (let f of fruits) {
    print("Fruit: " + f);
}
```

### C. Higher-Order Collection Methods & First-Class Lambdas
- **Lambdas**: Anonymous functions using `fn(param: Type, ...) -> ReturnType { ... }` with closure environment capture.
- **Array Methods**:
  - `map(callback: fn(item, index) -> U)`
  - `filter(predicate: fn(item, index) -> Bool)`
  - `reduce(reducer: fn(acc, item, index) -> U, initialValue?: U)`
  - `forEach(action: fn(item, index) -> Void)`
  - `find(predicate: fn(item, index) -> Bool)`
  - `findIndex(predicate: fn(item, index) -> Bool)`
  - `some(predicate: fn(item, index) -> Bool)`
  - `every(predicate: fn(item, index) -> Bool)`
  - `slice(start, end)`, `splice(start, count)`, `join(separator)`
  - `reverse()`, `sort()`, `indexOf(item)`, `includes(item)`
  - `push(item)`, `pop()`, `shift()`, `unshift(item)`
  - `at(index)`, `clear()`, `isEmpty()`, `len()` / `length`

```prady
let numbers = [1, 2, 3, 4, 5];
let evens = numbers.filter(fn(x: Int, i: Int) -> Bool {
    return x % 2 == 0;
});
let doubled = evens.map(fn(x: Int, i: Int) -> Int {
    return x * 10;
});
print(doubled); // [20, 40]
```

### D. Comprehensive OOP & Class Support
- Supports both `class` and `Class` keywords.
- Member declarations with `let name: Type = expr;` and `const name: Type = expr;`.
- Member methods with access to instance state.
- Constructors and instance creation: `let obj: MyClass = MyClass();`.

```prady
Class Hai {
    let a: Int = 3;
    let b: Int = 5;

    fn hello2() {
        print(a + b);
    }
}

fn main() {
    let hai: Hai = Hai();
    hai.hello2(); // 8
}
```

---

## 3. Complete Standard Library: 28 Production Data Structures

All 28 data structures are fully implemented in the compiler backend (`compiler/prady-interp/src/dsa.rs`), native evaluation engine, and TypeScript browser engine (`website/src/prady-engine.ts`):

| # | Data Structure | Key Methods | Time Complexity | Space Complexity |
|---|---|---|---|---|
| 1 | **Array** | `map`, `filter`, `reduce`, `forEach`, `find`, `slice`, `splice`, `join` | Access: $O(1)$, Append: $O(1)$ | $O(n)$ |
| 2 | **Map** | `set`, `get`, `has`, `delete`, `clear`, `keys`, `values`, `entries` | Get/Set: $O(1)$ | $O(n)$ |
| 3 | **Set** | `add`, `has`, `delete`, `clear`, `size`, `union`, `intersection` | Add/Has: $O(1)$ | $O(n)$ |
| 4 | **Stack** | `push`, `pop`, `peek`, `size`, `isEmpty`, `clear` | Push/Pop: $O(1)$ | $O(n)$ |
| 5 | **Queue** | `enqueue`, `dequeue`, `peek`, `size`, `isEmpty`, `clear` | Enqueue/Dequeue: $O(1)$ | $O(n)$ |
| 6 | **Deque** | `pushFront`, `pushBack`, `popFront`, `popBack`, `peekFront`, `peekBack` | Push/Pop: $O(1)$ | $O(n)$ |
| 7 | **MinHeap** | `insert`, `extractMin`, `peek`, `size`, `isEmpty` | Insert/Extract: $O(\log n)$ | $O(n)$ |
| 8 | **MaxHeap** | `insert`, `extractMax`, `peek`, `size`, `isEmpty` | Insert/Extract: $O(\log n)$ | $O(n)$ |
| 9 | **LinkedList** | `append`, `prepend`, `delete`, `find`, `size`, `toArray` | Prepend: $O(1)$, Access: $O(n)$ | $O(n)$ |
| 10 | **DoublyLinkedList** | `append`, `prepend`, `delete`, `size`, `toArray` | Prepend/Append: $O(1)$ | $O(n)$ |
| 11 | **BST** | `insert`, `search`, `inOrder`, `min`, `max`, `size` | Search: $O(\log n)$ avg | $O(n)$ |
| 12 | **AVLTree** | `insert`, `search`, `isBalanced`, `size` | Search/Insert: $O(\log n)$ worst | $O(n)$ |
| 13 | **RedBlackTree** | `insert`, `search`, `size` | Search/Insert: $O(\log n)$ worst | $O(n)$ |
| 14 | **Trie** | `insert`, `search`, `startsWith`, `delete`, `wordsWithPrefix` | Prefix Search: $O(L)$ | $O(\Sigma \cdot L)$ |
| 15 | **Graph** | `addVertex`, `addEdge`, `neighbors`, `bfs`, `dfs`, `hasPath` | BFS/DFS: $O(V + E)$ | $O(V + E)$ |
| 16 | **LRUCache** | `put`, `get`, `size`, `capacity`, `has` | Put/Get: $O(1)$ | $O(C)$ |
| 17 | **LFUCache** | `put`, `get`, `size`, `capacity` | Put/Get: $O(1)$ | $O(C)$ |
| 18 | **CircularBuffer** | `write`, `read`, `isFull`, `isEmpty`, `size`, `capacity` | Write/Read: $O(1)$ | $O(C)$ |
| 19 | **BloomFilter** | `add`, `mightContain` | Add/Query: $O(k)$ | $O(m)$ bits |
| 20 | **DisjointSet** | `union`, `find`, `connected` | Union/Find: $O(\alpha(n))$ | $O(n)$ |
| 21 | **SegmentTree** | `query`, `update`, `build` | Range Query: $O(\log n)$ | $O(4n)$ |
| 22 | **FenwickTree** | `update`, `query`, `rangeQuery` | Prefix Query: $O(\log n)$ | $O(n)$ |
| 23 | **BitSet** | `set`, `clear`, `get`, `count`, `size` | Set/Get: $O(1)$ | $O(n / 64)$ |
| 24 | **SkipList** | `insert`, `search`, `delete`, `size` | Search/Insert: $O(\log n)$ avg | $O(n)$ |
| 25 | **Matrix** | `get`, `set`, `multiply`, `transpose`, `rows`, `cols` | Multiply: $O(n^3)$ | $O(R \times C)$ |
| 26 | **SparseMatrix** | `set`, `get`, `nonZeroCount` | Get: $O(1)$ avg | $O(\text{nnz})$ |
| 27 | **TreeMap** | `put`, `get`, `minKey`, `maxKey`, `size` | Get/Put: $O(\log n)$ | $O(n)$ |
| 28 | **TreeSet** | `add`, `min`, `max`, `has`, `size` | Add/Has: $O(\log n)$ | $O(n)$ |

---

## 4. Documentation Website: Classical MDN Web Docs Architecture

The website has been completely redesigned with a strict **MDN Web Docs UI reference**:
- **Design Aesthetic**: Pure, clean technical documentation. No flashy modern neon, no glowing cards, no distractions.
- **Header**: Classical MDN bar with logo (`.pr Prady Docs`), global search input with keyboard shortcut `/`, Documentation/Playground tab toggles, light/dark theme switch, and version badge `v1.0.0 GA`.
- **Sidebar**: Complete categorized hierarchy (Guides, Language Reference, 28 Data Structures with deep sub-method navigation).
- **Article Structure**:
  - Breadcrumb navigation (`References / Data Structures / Array / map()`)
  - Title and status badge
  - Lead summary paragraph
  - MDN-style Syntax Box with blue accent border
  - Parameters Table (Name, Type, Description)
  - Return Value specification
  - Complexity Table (Time and Space Big-O)
  - Deep description and edge cases
  - Code Examples with interactive "Run in Playground" and "Copy" actions
  - See Also cross-references
- **Interactive In-Browser Playground**:
  - Split editor and terminal stdout pane.
  - Compiles and runs Prady code instantly with live execution timing and diagnostics.

---

## 5. Pure TypeScript Codebase Guarantee

In accordance with strict requirements ("never use javascript use only typescript for writing code in this programming language project"):
- All website logic is authored exclusively in pure TypeScript under [`website/src/`](file:///c:/Users/techn/Desktop/Acciojob/Programming%20Language/website/src):
  - [`types.ts`](file:///c:/Users/techn/Desktop/Acciojob/Programming%20Language/website/src/types.ts): Data contracts for parameters, return types, methods, and doc items.
  - [`docs-data.ts`](file:///c:/Users/techn/Desktop/Acciojob/Programming%20Language/website/src/docs-data.ts): Comprehensive guides for syntax, CLI, OOP, and architecture.
  - [`dsa-docs.ts`](file:///c:/Users/techn/Desktop/Acciojob/Programming%20Language/website/src/dsa-docs.ts): Detailed specifications for all 28 data structures and methods.
  - [`prady-engine.ts`](file:///c:/Users/techn/Desktop/Acciojob/Programming%20Language/website/src/prady-engine.ts): TypeScript compiler, parser, AST, and evaluator runtime.
  - [`app.ts`](file:///c:/Users/techn/Desktop/Acciojob/Programming%20Language/website/src/app.ts): MDN application router, sidebar manager, search, and playground coordinator.
- All legacy root `.js` files were removed from source control.
- TypeScript compiler (`tsc`) compiles cleanly to [`website/dist/`](file:///c:/Users/techn/Desktop/Acciojob/Programming%20Language/website/dist).

---

## 6. Verification and Validation Results

1. **Compiler Workspace Tests**:
   - `cargo test --workspace` passed 100% (17 unit and integration tests).
2. **Release Binary Build & Installation**:
   - Built with `cargo build --release --bin prady`.
   - Installed to `C:\Users\techn\.cargo\bin\prady.exe`.
3. **Execution Tests**:
   - `prady version`: Outputs `prady version 1.0.0 (Phase 10: General Availability - GA)`.
   - `prady run examples/v1_features_test.pr`: Passed all switch, map, filter, reduce, for-of, stack, queue, heap, cache, and trie verifications.
   - `prady run examples/hello_class_test.pr`: Passed class declaration (`Class Hai`), instance fields (`let a: Int = 3`), methods (`fn hello2`), and function execution (`hello()`).
   - `prady add lodash-pr`: Package added and updated in `prady.toml`.
   - `prady check examples/hello_class_test.pr`: 0 errors.
4. **TypeScript Build**:
   - `npx -p typescript tsc --project tsconfig.json` exited with code `0`.
