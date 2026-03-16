use wgpu;
use pollster;
use std::sync::OnceLock;
use wgpu::util::DeviceExt;

use rand::Rng;

pub fn random(rows: usize, cols: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut matrix = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            row.push(rng.gen::<f64>());
        }
        matrix.push(row);
    }
    matrix
}

pub fn one_hot(labels: &[f64], classes: usize) -> Vec<Vec<f64>> {
    let mut res = vec![vec![0.0; classes]; labels.len()];
    for (i, &label) in labels.iter().enumerate() {
        let idx = label as usize;
        if idx < classes {
            res[i][idx] = 1.0;
        }
    }
    res
}

pub fn transpose(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    if a.is_empty() { return vec![]; }
    let rows = a.len();
    let cols = a[0].len();
    let mut result = vec![vec![0.0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = a[i][j];
        }
    }
    result
}

pub fn exp(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    a.iter().map(|row| row.iter().map(|x| x.exp()).collect()).collect()
}

pub fn sqrt(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    a.iter().map(|row| row.iter().map(|x| x.sqrt()).collect()).collect()
}

pub fn log(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    a.iter().map(|row| row.iter().map(|x| x.ln()).collect()).collect()
}

pub fn sum(a: &Vec<Vec<f64>>) -> f64 {
    a.iter().map(|row| row.iter().sum::<f64>()).sum()
}

pub fn sum_columns(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    if a.is_empty() { return vec![]; }
    let cols = a[0].len();
    let mut result = vec![0.0; cols];
    for row in a {
        for (i, val) in row.iter().enumerate() {
            if i < cols {
                result[i] += val;
            }
        }
    }
    vec![result]
}

pub fn max(a: &Vec<Vec<f64>>) -> f64 {
    a.iter().flatten().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
}

pub fn argmax(a: &Vec<Vec<f64>>) -> f64 {
    // Flattens and returns index
    a.iter().flatten()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index as f64)
        .unwrap_or(0.0)
}

pub fn relu(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    a.iter().map(|row| row.iter().map(|&x| if x > 0.0 { x } else { 0.0 }).collect()).collect()
}

pub fn sigmoid(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    a.iter().map(|row| row.iter().map(|&x| 1.0 / (1.0 + (-x).exp())).collect()).collect()
}

pub fn softmax(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    a.iter().map(|row| {
        let max_val = row.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let exps: Vec<f64> = row.iter().map(|x| (x - max_val).exp()).collect();
        let sum_exps: f64 = exps.iter().sum();
        exps.iter().map(|x| x / sum_exps).collect()
    }).collect()
}

pub fn cross_entropy(pred: &Vec<Vec<f64>>, target: &Vec<Vec<f64>>) -> f64 {
    let mut loss = 0.0;
    let rows = pred.len();
    if rows == 0 { return 0.0; }
    
    for (i, row) in pred.iter().enumerate() {
        if i < target.len() {
             for (j, &p) in row.iter().enumerate() {
                 if j < target[i].len() {
                     let t = target[i][j];
                     if t > 0.0 {
                         loss -= t * (p + 1e-15).ln();
                     }
                 }
             }
        }
    }
    loss / rows as f64
}

pub fn tanh(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    a.iter().map(|row| row.iter().map(|&x| x.tanh()).collect()).collect()
}

static COMPUTE_CONTEXT: OnceLock<Option<ComputeContext>> = OnceLock::new();

pub struct ComputeContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub info: String,
    pub adapter_info: wgpu::AdapterInfo,
}

const SHADER: &str = r#"
struct Uniforms {
    M: u32,
    K: u32,
    N: u32,
};

@group(0) @binding(0) var<storage, read> a: array<f32>;
@group(0) @binding(1) var<storage, read> b: array<f32>;
@group(0) @binding(2) var<storage, read_write> c: array<f32>;
@group(0) @binding(3) var<uniform> u: Uniforms;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let row = global_id.x;
    let col = global_id.y;

    if (row >= u.M || col >= u.N) {
        return;
    }

    var sum = 0.0;
    for (var k = 0u; k < u.K; k = k + 1u) {
        sum = sum + a[row * u.K + k] * b[k * u.N + col];
    }

    c[row * u.N + col] = sum;
}
"#;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    M: u32,
    K: u32,
    N: u32,
}

const ELEMENT_WISE_SHADER: &str = r#"
struct Uniforms {
    size: u32,
    op: u32, // 0: add, 1: sub, 2: mul, 3: div
};

@group(0) @binding(0) var<storage, read> a: array<f32>;
@group(0) @binding(1) var<storage, read> b: array<f32>;
@group(0) @binding(2) var<storage, read_write> c: array<f32>;
@group(0) @binding(3) var<uniform> u: Uniforms;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = global_id.x;
    if (i >= u.size) {
        return;
    }

    if (u.op == 0u) {
        c[i] = a[i] + b[i];
    } else if (u.op == 1u) {
        c[i] = a[i] - b[i];
    } else if (u.op == 2u) {
        c[i] = a[i] * b[i];
    } else if (u.op == 3u) {
        c[i] = a[i] / b[i];
    } else if (u.op == 4u) {
        c[i] = pow(a[i], b[i]);
    }
}
"#;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ElementWiseUniforms {
    size: u32,
    op: u32,
}

const BROADCAST_OP_SHADER: &str = r#"
struct Uniforms {
    M: u32,
    N: u32,
    op: u32, // 0: add, 1: sub, 2: mul, 3: div
};

@group(0) @binding(0) var<storage, read> a: array<f32>;
@group(0) @binding(1) var<storage, read> b: array<f32>;
@group(0) @binding(2) var<storage, read_write> c: array<f32>;
@group(0) @binding(3) var<uniform> u: Uniforms;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let row = global_id.x;
    let col = global_id.y;

    if (row >= u.M || col >= u.N) {
        return;
    }

    let index = row * u.N + col;
    let val_a = a[index];
    let val_b = b[col];
    
    if (u.op == 0u) {
        c[index] = val_a + val_b;
    } else if (u.op == 1u) {
        c[index] = val_a - val_b;
    } else if (u.op == 2u) {
        c[index] = val_a * val_b;
    } else if (u.op == 3u) {
        c[index] = val_a / val_b;
    } else if (u.op == 4u) {
        c[index] = pow(val_a, val_b);
    }
}
"#;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct BroadcastUniforms {
    M: u32,
    N: u32,
    op: u32,
}

impl ComputeContext {
    pub fn global() -> Option<&'static ComputeContext> {
        COMPUTE_CONTEXT.get_or_init(|| {
            pollster::block_on(async {
                let instance = wgpu::Instance::default();
                
                let mut adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: None,
                    force_fallback_adapter: false,
                }).await;

                if adapter.is_none() {
                     adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::LowPower,
                        compatible_surface: None,
                        force_fallback_adapter: false,
                    }).await;
                }

                if adapter.is_none() {
                    adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::None,
                        compatible_surface: None,
                        force_fallback_adapter: true,
                    }).await;
                }

                let adapter = adapter?;

                let (device, queue) = adapter.request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("Linea Compute Device"),
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::downlevel_defaults(),
                    },
                    None,
                ).await.ok()?;

                let adapter_info = adapter.get_info();
                let info = format!("{} ({:?})", adapter_info.name, adapter_info.backend);

                Some(Self {
                    device,
                    queue,
                    info,
                    adapter_info,
                })
            })
        }).as_ref()
    }
}

pub fn device() -> String {
    if let Some(ctx) = ComputeContext::global() {
        ctx.info.clone()
    } else {
        "CPU (Software Fallback - No GPU Access)".to_string()
    }
}

pub fn device_type() -> String {
    if let Some(ctx) = ComputeContext::global() {
        format!("{:?}", ctx.adapter_info.device_type)
    } else {
        "Cpu".to_string()
    }
}

pub fn matmul(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let m = a.len();
    let k = a[0].len();
    if b.len() != k { return vec![]; }
    let n = b[0].len();
    
    let mut a_flat = Vec::with_capacity(m * k);
    for row in a {
        for &val in row {
            a_flat.push(val as f32);
        }
    }

    let mut b_flat = Vec::with_capacity(k * n);
    for row in b {
        for &val in row {
            b_flat.push(val as f32);
        }
    }

    let result_flat = matmul_impl(&a_flat, &b_flat, m as u32, k as u32, n as u32);
    
    if let Some(res) = result_flat {
        let mut result = Vec::with_capacity(m);
        for i in 0..m {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                row.push(res[i * n + j] as f64);
            }
            result.push(row);
        }
        result
    } else {
        vec![]
    }
}

fn matmul_impl(a: &[f32], b: &[f32], M: u32, K: u32, N: u32) -> Option<Vec<f32>> {
    let ctx = ComputeContext::global()?;
    let device = &ctx.device;
    let queue = &ctx.queue;

    pollster::block_on(async {
        let a_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Matrix A Buffer"),
            contents: bytemuck::cast_slice(a),
            usage: wgpu::BufferUsages::STORAGE,
        });
        
        let b_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Matrix B Buffer"),
            contents: bytemuck::cast_slice(b),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let output_size = (M * N) as usize * std::mem::size_of::<f32>();
        let c_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: output_size as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: output_size as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniforms = Uniforms { M, K, N };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Matmul Shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(SHADER)),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: a_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: b_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: c_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: uniform_buffer.as_entire_binding() },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main",
            compilation_options: Default::default(),
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Command Encoder") });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Compute Pass"), timestamp_writes: None });
            cpass.set_pipeline(&pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            let workgroup_size_x = 8;
            let workgroup_size_y = 8;
            cpass.dispatch_workgroups(
                (M + workgroup_size_x - 1) / workgroup_size_x,
                (N + workgroup_size_y - 1) / workgroup_size_y,
                1
            );
        }
        
        encoder.copy_buffer_to_buffer(&c_buffer, 0, &staging_buffer, 0, output_size as u64);
        queue.submit(Some(encoder.finish()));

        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures::channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        device.poll(wgpu::Maintain::Wait);

        if let Ok(Ok(())) = receiver.await {
            let data = buffer_slice.get_mapped_range();
            let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
            drop(data);
            staging_buffer.unmap();
            Some(result)
        } else {
            None
        }
    })
}

pub fn broadcast_op(a: &Vec<Vec<f64>>, b: &Vec<f64>, op: &str) -> Vec<Vec<f64>> {
    let m = a.len();
    if m == 0 { return vec![]; }
    let n = a[0].len();
    if b.len() != n { return vec![]; }
    
    let mut a_flat = Vec::with_capacity(m * n);
    for row in a {
        for &val in row {
            a_flat.push(val);
        }
    }
    
    let res_flat = broadcast_op_flat(&a_flat, b, m, n, op);
    
    let mut result = Vec::with_capacity(m);
    for i in 0..m {
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            row.push(res_flat[i * n + j]);
        }
        result.push(row);
    }
    result
}

pub fn broadcast_op_flat(a: &[f64], b: &[f64], m: usize, n: usize, op: &str) -> Vec<f64> {
    let op_code = match op {
        "add" => 0,
        "sub" => 1,
        "mul" => 2,
        "div" => 3,
        "pow" => 4,
        _ => return vec![],
    };

    let a_f32: Vec<f32> = a.iter().map(|&x| x as f32).collect();
    let b_f32: Vec<f32> = b.iter().map(|&x| x as f32).collect();
    
    if let Some(res) = broadcast_op_impl(&a_f32, &b_f32, m as u32, n as u32, op_code) {
        res.into_iter().map(|x| x as f64).collect()
    } else {
        vec![]
    }
}

fn broadcast_op_impl(a: &[f32], b: &[f32], m: u32, n: u32, op_code: u32) -> Option<Vec<f32>> {
    let ctx = ComputeContext::global()?;
    let device = &ctx.device;
    let queue = &ctx.queue;

    pollster::block_on(async {
        let a_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Matrix A Buffer"),
            contents: bytemuck::cast_slice(a),
            usage: wgpu::BufferUsages::STORAGE,
        });
        
        let b_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vector B Buffer"),
            contents: bytemuck::cast_slice(b),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let output_size = (m * n) as usize * std::mem::size_of::<f32>();
        let c_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: output_size as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: output_size as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniforms = BroadcastUniforms { M: m, N: n, op: op_code };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Broadcast Op Shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(BROADCAST_OP_SHADER)),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: a_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: b_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: c_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: uniform_buffer.as_entire_binding() },
            ],
        });
        
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main",
            compilation_options: Default::default(),
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Command Encoder") });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Compute Pass"), timestamp_writes: None });
            cpass.set_pipeline(&pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            let workgroup_size_x = 8;
            let workgroup_size_y = 8;
            cpass.dispatch_workgroups(
                (m + workgroup_size_x - 1) / workgroup_size_x,
                (n + workgroup_size_y - 1) / workgroup_size_y,
                1
            );
        }
        
        encoder.copy_buffer_to_buffer(&c_buffer, 0, &staging_buffer, 0, output_size as u64);
        queue.submit(Some(encoder.finish()));
        
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures::channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        device.poll(wgpu::Maintain::Wait);

        if let Ok(Ok(())) = receiver.await {
            let data = buffer_slice.get_mapped_range();
            let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
            drop(data);
            staging_buffer.unmap();
            Some(result)
        } else {
            None
        }
    })
}

pub fn element_wise(a: &Vec<f64>, b: &Vec<f64>, op: &str) -> Vec<f64> {
    let size = a.len();
    if b.len() != size { return vec![]; }

    let a_f32: Vec<f32> = a.iter().map(|&x| x as f32).collect();
    let b_f32: Vec<f32> = b.iter().map(|&x| x as f32).collect();

    let op_code = match op {
        "add" => 0,
        "sub" => 1,
        "mul" => 2,
        "div" => 3,
        "pow" => 4,
        _ => return vec![],
    };

    if let Some(res) = element_wise_impl(&a_f32, &b_f32, size as u32, op_code) {
        res.into_iter().map(|x| x as f64).collect()
    } else {
        vec![]
    }
}

fn element_wise_impl(a: &[f32], b: &[f32], size: u32, op: u32) -> Option<Vec<f32>> {
    let ctx = ComputeContext::global()?;
    let device = &ctx.device;
    let queue = &ctx.queue;

    pollster::block_on(async {
        let a_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Input A Buffer"),
            contents: bytemuck::cast_slice(a),
            usage: wgpu::BufferUsages::STORAGE,
        });
        
        let b_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Input B Buffer"),
            contents: bytemuck::cast_slice(b),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let output_size = size as usize * std::mem::size_of::<f32>();
        let c_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: output_size as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: output_size as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniforms = ElementWiseUniforms { size, op };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ElementWise Shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(ELEMENT_WISE_SHADER)),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: a_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: b_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: c_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: uniform_buffer.as_entire_binding() },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main",
            compilation_options: Default::default(),
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Command Encoder") });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Compute Pass"), timestamp_writes: None });
            cpass.set_pipeline(&pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            let workgroup_size = 64;
            cpass.dispatch_workgroups((size as u32 + workgroup_size - 1) / workgroup_size, 1, 1);
        }
        
        encoder.copy_buffer_to_buffer(&c_buffer, 0, &staging_buffer, 0, output_size as u64);
        queue.submit(Some(encoder.finish()));

        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures::channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        device.poll(wgpu::Maintain::Wait);

        if let Ok(Ok(())) = receiver.await {
            let data = buffer_slice.get_mapped_range();
            let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
            drop(data);
            staging_buffer.unmap();
            Some(result)
        } else {
            None
        }
    })
}
