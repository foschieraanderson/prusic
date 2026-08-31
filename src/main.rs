mod config;

use crate::config::load_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    println!("Volume: {}", config.player.volume);
    println!("Spectrum: {}", config.spectrum.enabled);
    println!("Color: {}", config.appearance.color);

    Ok(())
}
