const FLAG_START: &str = "-";

pub struct Config {
    pub(crate) step: String,
    pub(crate) flags: Vec<String>
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let step = args[0].to_string();
        let flags = args[1..].iter()
            .filter(|arg| arg.starts_with(FLAG_START))
            .map(Clone::clone)
            .collect();

        Config { step, flags }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_step_and_flags() {
        let args = ["a-step".to_string(), "-f".to_string(), "--a-flag".to_string()];
        let config = Config::new(&args);

        assert_eq!(config.step, "a-step");
        assert_eq!(config.flags, ["-f", "--a-flag"]);
    }
}