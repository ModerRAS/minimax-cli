use crate::config_file::ConfigFile;
use crate::keyring;
use anyhow::Result;

pub async fn run(command: &crate::cli::ConfigCommands) -> Result<()> {
    match command {
        crate::cli::ConfigCommands::SetApiKey { key } => {
            keyring::set_api_key(key)?;
            println!("API key stored securely in keyring.");
            Ok(())
        }
        crate::cli::ConfigCommands::SetApiHost { host } => {
            let mut config = ConfigFile::load()?;
            config.api_host = host.clone();
            config.save()?;
            println!("API host set to: {}", host);
            Ok(())
        }
        crate::cli::ConfigCommands::Show => {
            // Show config (without revealing API key)
            let config_file = ConfigFile::load()?;
            let api_key_set = keyring::get_api_key().is_ok();

            println!("Configuration:");
            println!(
                "  API Key: {}",
                if api_key_set {
                    "*** set ***"
                } else {
                    "*** not set ***"
                }
            );
            println!("  API Host: {}", config_file.api_host);
            println!("  Config file: {:?}", ConfigFile::path());
            println!("  Database: {:?}", config_file.db_path);
            println!("  Output dir: {:?}", config_file.output_dir);
            Ok(())
        }
        crate::cli::ConfigCommands::Init => {
            // Interactive setup
            println!("MiniMax CLI Configuration Wizard");
            println!("================================");

            // Get API key (hidden input)
            print!("\nEnter your API key: ");
            std::io::Write::flush(&mut std::io::stdout())?;
            let api_key = rpassword::read_password()?;

            if api_key.is_empty() {
                anyhow::bail!("API key cannot be empty");
            }

            // Get API host
            println!("\nSelect API region:");
            println!("  1. Global (https://api.minimax.io)");
            println!("  2. China (https://api.minimaxi.com)");
            print!("Enter choice [1]: ");
            std::io::Write::flush(&mut std::io::stdout())?;

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice)?;
            let choice = choice.trim();

            let host = match choice {
                "2" => "https://api.minimaxi.com",
                _ => "https://api.minimax.io",
            };

            // Save API key to keyring
            keyring::set_api_key(&api_key)?;
            println!("\n✓ API key stored securely");

            // Save config file
            let mut config = ConfigFile::default();
            config.api_host = host.to_string();
            config.save()?;
            println!("✓ Config file saved");

            println!("\nConfiguration complete!");
            println!("Config file: {:?}", ConfigFile::path());

            Ok(())
        }
    }
}
