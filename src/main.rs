mod bigram;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use plotly::{HeatMap, Layout, Plot, common::ColorScale, layout::Axis};

use crate::bigram::Bigram;

fn main() -> std::io::Result<()> {
    let file = File::open("./assets/names.txt")?;
    let reader = BufReader::new(file);
    let names: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    println!("{:?}", &names[..10]);
    println!("Total number of names: {}", names.len());
    let bigram = Bigram::new(names);
    let tensor = bigram.to_tensor();
    let sum = tensor.clone().sum().into_scalar();
    // turned into probability
    let x = tensor.clone() / sum;
    let raw: Vec<f32> = x.into_data().to_vec::<f32>().unwrap();

    let labels: Vec<String> = std::iter::once('.')
        .chain('a'..='z')
        .map(|c| c.to_string())
        .collect();
    let matrix: Vec<Vec<f32>> = raw.chunks(27).map(|row| row.to_vec()).collect();
    let trace = HeatMap::new(labels.clone(), labels.clone(), matrix)
        .color_scale(ColorScale::Palette(
            plotly::common::ColorScalePalette::Blues,
        ))
        .reverse_scale(true);

    let layout = Layout::new()
        .x_axis(Axis::new().tick_length(27))
        .y_axis(Axis::new().tick_length(27));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.write_html("bigram_heatmap.html");

    Ok(())
}
