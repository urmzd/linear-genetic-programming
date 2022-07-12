use std::error;

use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries, PathElement},
    style::{Color, IntoFont, BLACK, BLUE, GREEN, RED, WHITE},
};

use crate::core::characteristics::FitnessScore;

pub fn plot_from_benchmarks(
    // benchmarks: Array<
    benchmarks: Vec<ComplexityBenchmark<Option<FitnessScore>>>,
    plot_path: &str,
) -> Result<(), Box<dyn error::Error>> {
    let fitness_benchmarks: Vec<ComplexityBenchmark<f32>> = benchmarks
        .into_iter()
        .map(|benchmark| ComplexityBenchmark {
            best: benchmark.best.unwrap().into_inner(),
            worst: benchmark.worst.unwrap().into_inner(),
            median: benchmark.median.unwrap().into_inner(),
        })
        .collect();
    let root = BitMapBackend::new(plot_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Fitness Over Generations", ("sans-serif", 50).into_font())
        .margin(5u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0..fitness_benchmarks.len(), 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..fitness_benchmarks.len())
                .map(|x_i| (x_i, fitness_benchmarks.get(x_i).unwrap().best)),
            &RED,
        ))?
        .label("Best")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            (0..fitness_benchmarks.len())
                .map(|x_i| (x_i, fitness_benchmarks.get(x_i).unwrap().median)),
            &GREEN,
        ))?
        .label("Median")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(LineSeries::new(
            (0..fitness_benchmarks.len())
                .map(|x_i| (x_i, fitness_benchmarks.get(x_i).unwrap().worst)),
            &BLUE,
        ))?
        .label("Worst")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
