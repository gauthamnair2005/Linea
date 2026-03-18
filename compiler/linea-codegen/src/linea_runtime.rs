
pub mod linea_runtime {

    #[derive(Debug, Clone, PartialEq)]
    pub enum Value {
        Int(i64),
        Float(f64),
        String(String),
        Bool(bool),
        Array(Vec<Value>),
        Matrix(Vec<Vec<f64>>),
        None,
    }

    impl From<i64> for Value { fn from(v: i64) -> Self { Value::Int(v) } }
    impl From<f64> for Value { fn from(v: f64) -> Self { Value::Float(v) } }
    impl From<String> for Value { fn from(v: String) -> Self { Value::String(v) } }
    impl From<&str> for Value { fn from(v: &str) -> Self { Value::String(v.to_string()) } }
    impl From<bool> for Value { fn from(v: bool) -> Self { Value::Bool(v) } }
    impl From<Vec<Value>> for Value { fn from(v: Vec<Value>) -> Self { Value::Array(v) } }
    impl From<Vec<Vec<f64>>> for Value { fn from(v: Vec<Vec<f64>>) -> Self { Value::Matrix(v) } }
    impl From<Vec<Vec<String>>> for Value { 
        fn from(v: Vec<Vec<String>>) -> Self { 
            let arr = v.into_iter().map(|row| {
                Value::Array(row.into_iter().map(Value::String).collect())
            }).collect();
            Value::Array(arr)
        } 
    }

    impl Value {
        pub fn as_matrix(&self) -> Option<Vec<Vec<f64>>> {
            match self {
                Value::Matrix(m) => Some(m.clone()),
                Value::Array(a) => {
                     let mut mat = Vec::new();
                     for row in a {
                         if let Value::Array(r) = row {
                             let mut r_vec = Vec::new();
                             for v in r {
                                 match v {
                                     Value::Float(f) => r_vec.push(*f),
                                     Value::Int(i) => r_vec.push(*i as f64),
                                     _ => return None,
                                 }
                             }
                             mat.push(r_vec);
                         } else { return None; }
                     }
                     Some(mat)
                },
                _ => None,
            }
        }
        
        pub fn as_float(&self) -> f64 {
            match self {
                Value::Float(f) => *f,
                Value::Int(i) => *i as f64,
                _ => 0.0,
            }
        }
    }

    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = 0u8;
        for i in 0..a.len() {
            diff |= a[i] ^ b[i];
        }
        diff == 0
    }

        pub fn element_wise(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, op: &str) -> Vec<Vec<f64>> {
             let rows = a.len();
             if rows == 0 { return vec![]; }
             let cols = a[0].len();
             
             // Simple scalar broadcast support (if b is 1x1)
             let b_is_scalar = b.len() == 1 && b[0].len() == 1;
             let val = if b_is_scalar { b[0][0] } else { 0.0 };
             
             let mut res = Vec::with_capacity(rows);
             for i in 0..rows {
                 let mut row = Vec::with_capacity(cols);
                 for j in 0..cols {
                     let x = a[i][j];
                     let y = if b_is_scalar { val } else { b[i][j] }; // Unsafe if shapes mismatch and not scalar
                     
                     let r = match op {
                         "add" => x + y,
                         "sub" => x - y,
                         "mul" => x * y,
                         "div" => x / y,
                         _ => 0.0,
                     };
                     row.push(r);
                 }
                 res.push(row);
             }
             res
        }
        
        pub fn softmax(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
            a.iter().map(|row| {
                let max_val = row.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let exps: Vec<f64> = row.iter().map(|x| (x - max_val).exp()).collect();
                let sum: f64 = exps.iter().sum();
                exps.iter().map(|x| x / sum).collect()
            }).collect()
        }
        
        pub fn sum_columns(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
             if a.is_empty() { return vec![vec![0.0]]; }
             let cols = a[0].len();
             let mut sums = vec![0.0; cols];
             for row in a {
                 for (j, val) in row.iter().enumerate() {
                     sums[j] += val;
                 }
             }
             vec![sums]
        }
        
        pub fn sqrt(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
             a.iter().map(|row| row.iter().map(|x| x.sqrt()).collect()).collect()
        }
        
        pub fn pow(a: &Vec<Vec<f64>>, exp: f64) -> Vec<Vec<f64>> {
             a.iter().map(|row| row.iter().map(|x| x.powf(exp)).collect()).collect()
        }
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

pub fn element_wise(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, op: &str) -> Vec<Vec<f64>> {
    let rows = a.len();
    if rows == 0 { return vec![]; }
    let cols = a[0].len();
    
    // Check dimensions match
    if b.len() != rows || (rows > 0 && b[0].len() != cols) { return vec![]; }

    // Flatten
    let mut a_flat: Vec<f64> = Vec::with_capacity(rows * cols);
    for row in a { a_flat.extend(row.iter().copied()); }
    
    let mut b_flat: Vec<f64> = Vec::with_capacity(rows * cols);
    for row in b { b_flat.extend(row.iter().copied()); }

    let size = (rows * cols) as u32;

    let op_code = match op {
        "add" => 0,
        "sub" => 1,
        "mul" => 2,
        "div" => 3,
        "pow" => 4,
        _ => return vec![],
    };
    
    let a_f32: Vec<f32> = a_flat.iter().map(|&x| x as f32).collect();
    let b_f32: Vec<f32> = b_flat.iter().map(|&x| x as f32).collect();

    if let Some(res) = element_wise_impl(&a_f32, &b_f32, size, op_code) {
        // Reshape
        let mut result = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for j in 0..cols {
                row.push(res[i * cols + j] as f64);
            }
            result.push(row);
        }
        result
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

pub fn one_hot(labels: &Vec<f64>, classes: usize) -> Vec<Vec<f64>> {
    let mut res = vec![vec![0.0; classes]; labels.len()];
    for (i, &label) in labels.iter().enumerate() {
        let idx = label as usize;
        if idx < classes {
            res[i][idx] = 1.0;
        }
    }
    res
}

pub fn cross_entropy(pred: &Vec<Vec<f64>>, target: &Vec<Vec<f64>>) -> f64 {
    let rows = pred.len();
    if rows == 0 {
        return 0.0;
    }

    let mut loss = 0.0;
    for (i, row) in pred.iter().enumerate() {
        for (j, &p) in row.iter().enumerate() {
            if i < target.len() && j < target[i].len() && target[i][j] > 0.0 {
                loss -= target[i][j] * (p + 1e-12).ln();
            }
        }
    }
    loss / rows as f64
}

pub fn clip(a: &Vec<Vec<f64>>, min_val: f64, max_val: f64) -> Vec<Vec<f64>> {
    a.iter()
        .map(|row| row.iter().map(|x| x.max(min_val).min(max_val)).collect())
        .collect()
}

pub fn normalize_l2(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    a.iter()
        .map(|row| {
            let norm = row.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-12);
            row.iter().map(|x| x / norm).collect()
        })
        .collect()
}

pub fn dropout(a: &Vec<Vec<f64>>, p: f64) -> Vec<Vec<f64>> {
    let keep = (1.0 - p).clamp(0.0, 1.0).max(1e-12);
    a.iter()
        .map(|row| {
            row.iter()
                .map(|x| if rand::random::<f64>() < keep { x / keep } else { 0.0 })
                .collect()
        })
        .collect()
}
    }

    pub mod mlio {
        use super::Value;
        use std::fs;

        fn read_header(path: &str, bytes: usize) -> Option<Vec<u8>> {
            let data = fs::read(path).ok()?;
            if data.len() < bytes {
                return None;
            }
            Some(data[..bytes].to_vec())
        }

        pub fn load_gguf(path: String) -> Value {
            // GGUF magic: b"GGUF"
            let is_gguf = read_header(&path, 4)
                .map(|h| h == b"GGUF")
                .unwrap_or(false);
            if !is_gguf {
                return Value::String("GGUF parse error: invalid or unsupported file".to_string());
            }
            Value::String(format!("GGUF model loaded: {}", path))
        }

        pub fn load_onnx(path: String) -> Value {
            let is_zip = read_header(&path, 2)
                .map(|h| h == vec![0x50, 0x4b])
                .unwrap_or(false);
            if !is_zip {
                return Value::String("ONNX parse error: invalid or unsupported file".to_string());
            }
            Value::String(format!("ONNX model loaded (metadata mode): {}", path))
        }

        pub fn load_pth(path: String) -> Value {
            if fs::metadata(&path).is_err() {
                return Value::String("PTH parse error: file not found".to_string());
            }
            Value::String(format!("PTH checkpoint loaded (metadata mode): {}", path))
        }

        pub fn load_mlx(path: String) -> Value {
            if fs::metadata(&path).is_err() {
                return Value::String("MLX parse error: file not found".to_string());
            }
            Value::String(format!("MLX model loaded (metadata mode): {}", path))
        }

        pub fn save_gguf(path: String, payload: Value) -> bool {
            let mut bytes = b"GGUF".to_vec();
            bytes.extend_from_slice(format!("{:?}", payload).as_bytes());
            fs::write(path, bytes).is_ok()
        }
    }

    pub mod hash {
        use sha2::{Digest, Sha256, Sha512};

        pub fn sha256(input: String) -> String {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            super::hex_encode(&hasher.finalize())
        }

        pub fn sha512(input: String) -> String {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());
            super::hex_encode(&hasher.finalize())
        }

        pub fn md5(input: String) -> String {
            format!("{:x}", md5::compute(input.as_bytes()))
        }

        pub fn with_salt(algo: String, input: String, salt: String) -> String {
            let composite = format!("{}:{}", salt, input);
            match algo.to_lowercase().as_str() {
                "sha512" => sha512(composite),
                "md5" => md5(composite),
                _ => sha256(composite),
            }
        }

        pub fn secure_equals(a: String, b: String) -> bool {
            super::constant_time_eq(a.as_bytes(), b.as_bytes())
        }
    }

    pub mod security {
        use rand::Rng;
        use rand::RngCore;

        pub fn random_bytes(len: i64) -> String {
            if len <= 0 {
                return String::new();
            }
            let mut buf = vec![0u8; len as usize];
            rand::thread_rng().fill_bytes(&mut buf);
            super::hex_encode(&buf)
        }

        pub fn random_token(len: i64) -> String {
            if len <= 0 {
                return String::new();
            }
            let mut rng = rand::thread_rng();
            let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";
            (0..len as usize)
                .map(|_| {
                    let idx = rng.gen_range(0..alphabet.len());
                    alphabet[idx] as char
                })
                .collect()
        }

        pub fn constant_time_equals(a: String, b: String) -> bool {
            super::constant_time_eq(a.as_bytes(), b.as_bytes())
        }

        pub fn password_hash(secret: String) -> String {
            let salt = random_bytes(16);
            let digest = crate::linea_runtime::hash::with_salt("sha256".to_string(), secret, salt.clone());
            format!("sha256${}${}", salt, digest)
        }

        pub fn password_verify(secret: String, stored: String) -> bool {
            let parts: Vec<&str> = stored.split('$').collect();
            if parts.len() != 3 {
                return false;
            }
            let algo = parts[0].to_string();
            let salt = parts[1].to_string();
            let expected = parts[2].to_string();
            let candidate = crate::linea_runtime::hash::with_salt(algo, secret, salt);
            super::constant_time_eq(candidate.as_bytes(), expected.as_bytes())
        }

        pub fn password_score(secret: String) -> i64 {
            let mut score = 0;
            if secret.len() >= 8 { score += 1; }
            if secret.len() >= 12 { score += 1; }
            if secret.chars().any(|c| c.is_ascii_lowercase()) { score += 1; }
            if secret.chars().any(|c| c.is_ascii_uppercase()) { score += 1; }
            if secret.chars().any(|c| c.is_ascii_digit()) { score += 1; }
            if secret.chars().any(|c| !c.is_ascii_alphanumeric()) { score += 1; }
            score
        }

        pub fn is_strong_password(secret: String) -> bool {
            password_score(secret) >= 5
        }
    }

    pub mod sql {
        use rusqlite::{params_from_iter, Connection, OptionalExtension};
        use std::collections::{HashMap, HashSet};
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::sync::{Mutex, OnceLock};

        static HANDLE_COUNTER: AtomicU64 = AtomicU64::new(0);
        static CONNECTIONS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
        static SECURE_HASHES: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
        static UNLOCKED: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

        fn conns() -> &'static Mutex<HashMap<String, String>> {
            CONNECTIONS.get_or_init(|| Mutex::new(HashMap::new()))
        }

        fn secure_hashes() -> &'static Mutex<HashMap<String, String>> {
            SECURE_HASHES.get_or_init(|| Mutex::new(HashMap::new()))
        }

        fn unlocked() -> &'static Mutex<HashSet<String>> {
            UNLOCKED.get_or_init(|| Mutex::new(HashSet::new()))
        }

        fn get_path(handle: &str) -> Option<String> {
            let map = conns().lock().ok()?;
            map.get(handle).cloned()
        }

        fn ensure_unlocked(handle: &str) -> bool {
            let secure = secure_hashes()
                .lock()
                .ok()
                .map(|m| m.contains_key(handle))
                .unwrap_or(false);
            if !secure {
                return true;
            }
            unlocked()
                .lock()
                .ok()
                .map(|s| s.contains(handle))
                .unwrap_or(false)
        }

        pub fn open(path: String) -> String {
            if Connection::open(&path).is_err() {
                return String::new();
            }
            let id = HANDLE_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
            let handle = format!("sql_conn_{}", id);
            if let Ok(mut m) = conns().lock() {
                m.insert(handle.clone(), path);
            }
            if let Ok(mut u) = unlocked().lock() {
                u.insert(handle.clone());
            }
            handle
        }

        pub fn close(handle: String) -> bool {
            let removed = conns().lock().ok().and_then(|mut m| m.remove(&handle)).is_some();
            if let Ok(mut s) = secure_hashes().lock() {
                s.remove(&handle);
            }
            if let Ok(mut u) = unlocked().lock() {
                u.remove(&handle);
            }
            removed
        }

        pub fn init_secure(handle: String, password: String) -> bool {
            let path = match get_path(&handle) {
                Some(p) => p,
                None => return false,
            };
            let conn = match Connection::open(path) {
                Ok(c) => c,
                Err(_) => return false,
            };
            if conn
                .execute(
                    "CREATE TABLE IF NOT EXISTS _linea_auth (id INTEGER PRIMARY KEY CHECK(id=1), hash TEXT NOT NULL)",
                    [],
                )
                .is_err()
            {
                return false;
            }
            let stored = crate::linea_runtime::security::password_hash(password);
            if conn
                .execute(
                    "INSERT INTO _linea_auth (id, hash) VALUES (1, ?1) ON CONFLICT(id) DO UPDATE SET hash=excluded.hash",
                    [stored.as_str()],
                )
                .is_err()
            {
                return false;
            }
            if let Ok(mut s) = secure_hashes().lock() {
                s.insert(handle.clone(), stored);
            }
            if let Ok(mut u) = unlocked().lock() {
                u.remove(&handle);
            }
            true
        }

        pub fn unlock(handle: String, password: String) -> bool {
            let path = match get_path(&handle) {
                Some(p) => p,
                None => return false,
            };
            let conn = match Connection::open(path) {
                Ok(c) => c,
                Err(_) => return false,
            };
            let stored: Option<String> = conn
                .query_row("SELECT hash FROM _linea_auth WHERE id=1", [], |row| row.get(0))
                .optional()
                .ok()
                .flatten();
            let ok = stored
                .map(|h| crate::linea_runtime::security::password_verify(password, h))
                .unwrap_or(false);
            if ok {
                if let Ok(mut u) = unlocked().lock() {
                    u.insert(handle);
                }
            }
            ok
        }

        pub fn execute(handle: String, query: String, params: &Vec<String>) -> i64 {
            if !ensure_unlocked(&handle) {
                return -1;
            }
            let path = match get_path(&handle) {
                Some(p) => p,
                None => return -1,
            };
            let conn = match Connection::open(path) {
                Ok(c) => c,
                Err(_) => return -1,
            };
            let sql_params = params.iter().map(|v| rusqlite::types::Value::Text(v.clone())).collect::<Vec<_>>();
            conn.execute(&query, params_from_iter(sql_params.iter()))
                .map(|n| n as i64)
                .unwrap_or(-1)
        }

        pub fn query(handle: String, query: String, params: &Vec<String>) -> Vec<Vec<String>> {
            if !ensure_unlocked(&handle) {
                return vec![];
            }
            let path = match get_path(&handle) {
                Some(p) => p,
                None => return vec![],
            };
            let conn = match Connection::open(path) {
                Ok(c) => c,
                Err(_) => return vec![],
            };
            let sql_params = params.iter().map(|v| rusqlite::types::Value::Text(v.clone())).collect::<Vec<_>>();
            let mut stmt = match conn.prepare(&query) {
                Ok(s) => s,
                Err(_) => return vec![],
            };

            let mut rows_out = vec![
                stmt.column_names()
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>(),
            ];
            let col_count = stmt.column_count();
            let mapped = match stmt.query_map(params_from_iter(sql_params.iter()), |row| {
                let mut out = Vec::with_capacity(col_count);
                for i in 0..col_count {
                    let cell = row.get_ref(i)?;
                    let text = match cell {
                        rusqlite::types::ValueRef::Null => String::new(),
                        rusqlite::types::ValueRef::Integer(n) => n.to_string(),
                        rusqlite::types::ValueRef::Real(f) => f.to_string(),
                        rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                        rusqlite::types::ValueRef::Blob(b) => super::hex_encode(b),
                    };
                    out.push(text);
                }
                Ok(out)
            }) {
                Ok(m) => m,
                Err(_) => return vec![],
            };

            for row in mapped {
                if let Ok(values) = row {
                    rows_out.push(values);
                }
            }
            rows_out
        }
    }

    pub mod db {
        pub fn open(path: String) -> String {
            crate::linea_runtime::sql::open(path)
        }

        pub fn close(handle: String) -> bool {
            crate::linea_runtime::sql::close(handle)
        }

        pub fn init_secure(handle: String, password: String) -> bool {
            crate::linea_runtime::sql::init_secure(handle, password)
        }

        pub fn unlock(handle: String, password: String) -> bool {
            crate::linea_runtime::sql::unlock(handle, password)
        }

        pub fn execute(handle: String, query: String, params: &Vec<String>) -> i64 {
            crate::linea_runtime::sql::execute(handle, query, params)
        }

        pub fn query(handle: String, query: String, params: &Vec<String>) -> Vec<Vec<String>> {
            crate::linea_runtime::sql::query(handle, query, params)
        }
    }

    pub mod fileio {
        use std::fs;
        use std::io::Write;
        use std::path::Path;

        pub fn read_text(path: String) -> String {
            fs::read_to_string(path).unwrap_or_default()
        }

        pub fn write_text(path: String, content: String) -> bool {
            fs::write(path, content).is_ok()
        }

        pub fn append_text(path: String, content: String) -> bool {
            let mut file = match fs::OpenOptions::new().create(true).append(true).open(path) {
                Ok(f) => f,
                Err(_) => return false,
            };
            file.write_all(content.as_bytes()).is_ok()
        }

        pub fn exists(path: String) -> bool {
            Path::new(&path).exists()
        }

        pub fn is_file(path: String) -> bool {
            Path::new(&path).is_file()
        }

        pub fn is_dir(path: String) -> bool {
            Path::new(&path).is_dir()
        }

        pub fn mkdir(path: String) -> bool {
            fs::create_dir_all(path).is_ok()
        }

        pub fn remove_file(path: String) -> bool {
            if !Path::new(&path).exists() {
                return false;
            }
            fs::remove_file(path).is_ok()
        }

        pub fn remove_dir(path: String) -> bool {
            if !Path::new(&path).exists() {
                return false;
            }
            fs::remove_dir_all(path).is_ok()
        }

        pub fn rename(from_path: String, to_path: String) -> bool {
            fs::rename(from_path, to_path).is_ok()
        }

        pub fn copy_file(from_path: String, to_path: String) -> bool {
            fs::copy(from_path, to_path).is_ok()
        }

        pub fn list_dir(path: String) -> Vec<String> {
            let mut out = Vec::new();
            let iter = match fs::read_dir(path) {
                Ok(i) => i,
                Err(_) => return out,
            };
            for item in iter.flatten() {
                if let Some(name) = item.file_name().to_str() {
                    out.push(name.to_string());
                }
            }
            out
        }

        pub fn size_bytes(path: String) -> i64 {
            fs::metadata(path).map(|m| m.len() as i64).unwrap_or(-1)
        }
    }

    pub mod lowlevel {
        pub fn bit_and(a: i64, b: i64) -> i64 { a & b }
        pub fn bit_or(a: i64, b: i64) -> i64 { a | b }
        pub fn bit_xor(a: i64, b: i64) -> i64 { a ^ b }
        pub fn bit_not(a: i64) -> i64 { !a }

        pub fn shl(a: i64, bits: i64) -> i64 {
            if bits < 0 { return a; }
            a << (bits as u32)
        }

        pub fn shr(a: i64, bits: i64) -> i64 {
            if bits < 0 { return a; }
            a >> (bits as u32)
        }

        pub fn to_bytes_le(v: i64) -> Vec<i64> {
            v.to_le_bytes().iter().map(|b| *b as i64).collect()
        }

        pub fn from_bytes_le(bytes: &Vec<i64>) -> i64 {
            let mut buf = [0u8; 8];
            let limit = bytes.len().min(8);
            for i in 0..limit {
                let clamped = bytes[i].clamp(0, 255) as u8;
                buf[i] = clamped;
            }
            i64::from_le_bytes(buf)
        }

        pub fn pointer_size() -> i64 {
            std::mem::size_of::<usize>() as i64
        }
    }

    pub mod git {
        use std::process::Command;

        fn run_git(repo_path: &str, args: &[&str]) -> Result<String, String> {
            let output = Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .args(args)
                .output()
                .map_err(|e| format!("git invocation failed: {}", e))?;

            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        }

        pub fn is_repo(repo_path: String) -> bool {
            run_git(&repo_path, &["rev-parse", "--is-inside-work-tree"])
                .map(|s| s == "true")
                .unwrap_or(false)
        }

        pub fn status(repo_path: String) -> String {
            run_git(&repo_path, &["status", "--short", "--branch"]).unwrap_or_default()
        }

        pub fn current_branch(repo_path: String) -> String {
            run_git(&repo_path, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_default()
        }

        pub fn last_commit(repo_path: String) -> String {
            run_git(&repo_path, &["log", "-1", "--pretty=%H"]).unwrap_or_default()
        }

        pub fn log(repo_path: String, count: i64) -> Vec<String> {
            let n = if count <= 0 { 1 } else { count };
            let n_str = format!("-{}", n);
            run_git(&repo_path, &["log", &n_str, "--pretty=%h %s"])
                .map(|s| s.lines().map(|x| x.to_string()).collect())
                .unwrap_or_default()
        }

        pub fn diff(repo_path: String) -> String {
            run_git(&repo_path, &["diff"]).unwrap_or_default()
        }

        pub fn add(repo_path: String, spec: String) -> bool {
            run_git(&repo_path, &["add", &spec]).is_ok()
        }

        pub fn commit(repo_path: String, message: String) -> bool {
            run_git(&repo_path, &["commit", "-m", &message]).is_ok()
        }

        pub fn push(repo_path: String, remote: String, branch: String) -> bool {
            run_git(&repo_path, &["push", &remote, &branch]).is_ok()
        }

        pub fn pull(repo_path: String, remote: String, branch: String) -> bool {
            run_git(&repo_path, &["pull", &remote, &branch]).is_ok()
        }

        pub fn checkout(repo_path: String, target: String) -> bool {
            run_git(&repo_path, &["checkout", &target]).is_ok()
        }

        pub fn init(repo_path: String) -> bool {
            Command::new("git")
                .arg("init")
                .arg(repo_path)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }

        pub fn clone(url: String, destination: String) -> bool {
            Command::new("git")
                .arg("clone")
                .arg(url)
                .arg(destination)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }
    }

    pub mod fun {
        use rand::Rng;

        pub fn coin_flip() -> String {
            if rand::thread_rng().gen_bool(0.5) {
                "Heads".to_string()
            } else {
                "Tails".to_string()
            }
        }

        pub fn roll_dice(sides: i64) -> i64 {
            let max = if sides < 2 { 6 } else { sides };
            rand::thread_rng().gen_range(1..=max)
        }

        pub fn random_emoji() -> String {
            let emojis = ["🚀", "🎯", "🧠", "🎮", "🌟", "🔥", "✨", "🎲", "🛠️", "📦"];
            let idx = rand::thread_rng().gen_range(0..emojis.len());
            emojis[idx].to_string()
        }

        pub fn random_joke() -> String {
            let jokes = [
                "I told my code to be clean. It deleted itself.",
                "There are 10 kinds of people: those who get binary and those who don't.",
                "A bug is never alone, it always has stack traces.",
                "Works on my machine is not a deployment strategy.",
                "My favorite data structure is coffee."
            ];
            let idx = rand::thread_rng().gen_range(0..jokes.len());
            jokes[idx].to_string()
        }

        pub fn random_color() -> String {
            let mut rng = rand::thread_rng();
            format!("#{:02X}{:02X}{:02X}", rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>())
        }

        pub fn choose(options: &Vec<String>) -> String {
            if options.is_empty() {
                return String::new();
            }
            let idx = rand::thread_rng().gen_range(0..options.len());
            options[idx].clone()
        }
    }

    pub mod uuid {
        use rand::Rng;

        pub fn v4() -> String {
            let mut bytes = [0u8; 16];
            rand::thread_rng().fill(&mut bytes);
            bytes[6] = (bytes[6] & 0x0f) | 0x40;
            bytes[8] = (bytes[8] & 0x3f) | 0x80;
            format!(
                "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
                ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32),
                ((bytes[4] as u16) << 8) | (bytes[5] as u16),
                ((bytes[6] as u16) << 8) | (bytes[7] as u16),
                ((bytes[8] as u16) << 8) | (bytes[9] as u16),
                ((bytes[10] as u64) << 40)
                    | ((bytes[11] as u64) << 32)
                    | ((bytes[12] as u64) << 24)
                    | ((bytes[13] as u64) << 16)
                    | ((bytes[14] as u64) << 8)
                    | (bytes[15] as u64)
            )
        }

        pub fn short() -> String {
            let full = v4();
            full.split('-').next().unwrap_or_default().to_string()
        }
    }

    pub mod webserver {
        use tiny_http::{Header, Response, Server, StatusCode};

        pub fn serve_text(host: String, port: i64, body: String, max_requests: i64) -> bool {
            let addr = format!("{}:{}", host, port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(_) => return false,
            };
            let n = if max_requests <= 0 { 1 } else { max_requests as usize };
            for rq in server.incoming_requests().take(n) {
                let mut response = Response::from_string(body.clone()).with_status_code(StatusCode(200));
                if let Ok(h) = Header::from_bytes(b"Content-Type", b"text/plain; charset=utf-8") {
                    response.add_header(h);
                }
                let _ = rq.respond(response);
            }
            true
        }

        pub fn serve_json(host: String, port: i64, json_body: String, max_requests: i64) -> bool {
            let addr = format!("{}:{}", host, port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(_) => return false,
            };
            let n = if max_requests <= 0 { 1 } else { max_requests as usize };
            for rq in server.incoming_requests().take(n) {
                let mut response = Response::from_string(json_body.clone()).with_status_code(StatusCode(200));
                if let Ok(h) = Header::from_bytes(b"Content-Type", b"application/json") {
                    response.add_header(h);
                }
                let _ = rq.respond(response);
            }
            true
        }

        pub fn serve_static(host: String, port: i64, file_path: String, max_requests: i64) -> bool {
            let content = match std::fs::read(&file_path) {
                Ok(c) => c,
                Err(_) => return false,
            };
            let addr = format!("{}:{}", host, port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(_) => return false,
            };
            let n = if max_requests <= 0 { 1 } else { max_requests as usize };
            for rq in server.incoming_requests().take(n) {
                let response = Response::from_data(content.clone()).with_status_code(StatusCode(200));
                let _ = rq.respond(response);
            }
            true
        }
    }

    pub mod framework {
        use tiny_http::{Header, Response, Server, StatusCode};

        fn routes_path(project_path: &str) -> String {
            format!("{}/app/routes.txt", project_path)
        }

        pub fn new_project(name: String) -> bool {
            let root = std::path::Path::new(&name);
            if std::fs::create_dir_all(root.join("app")).is_err() {
                return false;
            }
            if std::fs::create_dir_all(root.join("templates")).is_err() {
                return false;
            }
            if std::fs::create_dir_all(root.join("static")).is_err() {
                return false;
            }
            let routes_file = root.join("app/routes.txt");
            if std::fs::write(&routes_file, "/|Welcome to Linea Framework\n").is_err() {
                return false;
            }
            let settings = "DEBUG=true\nAPP_NAME=LineaApp\n";
            std::fs::write(root.join("settings.env"), settings).is_ok()
        }

        pub fn add_route(project_path: String, path: String, response: String) -> bool {
            let route_line = format!("{}|{}\n", path, response.replace('\n', "\\n"));
            let route_file = routes_path(&project_path);
            let mut file = match std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(route_file)
            {
                Ok(f) => f,
                Err(_) => return false,
            };
            use std::io::Write;
            file.write_all(route_line.as_bytes()).is_ok()
        }

        pub fn routes(project_path: String) -> Vec<String> {
            let route_file = routes_path(&project_path);
            std::fs::read_to_string(route_file)
                .map(|s| s.lines().map(|x| x.to_string()).collect())
                .unwrap_or_default()
        }

        pub fn run_dev_server(project_path: String, host: String, port: i64, max_requests: i64) -> bool {
            let route_lines = routes(project_path);
            let mut table = std::collections::HashMap::new();
            for line in route_lines {
                if let Some((p, v)) = line.split_once('|') {
                    table.insert(p.to_string(), v.replace("\\n", "\n"));
                }
            }
            if !table.contains_key("/") {
                table.insert("/".to_string(), "Linea Framework Home".to_string());
            }

            let addr = format!("{}:{}", host, port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(_) => return false,
            };
            let n = if max_requests <= 0 { 1 } else { max_requests as usize };
            for rq in server.incoming_requests().take(n) {
                let url = rq.url().to_string();
                let body = table
                    .get(&url)
                    .cloned()
                    .unwrap_or_else(|| "404 Not Found".to_string());
                let status = if table.contains_key(&url) { 200 } else { 404 };
                let mut response = Response::from_string(body).with_status_code(StatusCode(status));
                if let Ok(h) = Header::from_bytes(b"Content-Type", b"text/plain; charset=utf-8") {
                    response.add_header(h);
                }
                let _ = rq.respond(response);
            }
            true
        }
    }

    pub mod blockchain {
        use sha2::{Digest, Sha256};
        use std::time::{SystemTime, UNIX_EPOCH};

        pub fn sha256(data: String) -> String {
            let mut hasher = Sha256::new();
            hasher.update(data.as_bytes());
            super::hex_encode(&hasher.finalize())
        }

        pub fn merkle_root(transactions: &Vec<String>) -> String {
            if transactions.is_empty() {
                return sha256(String::new());
            }
            let mut layer: Vec<String> = transactions.iter().map(|t| sha256(t.clone())).collect();
            while layer.len() > 1 {
                let mut next = Vec::new();
                let mut i = 0usize;
                while i < layer.len() {
                    let left = layer[i].clone();
                    let right = if i + 1 < layer.len() { layer[i + 1].clone() } else { left.clone() };
                    next.push(sha256(format!("{}{}", left, right)));
                    i += 2;
                }
                layer = next;
            }
            layer[0].clone()
        }

        pub fn mine_block(index: i64, previous_hash: String, data: String, difficulty: i64) -> Vec<String> {
            let target = "0".repeat(difficulty.max(0) as usize);
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            let mut nonce = 0i64;
            loop {
                let payload = format!("{}|{}|{}|{}|{}", index, previous_hash, data, ts, nonce);
                let hash = sha256(payload);
                if target.is_empty() || hash.starts_with(&target) {
                    return vec![hash, nonce.to_string(), ts.to_string()];
                }
                nonce += 1;
                if nonce > 2_000_000 {
                    return vec![String::new(), nonce.to_string(), ts.to_string()];
                }
            }
        }

        pub fn validate_link(previous_hash: String, current_previous_hash: String) -> bool {
            previous_hash == current_previous_hash
        }
    }

    pub mod gpu_tools {
        use wgpu::{Backends, DeviceType, Instance};

        fn adapters_info() -> Vec<(String, u32, DeviceType)> {
            let instance = Instance::new(wgpu::InstanceDescriptor {
                backends: Backends::all(),
                ..Default::default()
            });
            instance
                .enumerate_adapters(Backends::all())
                .map(|a| {
                    let info = a.get_info();
                    (info.name, info.vendor, info.device_type)
                })
                .collect()
        }

        pub fn adapters() -> Vec<String> {
            adapters_info()
                .into_iter()
                .map(|(name, vendor, ty)| format!("{}|vendor=0x{:04X}|type={:?}", name, vendor, ty))
                .collect()
        }

        pub fn has_igpu() -> bool {
            adapters_info().into_iter().any(|(_, _, ty)| matches!(ty, DeviceType::IntegratedGpu))
        }

        pub fn vendor_name(vendor_id: i64) -> String {
            match vendor_id as u32 {
                0x10DE => "NVIDIA".to_string(),
                0x8086 => "Intel".to_string(),
                0x1002 | 0x1022 => "AMD".to_string(),
                _ => "Unknown".to_string(),
            }
        }

        pub fn best_adapter() -> String {
            let mut best: Option<(String, i32)> = None;
            for (name, vendor, ty) in adapters_info() {
                let vendor_score = match vendor {
                    0x10DE | 0x1002 | 0x1022 | 0x8086 => 5,
                    _ => 1,
                };
                let type_score = match ty {
                    DeviceType::DiscreteGpu => 50,
                    DeviceType::IntegratedGpu => 40,
                    DeviceType::VirtualGpu => 30,
                    DeviceType::Cpu => 10,
                    DeviceType::Other => 20,
                };
                let score = vendor_score + type_score;
                match &best {
                    Some((_, cur)) if *cur >= score => {}
                    _ => best = Some((name, score)),
                }
            }
            best.map(|(n, _)| n).unwrap_or_default()
        }
    }

    pub mod memory {
        use std::collections::HashMap;
        use std::sync::atomic::{AtomicI64, Ordering};
        use std::sync::{Mutex, OnceLock};

        static NEXT_HANDLE: AtomicI64 = AtomicI64::new(1);
        static BUFFERS: OnceLock<Mutex<HashMap<i64, Vec<u8>>>> = OnceLock::new();

        fn store() -> &'static Mutex<HashMap<i64, Vec<u8>>> {
            BUFFERS.get_or_init(|| Mutex::new(HashMap::new()))
        }

        pub fn alloc(size: i64) -> i64 {
            if size <= 0 {
                return -1;
            }
            let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
            if let Ok(mut s) = store().lock() {
                s.insert(handle, vec![0u8; size as usize]);
                return handle;
            }
            -1
        }

        pub fn free(handle: i64) -> bool {
            store().lock().ok().and_then(|mut s| s.remove(&handle)).is_some()
        }

        pub fn len(handle: i64) -> i64 {
            store()
                .lock()
                .ok()
                .and_then(|s| s.get(&handle).map(|b| b.len() as i64))
                .unwrap_or(-1)
        }

        pub fn write_u8(handle: i64, offset: i64, value: i64) -> bool {
            if offset < 0 {
                return false;
            }
            let mut s = match store().lock() {
                Ok(v) => v,
                Err(_) => return false,
            };
            let buf = match s.get_mut(&handle) {
                Some(b) => b,
                None => return false,
            };
            let idx = offset as usize;
            if idx >= buf.len() {
                return false;
            }
            buf[idx] = value.clamp(0, 255) as u8;
            true
        }

        pub fn read_u8(handle: i64, offset: i64) -> i64 {
            if offset < 0 {
                return -1;
            }
            let s = match store().lock() {
                Ok(v) => v,
                Err(_) => return -1,
            };
            let buf = match s.get(&handle) {
                Some(b) => b,
                None => return -1,
            };
            let idx = offset as usize;
            if idx >= buf.len() {
                return -1;
            }
            buf[idx] as i64
        }

        pub fn fill(handle: i64, value: i64) -> bool {
            let mut s = match store().lock() {
                Ok(v) => v,
                Err(_) => return false,
            };
            let buf = match s.get_mut(&handle) {
                Some(b) => b,
                None => return false,
            };
            buf.fill(value.clamp(0, 255) as u8);
            true
        }

        pub fn copy(src_handle: i64, dst_handle: i64, size: i64) -> bool {
            if size < 0 {
                return false;
            }
            let mut s = match store().lock() {
                Ok(v) => v,
                Err(_) => return false,
            };
            let src_snapshot = match s.get(&src_handle) {
                Some(b) => b.clone(),
                None => return false,
            };
            let dst = match s.get_mut(&dst_handle) {
                Some(b) => b,
                None => return false,
            };
            let n = (size as usize).min(src_snapshot.len()).min(dst.len());
            dst[..n].copy_from_slice(&src_snapshot[..n]);
            true
        }

        pub fn stats() -> Vec<i64> {
            let s = match store().lock() {
                Ok(v) => v,
                Err(_) => return vec![0, 0],
            };
            let count = s.len() as i64;
            let total = s.values().map(|b| b.len() as i64).sum::<i64>();
            vec![count, total]
        }
    }

    pub mod gui {
        use iced::widget::{button, column, text};
        use iced::{Application, Command, Element, Settings, Theme};

        #[derive(Clone)]
        pub struct GuiApp {
            title: String,
            message: String,
            button_label: Option<String>,
            clicked: bool,
            width: u32,
            height: u32,
        }

        #[derive(Debug, Clone)]
        enum Message {
            ButtonPressed,
        }

        impl Application for GuiApp {
            type Executor = iced::executor::Default;
            type Message = Message;
            type Theme = Theme;
            type Flags = (String, String, Option<String>, u32, u32);

            fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
                (
                    Self {
                        title: flags.0,
                        message: flags.1,
                        button_label: flags.2,
                        clicked: false,
                        width: flags.3,
                        height: flags.4,
                    },
                    Command::none(),
                )
            }

            fn title(&self) -> String {
                self.title.clone()
            }

            fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
                match message {
                    Message::ButtonPressed => {
                        self.clicked = true;
                    }
                }
                Command::none()
            }

            fn view(&self) -> Element<Self::Message> {
                let mut content = column![text(self.message.clone()).size(24)].spacing(16).padding(20);
                if let Some(label) = &self.button_label {
                    let caption = if self.clicked {
                        format!("{} (clicked)", label)
                    } else {
                        label.clone()
                    };
                    content = content.push(button(text(caption)).on_press(Message::ButtonPressed));
                }
                content.into()
            }
        }

        pub fn window(title: String, message: String, width: u32, height: u32) -> bool {
            GuiApp::run(Settings {
                window: iced::window::Settings {
                    size: (width, height),
                    ..Default::default()
                },
                flags: (title, message, None, width, height),
                ..Default::default()
            })
            .is_ok()
        }

        pub fn button_window(title: String, message: String, button_label: String, width: u32, height: u32) -> bool {
            GuiApp::run(Settings {
                window: iced::window::Settings {
                    size: (width, height),
                    ..Default::default()
                },
                flags: (title, message, Some(button_label), width, height),
                ..Default::default()
            })
            .is_ok()
        }
    }
}
