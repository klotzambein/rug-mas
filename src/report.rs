use std::{
    cmp::Ordering, collections::HashMap, fmt::Write, hash::Hash, io::Write as IoWrite, path::Path,
};

use plotters::{
    coord::Shift,
    prelude::{ChartBuilder, DrawingArea, DrawingBackend, LineSeries},
    style::{Color, IntoFont, BLACK},
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

    pub fn render_chart<DA>(&self, da: DrawingArea<DA, Shift>)
    where
        DA: DrawingBackend,
    {
        let chart_count = self.per_step.len();
        let chart_width = (chart_count as f64).sqrt().round() as usize;
        let chart_height = chart_count / chart_width + (chart_count % chart_width != 0) as usize;
        let das = da.split_evenly((chart_width, chart_height));

        let mut cols = self.per_step.keys().collect::<Vec<_>>();
        cols.sort_by_key(|k| k.to_string());

        for ((target, series), da) in cols.iter().map(|c| (c, &self.per_step[c])).zip(das) {
            let label = target.to_string();
            let color = BLACK.mix(0.7);

            let max_step = series.len();

            let y_range = series
                .iter()
                .copied()
                .filter(|v| !f64::is_nan(*v))
                .map(|v| (v, v))
                .reduce(|(c_min, c_max), (n_min, n_max)| (c_min.min(n_min), c_max.max(n_max)))
                .expect("no values reported");

            let mut chart = ChartBuilder::on(&da)
                .caption(label, ("monospace", 25).into_font())
                .margin(10)
                .margin_top(0)
                .margin_bottom(0)
                .x_label_area_size(25)
                .y_label_area_size(50)
                .build_cartesian_2d(0..max_step, y_range.0..y_range.1)
                .unwrap();

            chart
                .configure_mesh()
                .y_label_formatter(&|x| format!("{:2.2e}", x))
                .draw()
                .unwrap();

            chart
                .draw_series(LineSeries::new(
                    series.iter().enumerate().map(|(i, v)| (i, *v)),
                    color,
                ))
                .unwrap();
            //     .label(label)
            //     .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

            // chart
            //     .configure_series_labels()
            //     .background_style(&WHITE.mix(0.8))
            //     .border_style(&BLACK)
            //     .draw()
            //     .unwrap();
        }
    }

    pub fn write_csv(&self, path: impl AsRef<Path>) {
        let mut cols = self.per_step.keys().collect::<Vec<_>>();
        cols.sort_by_key(|k| k.to_string());

        let mut file = std::fs::File::create(path).expect("can't create csv file");

        write!(file, "step").unwrap();
        for c in &cols {
            write!(file, ",{}", c.to_string()).unwrap();
        }

        for step in 0.. {
            let mut stop = true;

            writeln!(file).unwrap();
            write!(file, "{}", step).unwrap();

            for c in &cols {
                let val = self.per_step[c].get(step).copied();
                if val.is_some() {
                    stop = false;
                }
                write!(file, ",{}", val.unwrap_or(f64::NAN)).unwrap();
            }

            if stop {
                break;
            }
        }
    }

    pub(crate) fn set_step(&mut self, step: usize) {
        self.current_step = step;
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
