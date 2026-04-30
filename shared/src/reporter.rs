//! Defines the [`Reporter`] trait, as well as any auxiliary types.

/// Implemented by types the report a task's progress.
pub trait Reporter {
    /// Called when the task starts.
    ///
    /// `task_size` is the expected amount of items to be processed. Note that
    /// the task may process more or less than this amount.
    fn report_init(&mut self, task_size: usize);

    /// Called when items are processed.
    fn report_update(&mut self, update: Update);

    /// Called when the task finishes.
    ///
    /// If `error` is [`Some`], the task failed; otherwise, the task succeeded.
    fn report_completed(self, error: Option<String>);
}

/// An extension of [`Iterator`] for functions that have to do
/// with [`Reporter`]s.
pub trait IteratorReporterExt
where
    Self: Iterator + Sized,
{
    /// Converts this iterator into one that updates the provided [`Reporter`]
    /// after each non-[`None`] iteration.
    fn report<'r, R: Reporter>(self, reporter: &'r mut R) -> ReporterIter<'r, Self, R> {
        ReporterIter {
            inner: self,
            reporter,
        }
    }
}

impl<I: Iterator + Sized> IteratorReporterExt for I {}

/// The type created by [`IteratorReportExt::report`].
pub struct ReporterIter<'r, I: Iterator, R: Reporter> {
    inner: I,
    reporter: &'r mut R,
}

impl<I: Iterator, R: Reporter> Iterator for ReporterIter<'_, I, R> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next();

        if item.is_some() {
            self.reporter.report_update(Update::Processed(1));
        }

        item
    }
}

/// The possible kinds of update [`Reporter::report_update`] can receive.
pub enum Update {
    /// An amount of items were processed.
    Processed(usize),
    /// The task has sent a message to be displayed to the user.
    Message(String),
}
