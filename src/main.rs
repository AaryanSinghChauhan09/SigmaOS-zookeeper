use anyhow::Result;
use clap::Parser;
use zookeeper::zookeeper;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    cluster: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = zookeeper::Config { 
        cluster_mode: cli.cluster, 
        ..Default::default() 
    };
    let mut component = zookeeper::new(config);
    
    component.initialize()?;
    component.deploy()?;
    
    println!("SigmaOS-zookeeper deployed");
    Ok(())
}
