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
    RemoteAccess {
        #[command(subcommand)]
        command: RemoteAccessCommands,
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

#[derive(Debug, Subcommand)]
enum RemoteAccessCommands {
    Validate {
        #[arg(long, default_value = "examples/site")]
        config: PathBuf,
        #[arg(long)]
        json: bool,
    },
    Plan {
        #[arg(long, default_value = "examples/site")]
        config: PathBuf,
        #[arg(long)]
        json: bool,
    },
    Status {
        #[arg(long, default_value = "examples/site")]
        config: PathBuf,
        #[arg(long)]
        json: bool,
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
            Commands::RemoteAccess { command } => match command {
                RemoteAccessCommands::Validate { config, json } => {
                    let site = service.load_site(config)?;
                    let report = service.validate_site(&site);
                    let remote_issues = report
                        .issues
                        .iter()
                        .filter(|issue| issue.path.starts_with("remote_access."))
                        .cloned()
                        .collect::<Vec<_>>();

                    if json {
                        println!("{}", serde_json::to_string_pretty(&remote_issues)?);
                    } else if remote_issues.is_empty() {
                        println!("remote access validation: ok");
                    } else {
                        for issue in &remote_issues {
                            println!("[{:?}] {}: {}", issue.severity, issue.path, issue.message);
                        }
                    }

                    if remote_issues.iter().any(|issue| {
                        matches!(issue.severity, crate::validate::IssueSeverity::Error)
                    }) {
                        bail!("remote access validation failed");
                    }
                }
                RemoteAccessCommands::Plan { config, json } => {
                    let site = service.load_site(config)?;
                    let report = service.validate_site(&site);
                    if !report.is_valid() {
                        bail!("cannot plan remote access for invalid config");
                    }

                    let plan = service
                        .plan_remote_access(&site)
                        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
                    if json {
                        println!("{}", serde_json::to_string_pretty(&plan)?);
                    } else {
                        println!("planned publications: {}", plan.publications.len());
                        println!("planned DNS records: {}", plan.dns_records.len());
                        println!("planned WAN syncs: {}", plan.wan_updates.len());
                    }
                }
                RemoteAccessCommands::Status { config, json } => {
                    let site = service.load_site(config)?;
                    let report = service.validate_site(&site);
                    if !report.is_valid() {
                        bail!("cannot inspect remote access status for invalid config");
                    }

                    let status = service
                        .remote_access_status(&site)
                        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
                    if json {
                        println!("{}", serde_json::to_string_pretty(&status)?);
                    } else if status.is_empty() {
                        println!("remote access status: no configured publications");
                    } else {
                        for entry in &status {
                            println!(
                                "[{:?}] {} {}: {}",
                                entry.status, entry.provider, entry.action, entry.message
                            );
                        }
                    }
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
