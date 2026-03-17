// New Language Features in Linea v3.5.0

// 1. GPU Attribute - Mark functions for GPU execution
@gpu
fn matrix_multiply_gpu(a: Matrix, b: Matrix) -> Matrix {
    compute::matmul(a, b)
}

// 2. Async/Await (Preview)
@async
fn fetch_data(url: string) -> string {
    response := http::get(url)
    response
}

// Usage:
// data := await fetch_data("https://api.example.com/data")

// 3. Pattern Matching
fn classify(value: i64) -> string {
    result: string = match value {
        0 => "zero",
        1 | 2 | 3 => "small",
        10..20 => "medium",
        _ => "other"
    }
    result
}

// 4. Generic Functions (Preview)
fn first<T>(arr: Vector<T>) -> T {
    arr[0]
}

// 5. Macro Support (Preview)
// macro! create_layer(name, in_size, out_size) {
//     name: ml::Linear = ml::Linear(in_size, out_size)
// }

// 6. Type Inference Helpers
fn process_data(data) {  // Type automatically inferred from usage
    x := data[0]
    x + 1
}

// 7. Tensor Slicing Syntax
fn slice_operations(t: Tensor) {
    // Row slice
    row := t[0, :, :]
    
    // Column slice
    col := t[:, 1, :]
    
    // Range slice
    range := t[0:5, 2:8, :]
    
    // Step slice (every 2nd element)
    // stepped := t[0:10:2, :, :]
}

// 8. Inference Mode (no gradients)
@inference
fn predict_no_grad(model: ml::Model, input: Matrix) -> Matrix {
    predictions := model.forward(input)
    predictions
}

// 9. Constraint Programming (Preview)
fn solve_constraints() {
    x: i64 constraint_domain [0, 100]
    y: i64 constraint_domain [0, 100]
    
    // x > y
    // x + y <= 150
}

// 10. Parallel Iteration
for item in parallel data {
    process(item)
}
