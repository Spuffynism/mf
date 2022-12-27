use std::process::Command;

const FLAG_START: &str = "-";

fn parse_args(args: &[String]) -> (Vec<&str>, Vec<&str>) {
    let is_a_flag = |arg: &&String| arg.starts_with(FLAG_START);

    let args_without_callee = &args[1..];

    let steps: Vec<_> = args_without_callee.iter()
        .filter(|arg| !is_a_flag(arg))
        .map(AsRef::as_ref)
        .collect();
    let flags: Vec<_> = args_without_callee.iter()
        .filter(is_a_flag)
        .map(AsRef::as_ref)
        .collect();

    (steps, flags)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let (steps, mut flags) = parse_args(&args[..]);

    let make = Command::new("make")
        .args(steps)
        .arg("--just-print")
        .output()?;

    let recipe = String::from_utf8_lossy(&make.stdout).to_string();

    if recipe.trim().is_empty() {
        println!("{}", String::from_utf8_lossy(&make.stderr).to_string().trim());
        return Ok(())
    }

    let mut modified_recipe = vec![recipe.trim()];
    modified_recipe.append(&mut flags);

    let sh = Command::new("sh")
        .arg("-c")
        .arg(modified_recipe.join(" "))
        .output()?;

    let output_bytes = if sh.status.success() { &sh.stdout } else { &sh.stderr };
    let output = String::from_utf8_lossy(&output_bytes).to_string();
    println!("{}", output.trim());

    return Ok(())
}
