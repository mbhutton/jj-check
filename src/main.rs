use std::os::unix::process::CommandExt;
use jj_cli::cli_util::{CliRunner, CommandHelper, WorkspaceCommandHelper};
use jj_lib::config::StackedConfig;
use jj_lib::op_walk;
use jj_lib::repo::StoreFactories;
use jj_lib::settings::UserSettings;
use jj_lib::workspace::{
    DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory, default_working_copy_factories,
};

mod rules;
use rules::{RepoRule, CompositeRepoRule, DummyRepoRule};

mod command;
use command::{CustomCommand, run_custom_command};

pub fn main() -> std::process::ExitCode {
    #[cfg(unix)]
    {
        println!("Running on Unix-like system, executing example...");
    }
    #[cfg(not(unix))]
    {
        println!("Running on non Unix-like system, executing example...");
    }
  let first_arg = std::env::args().nth(1).unwrap_or_default();
    println!("first_arg: {}", first_arg);
    if first_arg == "check-repo" {
        println!("running cli runner. all args: {:?}", std::env::args().collect::<Vec<_>>());
        CliRunner::init()
            .add_subcommand(command::run_custom_command)
            .run()
            .into()
    } else {
        // Re-call ourself via exec, pre-pending "check-repo" to the args
        println!("recalling ourselve. all args: {:?}", std::env::args().collect::<Vec<_>>());
        let mut args = std::env::args().collect::<Vec<_>>();
        args.insert(1, "check-repo".to_string());
        args.remove(0);
        // Set command to use the same binary used to run this program
        let binary = std::env::current_exe().unwrap();
        // Call exec to replace the current process with the new command
        let err = std::process::Command::new(binary)
            .args(&args).exec();

        panic!("Failed to exec command: {}", err);
    }

}

fn mainy() -> Result<(), Box<dyn std::error::Error>> {
    // Load repo in current directory using default settings...
    let current_dir = std::env::current_dir()?;
    let default_config = StackedConfig::with_defaults();
    let user_settings = UserSettings::from_config(default_config)?;
    let store_factories = StoreFactories::default();
    let working_copy_factories = default_working_copy_factories();
    let workspace_loader = DefaultWorkspaceLoaderFactory.create(&current_dir)?;
    let workspace =
        workspace_loader.load(&user_settings, &store_factories, &working_copy_factories)?;
    let repo_loader = workspace.repo_loader();

    let current_head_op = op_walk::resolve_op_for_load(repo_loader, "@")?;

    let readonly_repo = repo_loader.load_at(&current_head_op)?;

    // Example: Compose and run rule checks
    let mut root_rule = CompositeRepoRule::new();
    root_rule.add_rule(Box::new(DummyRepoRule::new("Check heads count")));
    // Add more rule checks here...

    root_rule.run(&readonly_repo);
    root_rule.print_results(0);
    // Optionally: print JSON
    println!("{}", root_rule.to_json());

    let repo_ro = readonly_repo.view();
    let num_heads = repo_ro.heads().len();
    let head_one = repo_ro.heads().iter().nth(0).unwrap();
    let all_refd = repo_ro.all_referenced_commit_ids();
    for commit_id in all_refd {
        println!("Referenced commit: {}", commit_id);
    }
    repo_ro.wc_commit_ids().first_key_value().map(|(id, commit_id)| {
        // Print the working copy commit ID
        println!("Working copy commit ID: {:?}:{}", id, commit_id);
    });

    Ok(())
}
