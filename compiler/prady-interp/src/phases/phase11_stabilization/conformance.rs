// Phase 11 — Language Specification Conformance Suite
// Runs acceptance tests against official language specification fixtures.

#[derive(Debug, Clone)]
pub struct ConformanceTest {
    pub id: &'static str,
    pub name: &'static str,
    pub source: &'static str,
    pub expected_stdout: &'static str,
    pub should_succeed: bool,
}

pub struct ConformanceSuite;

impl ConformanceSuite {
    pub fn get_standard_tests() -> Vec<ConformanceTest> {
        vec![
            ConformanceTest {
                id: "CONF-001",
                name: "Hello World and Printing",
                source: "fn main() { print(\"Hello Prady\"); }",
                expected_stdout: "Hello Prady\n",
                should_succeed: true,
            },
            ConformanceTest {
                id: "CONF-002",
                name: "Arithmetic and Variables",
                source: "fn main() { let a = 10; let b = 20; print(a + b); }",
                expected_stdout: "30\n",
                should_succeed: true,
            },
            ConformanceTest {
                id: "CONF-003",
                name: "Function Invocation and Recursion",
                source: "fn fib(n: Int) -> Int { if n <= 1 { return n; } return fib(n - 1) + fib(n - 2); } fn main() { print(fib(6)); }",
                expected_stdout: "8\n",
                should_succeed: true,
            },
            ConformanceTest {
                id: "CONF-004",
                name: "Clean Architecture Declaration",
                source: "architecture backend { layer presentation; layer domain; presentation -> domain; } fn main() { print(\"Arch OK\"); }",
                expected_stdout: "Arch OK\n",
                should_succeed: true,
            },
        ]
    }
}
