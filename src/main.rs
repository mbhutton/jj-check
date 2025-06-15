use jj_lib::config::StackedConfig;
use jj_lib::op_walk;
use jj_lib::repo::StoreFactories;
use jj_lib::settings::UserSettings;
use jj_lib::workspace::{
    DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory, default_working_copy_factories,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load repo in current directory using default settings...
    let current_dir = std::env::current_dir()?;
    // TODO:Consider loading global and local configuration files rather than using defaults.
    let default_config = StackedConfig::with_defaults();
    let user_settings = UserSettings::from_config(default_config)?;
    let store_factories = StoreFactories::default();
    let working_copy_factories = default_working_copy_factories();
    let workspace_loader = DefaultWorkspaceLoaderFactory.create(&current_dir)?;
    let workspace =
        workspace_loader.load(&user_settings, &store_factories, &working_copy_factories)?;
    let repo_loader = workspace.repo_loader();

    // TODO: See if a more idiomatic way to get the current head operation.
    let current_head_op = op_walk::resolve_op_for_load(repo_loader, "@")?;
    // TODO:Snapshot the workspace before loading the repo.

    let readonly_repo = repo_loader.load_at(&current_head_op)?;

    let num_heads = readonly_repo.view().heads().len();
    println!("Number of heads: {}", num_heads);

    Ok(())
}
