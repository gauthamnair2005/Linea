
pub mod linea_runtime {
    pub mod csv {
        use std::fs::File;
        use std::io::Cursor;
        use std::collections::HashSet;

        pub fn parse(text: String) -> Vec<Vec<String>> {
            let mut rdr = ::csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(Cursor::new(text));
            
            let mut rows = Vec::new();
            for result in rdr.records() {
                if let Ok(record) = result {
                    let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                    rows.push(row);
                }
            }
            rows
        }

        pub fn stringify(data: &Vec<Vec<String>>) -> String {
            let mut wtr = ::csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(vec![]);
            
            for row in data {
                let _ = wtr.write_record(row);
            }
            
            String::from_utf8(wtr.into_inner().unwrap_or(vec![])).unwrap_or_default()
        }

        pub fn read(filepath: String) -> Vec<Vec<String>> {
            let file = match File::open(&filepath) {
                Ok(f) => f,
                Err(_) => return vec![],
            };

            let mut rdr = ::csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);
            
            let mut rows = Vec::new();
            for result in rdr.records() {
                if let Ok(record) = result {
                    let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                    rows.push(row);
                }
            }
            rows
        }

        pub fn write(filepath: String, data: &Vec<Vec<String>>) -> bool {
            let file = match File::create(&filepath) {
                Ok(f) => f,
                Err(_) => return false,
            };

            let mut wtr = ::csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(file);
            
            for row in data {
                if wtr.write_record(row).is_err() {
                    return false;
                }
            }
            wtr.flush().is_ok()
        }

        pub fn headers(data: &Vec<Vec<String>>) -> Vec<String> {
            if data.is_empty() {
                vec![]
            } else {
                data[0].clone()
            }
        }

        pub fn rows(data: &Vec<Vec<String>>) -> Vec<Vec<String>> {
            if data.len() <= 1 {
                vec![]
            } else {
                data[1..].to_vec()
            }
        }

        pub fn get_column(data: &Vec<Vec<String>>, col_name: String) -> Vec<String> {
            if data.is_empty() {
                return vec![];
            }
            
            let headers = &data[0];
            let col_idx = headers.iter().position(|h| h == &col_name);
            
            match col_idx {
                Some(idx) => {
                    data.iter().skip(1)
                        .map(|row| {
                            if idx < row.len() {
                                row[idx].clone()
                            } else {
                                String::new()
                            }
                        })
                        .collect()
                }
                None => vec![],
            }
        }

        pub fn filter(data: &Vec<Vec<String>>, col_name: String, value: String) -> Vec<Vec<String>> {
            if data.is_empty() {
                return vec![];
            }
            
            let headers = data[0].clone();
            let col_idx = headers.iter().position(|h| h == &col_name);
            
            match col_idx {
                Some(idx) => {
                    let mut result = vec![headers];
                    for row in data.iter().skip(1) {
                        if idx < row.len() && row[idx] == value {
                            result.push(row.clone());
                        }
                    }
                    result
                }
                None => vec![headers],
            }
        }

        pub fn sort(data: &Vec<Vec<String>>, col_name: String) -> Vec<Vec<String>> {
            if data.is_empty() {
                return vec![];
            }
            
            let mut data_rows = data.clone();
            let headers = data_rows.remove(0);
            let col_idx = headers.iter().position(|h| h == &col_name);
            
            match col_idx {
                Some(idx) => {
                    data_rows.sort_by(|a, b| {
                        let a_val = if idx < a.len() { &a[idx] } else { "" };
                        let b_val = if idx < b.len() { &b[idx] } else { "" };
                        
                        // Try numeric sort first
                        if let (Ok(a_num), Ok(b_num)) = (a_val.parse::<f64>(), b_val.parse::<f64>()) {
                             a_num.partial_cmp(&b_num).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                             a_val.cmp(b_val)
                        }
                    });
                    
                    let mut result = vec![headers];
                    result.extend(data_rows);
                    result
                }
                None => {
                    let mut result = vec![headers];
                    result.extend(data_rows);
                    result
                }
            }
        }

        pub fn unique(data: &Vec<Vec<String>>, col_name: String) -> Vec<String> {
            if data.is_empty() {
                return vec![];
            }
            
            let headers = &data[0];
            let col_idx = headers.iter().position(|h| h == &col_name);
            
            match col_idx {
                Some(idx) => {
                    let mut unique_vals = vec![];
                    let mut seen = HashSet::new();
                    
                    for row in data.iter().skip(1) {
                        if idx < row.len() {
                            let val = &row[idx];
                            if !seen.contains(val) {
                                seen.insert(val.clone());
                                unique_vals.push(val.clone());
                            }
                        }
                    }
                    unique_vals
                }
                None => vec![],
            }
        }

        pub fn stats(data: &Vec<Vec<String>>, col_name: String) -> Vec<f64> {
            if data.is_empty() {
                return vec![];
            }
            
            let headers = &data[0];
            let col_idx = headers.iter().position(|h| h == &col_name);
            
            match col_idx {
                Some(idx) => {
                    let mut values = vec![];
                    for row in data.iter().skip(1) {
                        if idx < row.len() {
                            if let Ok(num) = row[idx].parse::<f64>() {
                                values.push(num);
                            }
                        }
                    }
                    
                    if values.is_empty() {
                        return vec![];
                    }
                    
                    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
                    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    let sum: f64 = values.iter().sum();
                    let count = values.len() as f64;
                    let mean = sum / count;
                    
                    vec![min, max, mean, count, sum]
                }
                None => vec![],
            }
        }
        
        pub fn min(data: &Vec<Vec<String>>, col_name: String) -> f64 {
            let s = stats(data, col_name);
            if s.is_empty() { 0.0 } else { s[0] }
        }

        pub fn max(data: &Vec<Vec<String>>, col_name: String) -> f64 {
            let s = stats(data, col_name);
            if s.is_empty() { 0.0 } else { s[1] }
        }

        pub fn mean(data: &Vec<Vec<String>>, col_name: String) -> f64 {
            let s = stats(data, col_name);
            if s.is_empty() { 0.0 } else { s[2] }
        }

        pub fn row_count(data: &Vec<Vec<String>>) -> i64 {
            if data.is_empty() { 0 } else { (data.len() - 1) as i64 }
        }

        pub fn column_count(data: &Vec<Vec<String>>) -> i64 {
            if data.is_empty() { 0 } else { data[0].len() as i64 }
        }

        pub fn select(data: &Vec<Vec<String>>, col_names: Vec<String>) -> Vec<Vec<String>> {
            if data.is_empty() {
                return vec![];
            }
            
            let headers = &data[0];
            let col_indices: Vec<usize> = col_names.iter()
                .filter_map(|name| headers.iter().position(|h| h == name))
                .collect();
            
            if col_indices.is_empty() {
                return vec![];
            }
            
            let mut result = vec![];
            for row in data {
                let selected: Vec<String> = col_indices.iter()
                    .map(|&idx| {
                        if idx < row.len() {
                            row[idx].clone()
                        } else {
                            String::new()
                        }
                    })
                    .collect();
                result.push(selected);
            }
            result
        }

        pub fn remove_duplicates(data: &Vec<Vec<String>>) -> Vec<Vec<String>> {
            if data.is_empty() {
                return vec![];
            }
            
            let mut result = vec![data[0].clone()];
            let mut seen = HashSet::new();
            
            for row in data.iter().skip(1) {
                // Simplistic row serialization for deduplication
                let row_str = row.join(","); 
                if !seen.contains(&row_str) {
                    seen.insert(row_str);
                    result.push(row.clone());
                }
            }
            result
        }

        pub fn add_row(data: &Vec<Vec<String>>, row: Vec<String>) -> Vec<Vec<String>> {
            let mut new_data = data.clone();
            new_data.push(row);
            new_data
        }
    }

    pub mod http {
        use std::collections::HashMap;

        pub fn get(url: String) -> Vec<String> {
             match reqwest::blocking::get(&url) {
                Ok(resp) => {
                    let status = resp.status().as_u16().to_string();
                    let ok = resp.status().is_success().to_string();
                    let body = resp.text().unwrap_or_default();
                    vec![status, ok, body]
                }
                Err(_) => vec!["0".to_string(), "false".to_string(), "".to_string()],
            }
        }

        pub fn post(url: String, body: String) -> Vec<String> {
            let client = reqwest::blocking::Client::new();
            match client.post(&url).body(body).send() {
                Ok(resp) => {
                    let status = resp.status().as_u16().to_string();
                    let ok = resp.status().is_success().to_string();
                    let body = resp.text().unwrap_or_default();
                    vec![status, ok, body]
                }
                Err(_) => vec!["0".to_string(), "false".to_string(), "".to_string()],
            }
        }
        
        pub fn download(url: &String, path: &String) -> bool {
             match reqwest::blocking::get(url) {
                Ok(mut resp) => {
                    if let Ok(mut file) = std::fs::File::create(path) {
                        resp.copy_to(&mut file).is_ok()
                    } else {
                        false
                    }
                }
                Err(_) => false,
            }
        }

        pub fn request(method: String, url: String, headers: String, body: String) -> Vec<String> {
            let client = reqwest::blocking::Client::new();
            let req_method = match method.to_uppercase().as_str() {
                "GET" => reqwest::Method::GET,
                "POST" => reqwest::Method::POST,
                "PUT" => reqwest::Method::PUT,
                "DELETE" => reqwest::Method::DELETE,
                _ => return vec!["0".to_string(), "false".to_string(), "Unsupported method".to_string()],
            };
            
            let mut req = client.request(req_method, &url);
            
            if !headers.is_empty() {
                 if let Ok(json) = serde_json::from_str::<std::collections::HashMap<String, String>>(&headers) {
                     for (k, v) in json {
                         req = req.header(k, v);
                     }
                 }
            }
            
            req = req.body(body);
            
            match req.send() {
                Ok(resp) => {
                    let status = resp.status().as_u16().to_string();
                    let ok = resp.status().is_success().to_string();
                    let body = resp.text().unwrap_or_default();
                    vec![status, ok, body]
                }
                Err(_) => vec!["0".to_string(), "false".to_string(), "".to_string()],
            }
        }
    }

    pub mod markdown {
        pub fn parse(text: String) -> String {
            let options = comrak::ComrakOptions::default();
            comrak::markdown_to_html(&text, &options)
        }
    }

    pub mod excel {
        use calamine::{Reader, Xlsx, open_workbook};
        use rust_xlsxwriter::{Workbook, XlsxError};

        pub fn read(path: String) -> Vec<Vec<String>> {
             use calamine::{Reader, Xlsx, open_workbook, Data as ExcelData};
             let mut workbook: Xlsx<_> = match open_workbook(&path) {
                Ok(wb) => wb,
                Err(_) => return vec![],
            };
            
            if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
                 range.rows()
                     .map(|row| {
                         row.iter()
                             .map(|cell| match cell {
                                 ExcelData::String(s) => s.to_string(),
                                 ExcelData::Float(f) => f.to_string(),
                                 ExcelData::Int(i) => i.to_string(),
                                 ExcelData::Bool(b) => b.to_string(),
                                 ExcelData::Empty => "".to_string(),
                                 ExcelData::DateTime(d) => d.to_string(),
                                 ExcelData::Error(e) => format!("{:?}", e),
                                 ExcelData::DateTimeIso(d) => d.clone(),
                                 ExcelData::DurationIso(d) => d.clone(),
                             })
                             .collect()
                     })
                     .collect()
            } else {
                vec![]
            }
        }

        pub fn write(path: String, data: &Vec<Vec<String>>) -> bool {
            let mut workbook = Workbook::new();
            let worksheet = workbook.add_worksheet();
            
            for (row_idx, row_data) in data.iter().enumerate() {
                for (col_idx, cell_data) in row_data.iter().enumerate() {
                    let _ = worksheet.write_string(row_idx as u32, col_idx as u16, cell_data);
                }
            }
            
            workbook.save(&path).is_ok()
        }
    }

    pub mod graphics {
        use std::sync::Mutex;
        
        #[derive(Clone, Debug)]
        struct ChartConfig {
            title: String,
            series: Vec<Series>,
        }

        #[derive(Clone, Debug)]
        enum Series {
            Line { x: Vec<f64>, y: Vec<f64>, label: String, color: String },
            Scatter { x: Vec<f64>, y: Vec<f64>, label: String, color: String },
            Bar { labels: Vec<String>, values: Vec<f64>, label: String, color: String },
        }

        impl ChartConfig {
            fn new() -> Self {
                ChartConfig {
                    title: "Chart".to_string(),
                    series: Vec::new(),
                }
            }
        }

        static mut CHART_CONFIG: Option<ChartConfig> = None;

        unsafe fn get_config() -> &'static mut ChartConfig {
            if CHART_CONFIG.is_none() {
                CHART_CONFIG = Some(ChartConfig::new());
            }
            CHART_CONFIG.as_mut().unwrap()
        }

        pub fn title(t: String) -> bool {
            unsafe {
                get_config().title = t;
            }
            true
        }

        pub fn plot(x: Vec<f64>, y: Vec<f64>, label: String, color: String) -> bool {
            unsafe {
                get_config().series.push(Series::Line { x, y, label, color });
            }
            true
        }

        pub fn scatter(x: Vec<f64>, y: Vec<f64>, label: String, color: String) -> bool {
            unsafe {
                get_config().series.push(Series::Scatter { x, y, label, color });
            }
            true
        }

        pub fn bar(labels: Vec<String>, values: Vec<f64>, label: String, color: String) -> bool {
             unsafe {
                get_config().series.push(Series::Bar { labels, values, label, color });
            }
            true
        }

        pub fn save(filename: String) -> bool {
            use plotters::prelude::*;
            
            unsafe {
                let config = get_config();
                let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
                if let Err(_) = root.fill(&WHITE) { return false; }

                let mut x_min = f64::INFINITY;
                let mut x_max = f64::NEG_INFINITY;
                let mut y_min = f64::INFINITY;
                let mut y_max = f64::NEG_INFINITY;
                
                for s in &config.series {
                    match s {
                        Series::Line { x, y, .. } | Series::Scatter { x, y, .. } => {
                            for val in x { if *val < x_min { x_min = *val; } if *val > x_max { x_max = *val; } }
                            for val in y { if *val < y_min { y_min = *val; } if *val > y_max { y_max = *val; } }
                        },
                        Series::Bar { values, .. } => {
                            x_min = 0.0;
                            x_max = values.len() as f64;
                            y_min = 0.0;
                            for val in values { if *val > y_max { y_max = *val; } }
                        }
                    }
                }
                
                if x_min == f64::INFINITY { x_min = 0.0; x_max = 10.0; }
                if y_min == f64::INFINITY { y_min = 0.0; y_max = 10.0; }
                
                let x_range = x_max - x_min;
                let y_range = y_max - y_min;
                x_min -= x_range * 0.1;
                x_max += x_range * 0.1;
                y_min -= y_range * 0.1;
                y_max += y_range * 0.1;

                let mut chart_res = ChartBuilder::on(&root)
                    //.caption(&config.title, ("sans-serif", 40).into_font())
                    .margin(5)
                    //.x_label_area_size(30)
                    //.y_label_area_size(30)
                    .build_cartesian_2d(x_min..x_max, y_min..y_max);
                    
                if let Ok(mut chart) = chart_res {
                    let _ = chart.configure_mesh().draw();

                    for s in &config.series {
                        let color_ref = match s {
                            Series::Line { color, .. } => color,
                            Series::Scatter { color, .. } => color,
                            Series::Bar { color, .. } => color,
                        };
                        
                        let plot_color = match color_ref.as_str() {
                            "red" => RED,
                            "blue" => BLUE,
                            "green" => GREEN,
                            "yellow" => YELLOW,
                            "black" => BLACK,
                            "cyan" => CYAN,
                            "magenta" => MAGENTA,
                            _ => BLUE,
                        };

                        match s {
                            Series::Line { x, y, label, .. } => {
                                let points: Vec<(f64, f64)> = x.iter().zip(y.iter()).map(|(a, b)| (*a, *b)).collect();
                                let _ = chart.draw_series(LineSeries::new(points, plot_color.stroke_width(2)));
                                    //.map(|s| s.label(label.clone())
                                    //.legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], plot_color.filled())));
                            },
                            Series::Scatter { x, y, label, .. } => {
                                let points: Vec<(f64, f64)> = x.iter().zip(y.iter()).map(|(a, b)| (*a, *b)).collect();
                                let _ = chart.draw_series(PointSeries::of_element(
                                    points,
                                    5,
                                    plot_color.filled(),
                                    &|c, s, st| {
                                        return EmptyElement::at(c) + Circle::new((0,0), s, st.filled());
                                    },
                                ));
                                //.map(|s| s.label(label.clone())
                                //.legend(move |(x, y)| Circle::new((x + 10, y), 5, plot_color.filled())));
                            },
                            Series::Bar { values, label, .. } => {
                                let bars: Vec<(f64, f64)> = values.iter().enumerate().map(|(i, v)| (i as f64, *v)).collect();
                                let _ = chart.draw_series(
                                    bars.iter().map(|(x, y)| {
                                        Rectangle::new([(*x - 0.4, 0.0), (*x + 0.4, *y)], plot_color.filled())
                                    })
                                );
                                //.map(|s| s.label(label.clone())
                                //.legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], plot_color.filled())));
                            }
                        }
                    }
                    
                    /*let _ = chart.configure_series_labels()
                        .background_style(&WHITE.mix(0.8))
                        .border_style(&BLACK)
                        .draw();*/
                }
                
                // Reset config
                CHART_CONFIG = Some(ChartConfig::new());
                true
            }
        }
    }
    pub mod compute {
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

pub fn log(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    a.iter().map(|row| row.iter().map(|x| x.ln()).collect()).collect()
}

pub fn sum(a: &Vec<Vec<f64>>) -> f64 {
    a.iter().map(|row| row.iter().sum::<f64>()).sum()
}

pub fn max(a: &Vec<Vec<f64>>) -> f64 {
    a.iter().flatten().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
}

pub fn argmax(a: &Vec<Vec<f64>>) -> f64 {
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
    }
}
"#;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ElementWiseUniforms {
    size: u32,
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
    }
}
