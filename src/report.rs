use std::{collections::HashMap, hash::Hash};

use plotters::{
    coord::Shift,
    prelude::{ChartBuilder, DrawingArea, DrawingBackend, LineSeries, PathElement},
    style::{Color, IntoFont, BLACK, RED, WHITE},
};

/// Types implementing this trait can be used to report values and events
/// throughout the simulation.
pub trait Reporter: Sized {
    fn report_num(&mut self, target: ReporterTarget, value: f64);

    fn report_event(&mut self, target: ReporterTarget, value: String);

    fn stepped(&mut self, step: u32) -> SteppedReporter<&mut Self> {
        SteppedReporter { step, inner: self }
    }
}

impl<R> Reporter for &mut R
where
    R: Reporter,
{
    fn report_num(&mut self, target: ReporterTarget, value: f64) {
        (*self).report_num(target, value)
    }

    fn report_event(&mut self, target: ReporterTarget, value: String) {
        (*self).report_event(target, value)
    }
}

/// When reporting a value, a target is given, this target contains information
/// about the origin of the report, as well as how the value can be used.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReporterTarget {
    step: u32,
    description: &'static str,
}

/// This reporter augments produced value with information about the current step.
#[derive(Debug, Clone)]
pub struct SteppedReporter<R> {
    step: u32,
    inner: R,
}

impl<R> Reporter for SteppedReporter<R>
where
    R: Reporter,
{
    fn report_num(&mut self, mut target: ReporterTarget, value: f64) {
        target.step = self.step;
        self.report_num(target, value);
    }

    fn report_event(&mut self, target: ReporterTarget, value: String) {
        target.step = self.step;
        self.report_event(target, value);
    }
}

pub struct PlottersReporter {
    targets: HashMap<ReporterTarget, Vec<f64>>,
}

impl Reporter for PlottersReporter {
    fn report_num(&mut self, target: ReporterTarget, value: f64) {
        let mut chart = ChartBuilder::on(&self.backend)
            .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))
            .unwrap()
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();
    }

    fn report_event(&mut self, target: ReporterTarget, value: String) {
        todo!()
    }
}
