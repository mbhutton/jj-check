use std::io::Write as _;
use jj_cli::cli_util;
use jj_cli::cli_util::CliRunner;
use jj_cli::cli_util::CommandHelper;
use jj_cli::command_error::CommandError;
use jj_cli::ui::Ui;

#[derive(clap::Parser, Clone, Debug)]
pub enum CustomCommand {
    CheckRepo(CheckRepoArgs),
}

/// Check the repository
#[derive(clap::Args, Clone, Debug)]
struct CheckRepoArgs {
    /// Whether to skip snapshotting the repository first
    #[arg(long)]
    no_snapshot: bool,
}

fn get_workspace_helper(
    command_helper: &CommandHelper,
    ui: &mut Ui,
    no_snapshot: bool,
) -> cli_util::WorkspaceCommandHelper {
    if no_snapshot {
        command_helper.workspace_helper_no_snapshot(ui).unwrap()
    } else {
        command_helper.workspace_helper(ui).unwrap()
    }
}

pub fn run_custom_command(
    ui: &mut Ui,
    command_helper: &CommandHelper,
    command: CustomCommand,
) -> Result<(), CommandError> {
    match command {
        CustomCommand::CheckRepo(args) => {
            let mut workspace_command =
                get_workspace_helper(command_helper, ui, args.no_snapshot);
            writeln!(
                ui.status(),
                "Doing it!",
            )?;
            Ok(())
        }
    }
}

