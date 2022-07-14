use std::{error, fmt, ops::Range};

use itertools::Itertools;
use ndarray::{aview1, Array, Axis, Dim};
use ndarray_stats::{interpolate, QuantileExt};
use noisy_float::prelude::n64;
use ordered_float::OrderedFloat;
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

    let n_benchmarks = populations.shape()[0];

    let mut chart = ChartBuilder::on(&root)
        .caption("Fitness Over Generations", ("sans-serif", 50).into_font())
        .margin(5u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0..n_benchmarks, y_range)?;

    chart.configure_mesh().draw()?;

    let quantiles = populations
        .quantiles_axis_mut(
            Axis(0),
            &aview1(&[n64(1.), n64(0.5), n64(0.)]),
            &interpolate::Higher,
        )
        .unwrap();

    let benchmarks = quantiles
        .axis_iter(Axis(1))
        .map(|b| {
            let mut b_vec = b.to_vec();
            let benchmark = (
                b_vec
                    .pop()
                    .and_then(|p| p.get_fitness())
                    .map(|f| f.into_inner())
                    .unwrap(),
                b_vec
                    .pop()
                    .and_then(|p| p.get_fitness())
                    .map(|f| f.into_inner())
                    .unwrap(),
                b_vec
                    .pop()
                    .and_then(|p| p.get_fitness())
                    .map(|f| f.into_inner())
                    .unwrap(),
            );

            benchmark
        })
        .sorted_by_key(|(w, _m, _b)| OrderedFloat(*w))
        .collect_vec();

    chart
        .draw_series(
            benchmarks
                .iter()
                .enumerate()
                .map(|(index, (worst, median, best))| {
                    ErrorBar::new_vertical(index, *worst, *median, *best, colors::BLUE, 10)
                }),
        )
        .unwrap();

    root.present()?;
    Ok(())
}
