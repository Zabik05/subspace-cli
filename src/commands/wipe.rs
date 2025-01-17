use color_eyre::eyre::Result;
use owo_colors::OwoColorize;
use subspace_sdk::farmer::CacheDescription;
use subspace_sdk::{Node, PlotDescription};

use crate::config::parse_config;
use crate::summary::delete_summary;
use crate::utils::{cache_directory_getter, node_directory_getter, plot_directory_getter};

/// implementation of the `wipe` command
///
/// wipes both farmer and node files (basically a fresh start)
pub(crate) async fn wipe() -> Result<()> {
    let config = match parse_config() {
        Ok(args) => Some(args),
        Err(_) => {
            println!(
                "could not read your config. Wipe will still continue... \n{}",
                "However, if you have set a custom location for your plots, you will need to \
                 manually delete your plots!"
                    .underline()
            );
            None
        }
    };
    let node_directory = node_directory_getter();
    let _ = Node::wipe(node_directory).await;

    // TODO: modify here when supporting multi-plot
    // if config can be read, delete the farmer using the path in the config, else,
    // delete the default location
    if let Some(config) = config {
        match PlotDescription::new(config.farmer.plot_directory, config.farmer.plot_size) {
            Ok(plot) => {
                let _ = plot.wipe().await;
            }
            Err(err) => println!(
                "Skipping wiping plot. Got error while constructing the plot reference: {err}"
            ),
        }
        let _ = CacheDescription::new(cache_directory_getter(), config.farmer.advanced.cache_size)?
            .wipe()
            .await;
    } else {
        let _ = tokio::fs::remove_dir_all(plot_directory_getter()).await;
    }

    delete_summary().await;

    println!("Wipe successful!");

    Ok(())
}
