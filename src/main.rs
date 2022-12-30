mod recipe;
mod config;

use anyhow::Result;
use crate::recipe::Recipe;
use crate::config::Config;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config: Config = Config::new(&args[1..]);

    let recipe = Recipe::parse(config.step.clone())?;
    recipe.execute(&config.flags[..])
}
