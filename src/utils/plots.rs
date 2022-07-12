use std::error;

use ndarray::{Array, Axis, Dim};
use ndarray_stats::{
    interpolate::{self},
    QuantileExt,
};
use noisy_float::prelude::N64;
use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries, PathElement},
    style::{Color, IntoFont, BLACK, BLUE, GREEN, RED, WHITE},
};

use crate::core::characteristics::Fitness;

pub fn plot_population_benchmarks<T: Fitness + Clone + Ord>(
    mut populations: Array<T, Dim<[usize; 2]>>,
    plot_path: &str,
) -> Result<(), Box<dyn error::Error>> {
    let root = BitMapBackend::new(plot_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let n_benchmarks = populations.shape()[0];

    let mut chart = ChartBuilder::on(&root)
        .caption("Fitness Over Generations", ("sans-serif", 50).into_font())
        .margin(5u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0..n_benchmarks, 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    let best: Vec<(usize, f32)> = populations
        .quantile_axis_mut(Axis(0), N64::unchecked_new(1.), &interpolate::Higher)
        .unwrap()
        .indexed_iter()
        .into_iter()
        .map(|(x_i, x)| (x_i, x.get_fitness().unwrap().into_inner()))
        .collect();

    let median: Vec<(usize, f32)> = populations
        .quantile_axis_mut(Axis(0), N64::unchecked_new(0.5), &interpolate::Higher)
        .unwrap()
        .indexed_iter()
        .into_iter()
        .map(|(x_i, x)| (x_i, x.get_fitness().unwrap().into_inner()))
        .collect();

    let worst: Vec<(usize, f32)> = populations
        .quantile_axis_mut(Axis(0), N64::unchecked_new(0.), &interpolate::Higher)
        .unwrap()
        .indexed_iter()
        .into_iter()
        .map(|(x_i, x)| (x_i, x.get_fitness().unwrap().into_inner()))
        .collect();

    chart
        .draw_series(LineSeries::new(best, &RED))?
        .label("Best")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(median, &GREEN))?
        .label("Median")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(LineSeries::new(worst, &BLUE))?
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
