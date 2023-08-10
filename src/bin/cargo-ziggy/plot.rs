use crate::{find_target, Plot};
use anyhow::Result;
use std::{env, process};

impl Plot {
    pub fn generate_plot(&mut self) -> Result<(), anyhow::Error> {
        eprintln!("Generating plot");

        self.target = find_target(&self.target)?;

        // The cargo executable
        let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));

        let fuzzer_data_dir = format!("./output/{}/afl/{}/", &self.target, &self.input);
        let fuzzer_output_dir = self
            .output
            .display()
            .to_string()
            .replace("{target_name}", &self.target);

        // We run the afl-plot command
        process::Command::new(cargo)
            .args(["afl", "plot", &fuzzer_data_dir, &fuzzer_output_dir])
            .spawn()?
            .wait()?;

        Ok(())
    }
}
