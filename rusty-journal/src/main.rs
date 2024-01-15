use structopt::StructOpt;
mod cli;
mod tasks;

use tasks::Task;
use cli::{Action::*, CommandLineArgs};

fn main() {
    //  Get the command-line aargument
    let CommandLineArgs {
        action,
        journal_file,
      } = CommandLineArgs::from_args();

    // Unpack the journal filer
    let journal_file = journal_file.expect("Failed to find journal file");

    // Perform the action
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        Done { position } => tasks::complete_task(journal_file, position),
        List => tasks::list_tasks(journal_file),
    }
    .expect("Failed to perform action");

    // cli::CommandLineArgs::from_args();
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
