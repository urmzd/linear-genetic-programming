use std::{error, fmt, ops::Range};

use itertools::Itertools;
use ndarray::{aview1, Array, Axis, Dim};
use ndarray_stats::{interpolate, QuantileExt};
use noisy_float::prelude::n64;
use plotters::{
    prelude::{BitMapBackend, ChartBuilder, ErrorBar, IntoDrawingArea},
    style::{colors, IntoFont, WHITE},
};

use crate::core::characteristics::Fitness;

pub fn plot_population_benchmarks<T>(
    mut populations: Array<T, Dim<[usize; 2]>>,
    plot_path: &str,
    y_range: Range<f32>,
) -> Result<(), Box<dyn error::Error>>
where
    T: Fitness + Clone + Ord + fmt::Debug,
{
    let root = BitMapBackend::new(plot_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let shape = populations.shape();
    let n_benchmarks = shape[0];

    let mut chart = ChartBuilder::on(&root)
        .caption("Fitness Over Generations", ("sans-serif", 50).into_font())
        .margin(5u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0..n_benchmarks, y_range)?;

    chart.configure_mesh().draw()?;

    let benchmarks: Vec<(f32, f32, f32)> = {
        let mut v = Vec::with_capacity(n_benchmarks);
        for mut population in populations.rows_mut() {
            let quantile_arr = [n64(1.), n64(0.5), n64(0.)];
            let quantiles = aview1(&quantile_arr);
            let values = population
                .quantiles_axis_mut(Axis(0), &quantiles, &interpolate::Higher)
                .unwrap();
            let value_tuples = values
                .to_vec()
                .into_iter()
                .map(|program| program.get_fitness().unwrap().into_inner())
                .collect_tuple()
                .unwrap();
            v.push(value_tuples);
        }
        v
    };

    chart
        .draw_series(
            benchmarks
                .iter()
                .enumerate()
                .map(|(index, (best, median, worst))| {
                    ErrorBar::new_vertical(index, *worst, *median, *best, colors::BLUE, 10)
                }),
        )
        .unwrap();

    root.present()?;
    Ok(())
}
