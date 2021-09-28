use std::{cmp::Ordering, collections::HashMap, fmt::Write, hash::Hash};

use plotters::{
    coord::Shift,
    prelude::{ChartBuilder, DrawingArea, DrawingBackend, LineSeries, PathElement},
    style::{Color, IntoFont, Palette, Palette99, BLACK, WHITE},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileLocation {
    pub file: &'static str,
    pub line: u32,
    pub col: u32,
}

impl ToString for FileLocation {
    fn to_string(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.col)
    }
}

/// When reporting a value, a target is given, this target contains information
/// about the origin of the report, as well as how the value can be used.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReporterTarget {
    pub origin: Option<FileLocation>,
    pub description: Option<&'static str>,
    pub index: Option<u32>,
}

impl ToString for ReporterTarget {
    fn to_string(&self) -> String {
        let mut label = self
            .description
            .map(ToOwned::to_owned)
            .or_else(|| self.origin.clone()?.to_string().into())
            .unwrap_or_else(|| "no-label".into());
        if let Some(i) = self.index {
            write!(&mut label, "[{}]", i).unwrap();
        }
        label
    }
}

#[derive(Default, Clone, Debug)]
pub struct Reporter {
    current_step: usize,
    per_step: HashMap<ReporterTarget, Vec<f64>>,
}

impl Reporter {
    pub fn new() -> Reporter {
        Self::default()
    }

    pub fn report_num(&mut self, target: ReporterTarget, value: f64) {
        println!("{:?} => {}", target.description, value);
        let vec = self.per_step.entry(target).or_default();
        match vec.len().cmp(&self.current_step) {
            Ordering::Equal => vec.push(value),
            Ordering::Less => {
                vec.extend(std::iter::repeat(f64::NAN).take(self.current_step - vec.len()));
                vec.push(value);
            }
            Ordering::Greater => vec[self.current_step] = value,
        }
    }

    pub fn render_chart<DB>(&self, db: DrawingArea<DB, Shift>)
    where
        DB: DrawingBackend,
    {
        let max_step = self
            .per_step
            .values()
            .map(|v| v.len())
            .max()
            .expect("no values reported");

        let y_range = self
            .per_step
            .values()
            .flatten()
            .copied()
            .filter(|v| !f64::is_nan(*v))
            .map(|v| (v, v))
            .reduce(|(c_min, c_max), (n_min, n_max)| (c_min.min(n_min), c_max.max(n_max)))
            .expect("no values reported");

        let mut chart = ChartBuilder::on(&db)
            .caption("Simulation Report", ("sans-serif", 50).into_font())
            .margin(25)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..max_step, y_range.0..y_range.1)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        for (i, (target, series)) in self.per_step.iter().enumerate() {
            let label = target.to_string();
            let color = Palette99::pick(i).to_rgba();
            chart
                .draw_series(LineSeries::new(
                    series.iter().enumerate().map(|(i, v)| (i, *v)),
                    color,
                ))
                .unwrap()
                .label(label)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();
    }

    pub fn write_csv(&self) {
        todo!()
    }

    pub(crate) fn set_step(&mut self, step: u32) {
        self.current_step = step as usize;
    }
}

macro_rules! report {
    ($r:expr, $desc:literal[$idx:expr], $val:expr) => {
        crate::report::report!("internal", $r, Some($desc), Some($idx), $val)
    };
    ($r:expr, $desc:literal, $val:expr) => {
        crate::report::report!("internal", $r, Some($desc), None, $val)
    };
    ($r:expr, $val:expr) => {
        crate::report::report!("internal", $r, None, None, $val)
    };
    ("internal", $r:expr, $desc:expr, $idx:expr, $val:expr) => {{
        let target = crate::report::ReporterTarget {
            origin: Some(crate::report::FileLocation {
                file: file!(),
                line: line!(),
                col: column!(),
            }),
            description: $desc,
            index: $idx,
        };
        $r.report_num(target, $val);
    }};
}
pub(crate) use report;

#[cfg(test)]
pub mod test {
    use plotters::prelude::{IntoDrawingArea, SVGBackend};

    use super::*;

    #[test]
    fn test_csv_plot() {
        let r = Reporter::new();
        let mut s = String::new();
        r.render_chart(SVGBackend::with_string(&mut s, (1000, 1000)).into_drawing_area());
        assert!(!s.is_empty());
    }
}
