# `jj-check` - To check local `git`+`jj` repos.

# Status: Pre-Pre-Alpha

Nascent work in progress, nowhere near ready for general use.
Most work happens in `dev` branch with frequent history rewrites, then pushed to `main` branch with more stable history.

# TODOs / ROADMAP

- [ ] Rename package and repo to `jj-check`
- [ ] Inline TODOs in code
- [ ] Snapshot the workspace before loading the repo (unless `--no-snapshot` is passed)
- [ ] Add example repo based tests for each rule check
- [ ] Add unit tests where useful
- [ ] Balk at jj-only or git-only repos
- [ ] Choose whether to load jj config (global and per-repo)
- [ ] Add git checks, including upstreams for branches, and pull vendored-libgit2 back in
- [ ] Add global TOML configuration file
- [ ] Add TOML configuration per-repo (in jj's toml file or in a separate file), generally overriding global config
- [ ] Handle Windows OS case re `exec` call: fail fast with guidance to add the jj-check subcommand, or use spawn.

