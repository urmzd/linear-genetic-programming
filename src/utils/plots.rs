use std::{error, fmt, ops::Range};

use plotters::{
    prelude::{BitMapBackend, ChartBuilder, ErrorBar, IntoDrawingArea},
    style::{colors, IntoFont, WHITE},
};

use crate::core::{characteristics::Fitness, population::Population};

pub fn plot_population_benchmarks<T>(
    populations: Vec<Population<T>>,
    plot_path: &str,
    y_range: Range<f32>,
) -> Result<(), Box<dyn error::Error>>
where
    T: Fitness + Clone + Ord + fmt::Debug,
{
    let root = BitMapBackend::new(plot_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let n_benchmarks = populations.len();

    let mut chart = ChartBuilder::on(&root)
        .caption("Fitness Over Generations", ("sans-serif", 50).into_font())
        .margin(5u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0..n_benchmarks, y_range)?;

    chart.configure_mesh().draw()?;

    let benchmarks: Vec<[f32; 3]> = populations
        .into_iter()
        .map(|population| {
            let best = population.first();
            let median = population.middle();
            let worst = population.last();

            let benchmark = [best, median, worst]
                .map(|quantile| quantile.unwrap().get_fitness().unwrap().into_inner());

            benchmark
        })
        .collect();

    chart
        .draw_series(
            benchmarks
                .iter()
                .enumerate()
                .map(|(index, [best, median, worst])| {
                    ErrorBar::new_vertical(index, *worst, *median, *best, colors::BLUE, 10)
                }),
        )
        .unwrap();

    root.present()?;
    Ok(())
}
