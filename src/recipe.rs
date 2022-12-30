use anyhow::anyhow;
use anyhow::Result;
use std::fmt::{Display, Formatter};
use std::process::Command;

const SH_COMMAND_FLAG: &str = "-c";

#[derive(Debug)]
pub struct Recipe {
    value: String,
}

impl Recipe {
    pub fn parse(step: String) -> Result<Recipe> {
        let print_recipe = Command::new("make")
            .arg(step)
            .arg("--just-print")
            .output()?;

        let recipe = String::from_utf8_lossy(&print_recipe.stdout);

        if recipe.trim().is_empty() {
            return Err(anyhow!("{}", String::from_utf8_lossy(&print_recipe.stderr).trim()));
        }

        Ok(Recipe { value: recipe.trim().to_string() })
    }

    pub fn execute(&self, flags: &[String]) -> Result<()> {
        let modified_recipe = self.add_flags(flags);

        println!("{}", modified_recipe);

        Command::new("sh")
            .arg(SH_COMMAND_FLAG)
            .arg(modified_recipe.to_string())
            .status()?;

        Ok(())
    }

    fn add_flags(&self, flags: &[String]) -> Self {
        let modified_recipe = [vec![self.value.clone()], flags.to_vec()].concat();

        Recipe {
            value: modified_recipe.join(" ")
        }
    }
}

impl Display for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
