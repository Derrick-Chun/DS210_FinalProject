use std::collections::HashMap;
use crate::graph::Graph;

extern crate plotters;

use plotters::prelude::*;

pub fn degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut distribution = HashMap::new();
    for edges in graph.adjacency_list.values() {
        let degree = edges.len();
        *distribution.entry(degree).or_insert(0) += 1;
    }
    distribution
}

pub fn plot_degree_distribution(distribution: &HashMap<usize, usize>) {
    println!("Degree Distribution:");
    for (degree, count) in distribution {
        println!("Degree {}: {}", degree, count);
    }

    let root_area = BitMapBackend::new("degree_distribution_histogram.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Degree Distribution Histogram", ("Arial", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..120, 0..*distribution.values().max().unwrap_or(&0) + 1)
        .unwrap();

    chart.configure_mesh()
        .x_desc("Degree")
        .y_desc("Count")
        .draw()
        .unwrap();

    chart.draw_series(
        distribution.iter().map(|(&degree, &count)| {
            Rectangle::new(
                [(degree as i32, 0), (degree as i32 + 1, count)],
                BLUE.filled(),
            )
        })
    ).unwrap();
}

pub fn trust_correlation(edges: &[(i32, i32, i32)]) {
    let mut trust_to_neighbors = HashMap::new();
    for &(source, _target, weight) in edges {
        trust_to_neighbors.entry(source).or_insert_with(Vec::new).push(weight);
    }

    println!("Trust Rating vs Number of Connections:");
    let mut points: Vec<(f64, f64)> = Vec::new();
    for (vertex, trust_ratings) in trust_to_neighbors {
        let degree = trust_ratings.len();
        let avg_trust: f64 = trust_ratings.iter().map(|&x| x as f64).sum::<f64>() / degree as f64;
        points.push((avg_trust, degree as f64));

        println!("Vertex {}: Degree = {}, Average Trust = {:.2}", vertex, degree, avg_trust);
    }

    let root_area = BitMapBackend::new("trust_vs_degree.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Average Trust vs Degree", ("Arial", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..points.iter().map(|(x, _)| *x).fold(0./0., f64::max) + 1.0, 0f64..points.iter().map(|(_, y)| *y).fold(0./0., f64::max) + 1.0)
        .unwrap();

    chart.configure_mesh()
        .x_desc("Average Trust")
        .y_desc("Degree")
        .draw()
        .unwrap();

    chart.draw_series(
        points.iter().map(|(x, y)| Circle::new((*x, *y), 3, GREEN.filled()))
    ).unwrap();

    // evcxr_figure((640, 480), |root| {
    //     // the following code will create a chart context
    //     let mut chart = ChartBuilder::on(&root)
    //         .caption("Normal Distribution", ("Arial", 20).into_font())
    //         .x_label_area_size(40)
    //         .y_label_area_size(40)
    //         .build_ranged(0f64..1f64, 0f64..1f64)?;
        
    //     chart.configure_mesh()
    //         .disable_x_mesh()
    //         .disable_y_mesh()
    //         .draw()?;
        
    //     // Draw little green circles
    //     chart.draw_series(points.iter().map(|(x,y)| Circle::new((*x,*y), 3, GREEN.filled())));
        
    //     // You can always freely draw on the drawing backend.  So we can add background after the fact
    //     //  let area = chart.plotting_area();
    //     //  let two_sigma = sd * 2.0;
    //     // area.draw(&Rectangle::new(
    //     //     [(0.5 - two_sigma, 0.5 - two_sigma), (0.5 + two_sigma, 0.5 + two_sigma)], 
    //     //     RED.mix(0.3).filled())
    //     // )?;
    //     // area.draw(&Cross::new((0.5, 0.5), 5, &RED))?;
        
    //     Ok(())
    // }).style("width:60%")
}