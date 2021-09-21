use std::hash::Hash;

/// Types implementing this trait can be used to report values and events
/// throughout the simulation.
pub trait Reporter {
    fn report_num<T>(&mut self, target: T, value: f64)
    where
        T: ReporterTarget;

    fn report_event<T>(&mut self, target: T, value: String)
    where
        T: ReporterTarget;
}

/// When reporting a value, a target is given, this target contains information
/// about the origin of the report, as well as how the value can be used.
pub trait ReporterTarget {
    fn get_inner(&self) -> Option<&dyn ReporterTarget> {
        None
    }

    fn get_step(&self) -> Option<u32> {
        self.get_inner()?.get_step()
    }

    fn get_description(&self) -> Option<&str> {
        self.get_inner()?.get_description()
    }
}

impl ReporterTarget for &'static str {
    fn get_description(&self) -> Option<&str> {
        Some(self)
    }
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
    fn report_num<T>(&mut self, target: T, value: f64)
    where
        T: ReporterTarget,
    {
        self.inner
            .report_num(SteppedTarget(target, self.step), value);
    }

    fn report_event<T>(&mut self, target: T, value: String)
    where
        T: ReporterTarget,
    {
        self.inner
            .report_event(SteppedTarget(target, self.step), value);
    }
}

/// A target augmenting the inner target with step information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SteppedTarget<T>(T, u32);

impl<T> ReporterTarget for SteppedTarget<T>
where
    T: ReporterTarget,
{
    fn get_step(&self) -> Option<u32> {
        Some(self.1)
    }
}
