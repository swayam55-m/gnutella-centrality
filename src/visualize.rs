use plotters::prelude::*;
use std::collections::HashMap;
use petgraph::graph::NodeIndex;
pub fn draw_bar_chart(
    title: &str,
    filename: &str,
    scores: &HashMap<NodeIndex, f64>,
    top_n: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut sorted: Vec<_> = scores.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    let top_nodes: Vec<_> = sorted.iter().take(top_n).collect();
    let max_score = top_nodes.iter().map(|(_, &v)| v).fold(0./0., f64::max);

    let labels: Vec<String> = top_nodes.iter().map(|(idx, _)| idx.index().to_string()).collect();
    let values: Vec<f64> = top_nodes.iter().map(|(_, &v)| v).collect();

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30).into_font())
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            0..top_n as i32,
            0.0..(max_score * 1.1),
        )?;

    chart.configure_mesh()
        .x_labels(top_n)
        .x_label_formatter(&|x| {
            let index = *x as usize;
            if index < labels.len() {
                labels[index].clone()
            } else {
                "".to_string()
            }
        })
        .y_desc("Score")
        .x_desc("Node ID")
        .axis_desc_style(("sans-serif", 18))
        .draw()?;

    chart.draw_series(
        values.iter().enumerate().take(top_n).map(|(i, &val)| {
            Rectangle::new(
                [(i as i32, 0.0), ((i + 1) as i32, val)],
                BLUE.filled(),
            )
        }),
    )?;          

    root.present()?;
    Ok(())
}
