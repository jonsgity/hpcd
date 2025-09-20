// Hancock Pattern in Rust
// Requires: plotters = "0.3", clap = "4.0"

use std::collections::{HashMap, HashSet};
use std::env;
use plotters::prelude::*;

fn sum_digits(mut n: u32, base: u32) -> u32 {
    let mut s = 0;
    while n > 0 {
        s += n % base;
        n /= base;
    }
    s
}

fn minimal_rotation(cycle: &[u32]) -> Vec<u32> {
    if cycle.is_empty() {
        return vec![];
    }
    let n = cycle.len();
    let mut rotations = Vec::with_capacity(n);
    for i in 0..n {
        let mut rot = cycle[i..].to_vec();
        rot.extend_from_slice(&cycle[..i]);
        rotations.push(rot);
    }
    rotations.into_iter().min().unwrap()
}

fn detect_cycle(n: u32, base: u32, max_iter: usize) -> Option<Vec<u32>> {
    let mut history = Vec::new();
    let mut x = n;
    for _ in 0..max_iter {
        let s = sum_digits(x, base);
        x = s * s;
        if let Some(idx) = history.iter().position(|&v| v == x) {
            let cycle = &history[idx..];
            return Some(minimal_rotation(cycle));
        }
        history.push(x);
    }
    None
}

fn get_label(idx: usize) -> String {
    let letters = b"abcdefghijklmnopqrstuvwxyz";
    if idx < 26 {
        (letters[idx] as char).to_string()
    } else {
        let first = letters[(idx / 26) - 1] as char;
        let second = letters[idx % 26] as char;
        format!("{}{}", first, second)
    }
}

fn to_base_digits(mut n: u32, base: u32) -> Vec<u32> {
    if n == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % base);
        n /= base;
    }
    digits.reverse();
    digits
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: hancock_pattern <base> [N] [vertical_spacing]");
        std::process::exit(1);
    }
    let base: u32 = args[1].parse()?;
    let n_max: u32 = if args.len() > 2 { args[2].parse()? } else { 255 };
    let vertical_spacing: i32 = if args.len() > 3 { args[3].parse()? } else { 20 };

    let mut patterns = Vec::new();
    let mut cycle_to_label: HashMap<Vec<u32>, String> = HashMap::new();
    let mut label_to_cycle: HashMap<String, Vec<u32>> = HashMap::new();
    let mut label_idx = 0;

    for n in 1..=n_max {
        let cycle = detect_cycle(n, base, 100);
        let label = if let Some(cycle) = cycle {
            if !cycle_to_label.contains_key(&cycle) {
                let lbl = get_label(label_idx);
                cycle_to_label.insert(cycle.clone(), lbl.clone());
                label_to_cycle.insert(lbl.clone(), cycle.clone());
                label_idx += 1;
            }
            cycle_to_label.get(&cycle).unwrap().clone()
        } else {
            "other".to_string()
        };
        patterns.push(label);
    }

    // Map labels to y-values and colors
    let mut unique_labels: Vec<String> = patterns.iter().cloned().collect::<HashSet<_>>().into_iter().collect();
    unique_labels.sort_by(|a, b| if a == "other" { std::cmp::Ordering::Greater } else if b == "other" { std::cmp::Ordering::Less } else { a.cmp(b) });
    let label_to_y: HashMap<String, i32> = unique_labels.iter().enumerate().map(|(i, lbl)| (lbl.clone(), i as i32 * vertical_spacing)).collect();
    let palette = Palette99::pick;
    let color_map: HashMap<String, RGBColor> = unique_labels.iter().enumerate().map(|(i, lbl)| (lbl.clone(), palette(i))).collect();

    let x_vals: Vec<u32> = (1..=n_max).collect();
    let y_vals: Vec<i32> = patterns.iter().map(|p| *label_to_y.get(p).unwrap()).collect();
    let colors: Vec<RGBColor> = patterns.iter().map(|p| color_map.get(p).unwrap().clone()).collect();

    // Calculate figure height
    let fig_height = std::cmp::max(200, (unique_labels.len() * vertical_spacing) as u32);
    let root = BitMapBackend::new("hancock_pattern.png", (1200, fig_height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Hancock Pattern Scatter Plot (base {}, N={}, spacing={})", base, n_max, vertical_spacing), ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(1u32..n_max, 0i32..(fig_height as i32))?;

    chart.configure_mesh()
        .x_desc("Positive Integer")
        .y_desc("Pattern")
        .y_labels(unique_labels.len())
        .y_label_formatter(&|y| {
            unique_labels.iter().find(|lbl| label_to_y[lbl.as_str()] == *y).cloned().unwrap_or("".to_string())
        })
        .draw()?;

    for ((&x, &y), color) in x_vals.iter().zip(y_vals.iter()).zip(colors.iter()) {
        chart.draw_series(PointSeries::of_element(vec![(x, y)], 5, color, &|c, s, st| {
            return EmptyElement::at(c)    
                + Circle::new((0,0), s, st.filled());
        }))?;
    }

    // Print key mapping pattern labels to cycle details
    println!("\nPattern Key:");
    for lbl in unique_labels.iter().filter(|l| l.as_str() != "other") {
        let cycle = &label_to_cycle[lbl];
        let cycle_str = cycle.iter().map(|&x| to_base_digits(x, base).iter().map(|d| d.to_string()).collect::<Vec<_>>().join(".")).collect::<Vec<_>>().join(", ");
        println!("  {}: {}", lbl, cycle_str);
    }

    println!("\nPlot saved to hancock_pattern.png");
    Ok(())
}
