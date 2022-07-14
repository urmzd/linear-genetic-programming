use std::{error, fmt, ops::Range};

use ndarray::{aview1, Array, Axis, Dim};
use ndarray_stats::{interpolate, QuantileExt};
use noisy_float::prelude::n64;
use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries, PathElement},
    style::{Color, IntoFont, Palette, Palette99, BLACK, WHITE},
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
            &aview1(&[n64(0.), n64(0.5), n64(1.)]),
            &interpolate::Higher,
        )
        .unwrap();

    let mut worst = vec![];
    let mut median = vec![];
    let mut best = vec![];

    quantiles
        .axis_iter(Axis(1))
        .enumerate()
        .for_each(|(index, b)| {
            let mut b_vec = b.to_vec();
            let (best_p, median_p, worst_p) = (
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
            worst.push((index, worst_p));
            median.push((index, median_p));
            best.push((index, best_p));
        });

    [("WORST", worst), ("MEDIAN", median), ("BEST", best)]
        .iter()
        .enumerate()
        .for_each(|(index, (label, metric))| {
            let color = Palette99::pick(index).mix(0.9);
            let legend = move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color);

            chart
                .draw_series(LineSeries::new(metric.clone(), color.stroke_width(3)))
                .unwrap()
                .label(label.clone())
                .legend(legend);
        });

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
