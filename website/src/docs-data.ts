import { DocItem } from './types';

export const LANGUAGE_GUIDES: DocItem[] = [
  {
    id: 'guide-getting-started',
    title: 'Getting Started with Prady',
    category: 'Guides',
    subCategory: 'Introduction',
    breadcrumbs: ['Guides', 'Introduction', 'Getting Started'],
    summary: 'An introduction to the Prady programming language, installation instructions, and writing your first program.',
    badge: 'v1.0.0 GA',
    overview: `Prady is a statically-typed, high-performance systems and application programming language with built-in architecture-as-code verification, production-grade data structures, TypeScript-inspired syntax ergonomics, and native compilation via LLVM.

Prady combines the safety and architecture enforcement needed for mission-critical enterprise systems with the developer ergonomics of modern TypeScript and Rust.`,
    example: `// hello.pr
fn main() {
    print("Welcome to Prady v1.0.0!");
    let numbers = [1, 2, 3, 4, 5];
    let evens = numbers.filter(fn(x: Int, i: Int) -> Bool {
        return x % 2 == 0;
    });
    print("Even numbers: " + evens.join(", "));
}`,
    seeAlso: [
      { title: 'Variables & Types', link: '#guide-variables' },
      { title: 'CLI & Package Manager', link: '#guide-cli' }
    ]
  },
  {
    id: 'guide-cli',
    title: 'CLI & Package Manager (prady add)',
    category: 'Guides',
    subCategory: 'Tooling',
    breadcrumbs: ['Guides', 'Tooling', 'CLI & Packages'],
    summary: 'Complete reference for the Prady Command Line Interface, compiler commands, and package management.',
    badge: 'CLI Tooling',
    overview: `The \`prady\` CLI provides end-to-end tooling for project scaffolding, compilation, type checking, architecture verification, execution, and dependency resolution.

### Basic Commands
- \`prady run <file.pr>\` - Compiles and runs a source file immediately.
- \`prady check <file.pr>\` - Performs full lexical, grammatical, semantic, and architecture policy validation without executing.
- \`prady build <file.pr>\` - Compiles the project into an optimized native standalone binary using the LLVM backend.
- \`prady add <package>\` - Adds a dependency to your \`prady.toml\` file with automatic semver tracking.
- \`prady version\` - Displays the current compiler version and target platform.`,
    example: `# Run your program
prady run main.pr

# Add a package to prady.toml
prady add http-server

# Verify architecture and types
prady check main.pr`,
    seeAlso: [
      { title: 'Architecture Verification', link: '#guide-architecture' }
    ]
  },
  {
    id: 'guide-variables',
    title: 'Variables, Constants & Scoping',
    category: 'Language Reference',
    subCategory: 'Core Syntax',
    breadcrumbs: ['Language Reference', 'Core Syntax', 'Variables & Constants'],
    summary: 'Declaring variables with let, immutable constants with const, and lexical scope rules in Prady.',
    badge: 'Core Syntax',
    constructorSyntax: 'let identifier: Type = initialValue;\nconst identifier: Type = initialValue;',
    overview: `Prady enforces strict block scoping. Variables declared with \`let\` are mutable within their lexical scope, while variables declared with \`const\` are immutable after initialization.

Type annotations are optional when the type can be inferred unambiguously from the right-hand expression, but explicit type annotations are encouraged for public interfaces.`,
    example: `// Mutable variable with explicit type
let count: Int = 0;
count = count + 1;

// Immutable constant with type inference
const MAX_RETRIES: Int = 5;

// Block scoping
{
    let innerScope = "isolated";
    print(innerScope);
}
// innerScope is out of scope here`,
    seeAlso: [
      { title: 'Primitive Types', link: '#guide-types' }
    ]
  },
  {
    id: 'guide-types',
    title: 'Primitive & Composite Types',
    category: 'Language Reference',
    subCategory: 'Type System',
    breadcrumbs: ['Language Reference', 'Type System', 'Types'],
    summary: 'Comprehensive guide to primitive numbers, floating points, strings, booleans, arrays, maps, and monads.',
    badge: 'Type System',
    overview: `Prady features a rich, sound static type system:
- **Integers**: \`Int\` (default 64-bit), \`Int8\`, \`Int16\`, \`Int32\`, \`Int64\`, \`UInt\`, \`UInt8\`, \`UInt16\`, \`UInt32\`, \`UInt64\`
- **Floating Points**: \`Float\` (default 64-bit IEEE 754), \`Float32\`, \`Float64\`
- **Booleans**: \`Bool\` (\`true\` or \`false\`)
- **Strings**: \`String\` (UTF-8 encoded, with built-in string manipulation methods)
- **Collections**: \`Array<T>\`, \`Map<K, V>\`, \`Set<T>\`
- **Monads**: \`Option<T>\` (\`Some(T)\` | \`None\`), \`Result<T, E>\` (\`Ok(T)\` | \`Err(E)\`)`,
    example: `let age: Int = 28;
let ratio: Float = 3.14159265;
let isActive: Bool = true;
let greeting: String = "Hello, Prady!";
let scores: Array<Int> = [95, 88, 100];`,
    seeAlso: [
      { title: 'Pattern Matching & Monads', link: '#guide-monads' }
    ]
  },
  {
    id: 'guide-control-flow',
    title: 'Control Flow: If, While, For & Foreach',
    category: 'Language Reference',
    subCategory: 'Statements',
    breadcrumbs: ['Language Reference', 'Statements', 'Control Flow'],
    summary: 'Branching and looping constructs including if/else, while loops, for..of, for..in, and foreach.',
    badge: 'Statements',
    overview: `Prady supports modern iteration patterns inspired by TypeScript and Rust. In addition to traditional while loops, you can iterate over collections, arrays, maps, and custom data structures using \`for (let item of collection)\` or \`foreach\`.`,
    example: `// 1. If-Else conditional
let temp = 25;
if (temp > 30) {
    print("Hot");
} else if (temp > 20) {
    print("Comfortable");
} else {
    print("Cold");
}

// 2. For..of iteration over Array
let fruits = ["Apple", "Banana", "Cherry"];
for (let fruit of fruits) {
    print("Fruit: " + fruit);
}

// 3. For..in iteration over indices/keys
for (let idx in fruits) {
    print("Index: " + idx);
}

// 4. Foreach statement
foreach (let item of fruits) {
    print("Item: " + item);
}`,
    seeAlso: [
      { title: 'Switch Statements', link: '#guide-switch' }
    ]
  },
  {
    id: 'guide-switch',
    title: 'Switch Statements (TypeScript-style)',
    category: 'Language Reference',
    subCategory: 'Statements',
    breadcrumbs: ['Language Reference', 'Statements', 'Switch'],
    summary: 'Multi-branch evaluation using switch, case, and default clauses with exact value evaluation.',
    badge: 'New in v1.0.0',
    constructorSyntax: `switch (expression) {
    case value1:
        // statements
        break;
    case value2:
        // statements
        break;
    default:
        // fallback statements
}`,
    overview: `The \`switch\` statement evaluates an expression, matching the expression's value against a series of \`case\` clauses, and executes statements after the first case clause with a matching value, until a \`break\` statement is encountered or the switch terminates.

An optional \`default\` clause acts as the fallback if no case matches.`,
    example: `let role = "admin";

switch (role) {
    case "admin":
        print("Full system access granted.");
        break;
    case "editor":
        print("Content editing permissions.");
        break;
    case "viewer":
        print("Read-only access.");
        break;
    default:
        print("Unknown role. Access denied.");
}`,
    seeAlso: [
      { title: 'Control Flow', link: '#guide-control-flow' }
    ]
  },
  {
    id: 'guide-functions',
    title: 'Functions & First-Class Lambdas',
    category: 'Language Reference',
    subCategory: 'Functions',
    breadcrumbs: ['Language Reference', 'Functions', 'Lambdas'],
    summary: 'Defining named functions, anonymous lambda expressions, closures, and higher-order callbacks.',
    badge: 'First-Class Functions',
    overview: `In Prady, functions are first-class citizens. They can be assigned to variables, passed as arguments to higher-order functions (such as \`map\`, \`filter\`, \`reduce\`), and returned from other functions.

Anonymous functions (lambdas) use the syntax:
\`fn(param1: Type1, param2: Type2) -> ReturnType { ... }\``,
    example: `// Named function
fn add(a: Int, b: Int) -> Int {
    return a + b;
}

// Anonymous lambda passed to higher-order method
let numbers = [10, 20, 30, 40];
let incremented = numbers.map(fn(x: Int, i: Int) -> Int {
    return x + 1;
});
print(incremented); // [11, 21, 31, 41]

// Higher-order function accepting a callback
fn executeTwice(value: Int, action: fn(v: Int) -> Int) -> Int {
    return action(action(value));
}

let result = executeTwice(5, fn(n: Int) -> Int {
    return n * 2;
});
print(result); // 20`,
    seeAlso: [
      { title: 'Array Methods', link: '#dsa-array' }
    ]
  },
  {
    id: 'guide-classes',
    title: 'Classes, OOP & Object Instantiation',
    category: 'Language Reference',
    subCategory: 'OOP',
    breadcrumbs: ['Language Reference', 'OOP', 'Classes'],
    summary: 'Object-Oriented Programming in Prady: class declarations, instance fields, methods, and constructors.',
    badge: 'OOP Engine',
    overview: `Prady provides comprehensive Object-Oriented programming capabilities. Classes define blueprints for objects with typed fields and member functions.`,
    example: `class Hai {
    let a: Int = 3;
    let b: Int = 5;

    fn hello2() {
        print(a + b);
    }
}

fn main() {
    let hai: Hai = Hai();
    hai.hello2(); // prints 8
}`,
    seeAlso: [
      { title: 'Architecture Verification', link: '#guide-architecture' }
    ]
  },
  {
    id: 'guide-monads',
    title: 'Error Handling, Option & Result Monads',
    category: 'Language Reference',
    subCategory: 'Error Handling',
    breadcrumbs: ['Language Reference', 'Error Handling', 'Result & Option'],
    summary: 'Safe, expressive error handling without unchecked exceptions using Result, Option, and the ? operator.',
    badge: 'Monads',
    overview: `Prady eliminates null pointer exceptions and unhandled crashes through algebraic data types:
- \`Option<T>\`: Expresses the possible absence of a value (\`Some(val)\` or \`None\`).
- \`Result<T, E>\`: Expresses operations that may fail (\`Ok(val)\` or \`Err(error)\`).
- \`?\` Operator: Propagates errors automatically up the call stack.`,
    example: `fn divide(numerator: Int, denominator: Int) -> Result<Int, String> {
    if (denominator == 0) {
        return Err("Division by zero error");
    }
    return Ok(numerator / denominator);
}

fn calculate() -> Result<Int, String> {
    let answer = divide(100, 5)?;
    return Ok(answer + 10);
}`,
    seeAlso: [
      { title: 'Primitive Types', link: '#guide-types' }
    ]
  },
  {
    id: 'guide-architecture',
    title: 'Architecture as Code (SOLID Heuristics)',
    category: 'Language Reference',
    subCategory: 'Enterprise Features',
    breadcrumbs: ['Language Reference', 'Enterprise Features', 'Architecture as Code'],
    summary: 'Compile-time architectural rules, dependency inversion checks, cyclomatic complexity limits, and SOLID verification.',
    badge: 'Enterprise Engine',
    overview: `Prady introduces **Architecture as Code** directly into the compiler. Define structural boundaries, layer dependencies, and quality limits directly in your source code. If any team member violates architectural contracts, the compiler fails with a high-priority diagnostic.`,
    example: `architecture CoreBanking {
    layer Domain {
        allow: [] // Domain cannot depend on outer layers
    }
    layer Service {
        allow: [Domain]
    }
    layer Presentation {
        allow: [Service, Domain]
    }
    enforce: [SRP, DIP]
    max_cyclomatic_complexity: 12
}`,
    seeAlso: [
      { title: 'CLI & Package Manager', link: '#guide-cli' }
    ]
  }
];
