use criterion::{black_box, criterion_group, criterion_main, Criterion};
use linea_ast::parse;

fn parser_smoke_benchmark(c: &mut Criterion) {
    let source = r#"
import math
import dsa

func fib(n: int) -> int {
    if n <= 1 {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}

var total @ int = 0
for i from 1~100 {
    total = total + i
}
display total
"#;

    c.bench_function("parse_small_program", |b| {
        b.iter(|| {
            let program = parse(black_box(source)).expect("parser should succeed");
            black_box(program);
        })
    });
}

criterion_group!(benches, parser_smoke_benchmark);
criterion_main!(benches);
