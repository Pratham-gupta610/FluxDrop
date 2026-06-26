use std::env;

use anyhow::{Context, Result};

use aya::{
    programs::{Xdp, XdpFlags},
    Ebpf,
};

use tokio::signal;

use env_logger;
use log::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let iface = env::args()
        .nth(1)
        .context("Usage: fluxdrop <interface>\nExample: fluxdrop eth0")?;

    info!("FluxDrop starting on interface: {}", iface);

    let bpf_bytes = include_bytes!("../../target/bpfel-unknown-none/release/fluxdrop-ebpf");

    let mut bpf =
        Ebpf::load(bpf_bytes).context("Failed to load BPF ELF. Are you running as root?")?;

    info!("BPF ELF loaded successfully");

    let program: &mut Xdp = bpf
        .program_mut("fluxdrop_main")
        .context(
            "Program 'fluxdrop_main' not found in ELF. \
                  Check that the #[xdp] function name in ebpf/src/main.rs matches.",
        )?
        .try_into()
        .context("Program is not an XDP program")?;

    program
        .load()
        .context("BPF verifier rejected the program")?;

    info!("BPF program verified and loaded");

    let _link = program
        .attach(&iface, XdpFlags::default())
        .with_context(|| {
            format!(
                "Failed to attach XDP program to interface '{}'.\n\
             Causes: not root, interface doesn't exist, \
             or another XDP program already attached.",
                iface
            )
        })?;

    info!(
        "XDP program attached to '{}'. All packets now pass through FluxDrop.",
        iface
    );
    info!("Press Ctrl+C to detach and exit.");

    signal::ctrl_c()
        .await
        .context("Failed to set up Ctrl+C signal handler")?;

    warn!("Ctrl+C received. Detaching XDP program from '{}'...", iface);
    drop(_link);

    info!("XDP program detached. FluxDrop exited cleanly.");
    Ok(())
}
