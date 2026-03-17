use std::{fs, net::SocketAddr, path::PathBuf};

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

use crate::{
    agent::HostAgentService, api::build_router, app::SiteService, deploy::DeploymentPlanner,
};

#[derive(Debug, Parser)]
#[command(name = "lanuminous")]
#[command(about = "Intent-driven network orchestration for home-lab and small-site deployments")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Validate {
        #[arg(long, default_value = "examples/site")]
        config: PathBuf,
        #[arg(long)]
        json: bool,
    },
    Render {
        #[arg(long, default_value = "examples/site")]
        config: PathBuf,
        #[arg(long)]
        out_dir: Option<PathBuf>,
    },
    Serve {
        #[arg(long, default_value = "examples/site")]
        config: PathBuf,
        #[arg(long, default_value = "127.0.0.1:9097")]
        listen: SocketAddr,
        #[arg(long)]
        ui_dir: Option<PathBuf>,
        #[arg(long, default_value = ".lanuminous")]
        state_dir: PathBuf,
    },
    Agent {
        #[command(subcommand)]
        command: AgentCommands,
    },
}

#[derive(Debug, Subcommand)]
enum AgentCommands {
    Info {
        #[arg(long, default_value = "/var/lib/lanuminous")]
        state_dir: PathBuf,
    },
    InspectStage {
        #[arg(long)]
        stage_dir: PathBuf,
    },
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let service = SiteService;
        let agent_service = HostAgentService;

        match self.command {
            Commands::Validate { config, json } => {
                let site = service.load_site(config)?;
                let report = service.validate_site(&site);

                if json {
                    println!("{}", serde_json::to_string_pretty(&report)?);
                } else if report.issues.is_empty() {
                    println!("validation: ok");
                } else {
                    for issue in &report.issues {
                        println!("[{:?}] {}: {}", issue.severity, issue.path, issue.message);
                    }
                }

                if !report.is_valid() {
                    bail!("validation failed");
                }
            }
            Commands::Render { config, out_dir } => {
                let site = service.load_site(config)?;
                let report = service.validate_site(&site);
                if !report.is_valid() {
                    bail!("cannot render invalid config");
                }

                let artifacts = service.render_site(&site)?;
                let planner = DeploymentPlanner;
                let plan = planner.plan_stage1(&artifacts);

                if let Some(out_dir) = out_dir {
                    fs::create_dir_all(&out_dir)?;
                    for artifact in &artifacts {
                        let output_path =
                            out_dir.join(format!("{}.generated", artifact.logical_name));
                        fs::write(output_path, &artifact.contents)?;
                    }
                } else {
                    println!("{}", serde_json::to_string_pretty(&artifacts)?);
                }

                println!("planned changed paths: {}", plan.changed_paths.len());
            }
            Commands::Serve {
                config,
                listen,
                ui_dir,
                state_dir,
            } => {
                let site = service.load_site(config)?;
                let report = service.validate_site(&site);
                if !report.is_valid() {
                    bail!("cannot serve invalid config");
                }

                let stage_root = state_dir.join("staging");
                fs::create_dir_all(&stage_root)?;
                let router = build_router(site, stage_root, resolve_ui_dir(ui_dir));
                let listener = tokio::net::TcpListener::bind(listen).await?;
                println!("serving lanuminous api on http://{listen}");
                axum::serve(listener, router).await?;
            }
            Commands::Agent { command } => match command {
                AgentCommands::Info { state_dir } => {
                    let descriptor = agent_service.describe(state_dir);
                    println!("{}", serde_json::to_string_pretty(&descriptor)?);
                }
                AgentCommands::InspectStage { stage_dir } => {
                    let inspection = agent_service.inspect_stage_dir(stage_dir)?;
                    println!("{}", serde_json::to_string_pretty(&inspection)?);
                }
            },
        }

        Ok(())
    }
}

fn resolve_ui_dir(explicit_ui_dir: Option<PathBuf>) -> Option<PathBuf> {
    explicit_ui_dir.or_else(|| {
        let workspace_dist = std::env::current_dir().ok()?.join("frontend/dist");
        workspace_dist.exists().then_some(workspace_dist)
    })
}
