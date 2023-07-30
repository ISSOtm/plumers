use std::fmt::Display;

use eyre::{bail, eyre, Context};

fn main() -> Result<(), eyre::Report> {
    let mut args = std::env::args();
    let _exe_name = args.next();

    let Some(task_name) = args.next() else {
        bail!("No xtask specified! {}", ReportTaskNames)
    };

    let Some(task_fn) = TASKS
        .iter()
        .find_map(|(name, func)| (**name == task_name).then_some(func))
        else {
            bail!("error: Unknown xtask \"{task_name}\". {}", ReportTaskNames)
        };

    task_fn(args).wrap_err(eyre!("xtask \"{task_name}\" failed"))
}

type TaskFn = fn(std::env::Args) -> Result<(), eyre::Report>;
mod update_libplum;

static TASKS: [(&str, TaskFn); 1] = [("update_libplum", update_libplum::update_libplum)];
struct ReportTaskNames;
impl Display for ReportTaskNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The only supported task at the moment is \"update_libplum\"."
        )
    }
}
