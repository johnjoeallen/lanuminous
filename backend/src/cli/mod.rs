use std::{fs, net::SocketAddr, path::PathBuf};

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

use crate::{api::build_router, app::SiteService, deploy::DeploymentPlanner};

#[derive(Debug, Parser)]
#[command(name = "lantricate")]
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
        #[arg(long, default_value = "127.0.0.1:3000")]
        listen: SocketAddr,
    },
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let service = SiteService;

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
            Commands::Serve { config, listen } => {
                let site = service.load_site(config)?;
                let report = service.validate_site(&site);
                if !report.is_valid() {
                    bail!("cannot serve invalid config");
                }

                let router = build_router(site);
                let listener = tokio::net::TcpListener::bind(listen).await?;
                println!("serving lantricate api on http://{listen}");
                axum::serve(listener, router).await?;
            }
        }

        Ok(())
    }
}
