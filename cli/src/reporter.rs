//! Defines this crate's [`Reporter`] implementation.

use std::fmt::Display;

use indicatif::ProgressBar;
use shared::reporter::{Reporter, Update};

/// A [`Reporter`] that displays progress through a [`ProgressBar`].
pub struct CliReporter {
    bar: ProgressBar,
    title: String,
}

impl CliReporter {
    /// Creates a new [`CliReporter`].
    pub fn new(bar: ProgressBar, title: &str) -> Self {
        Self {
            bar,
            title: title.into(),
        }
    }

    fn set_message(&self, msg: impl Display) {
        self.bar.set_message(format!("{:12} : {msg}", self.title));
    }

    fn finish_with_message(&self, msg: impl Display) {
        self.bar
            .finish_with_message(format!("{:12} : {msg}", self.title));
    }
}

impl Reporter for CliReporter {
    fn report_init(&mut self, task_size: usize) {
        self.bar.set_length(task_size as u64);
    }

    fn report_update(&mut self, update: Update) {
        match update {
            Update::Processed(delta) => self.bar.inc(delta as u64),
            Update::Message(msg) => self.set_message(msg),
        }
    }

    fn report_completed(self, error: Option<String>) {
        match error {
            None => self.finish_with_message("All done!"),
            Some(msg) => self.finish_with_message(msg),
        }
    }
}
