use crate::Build;
use anyhow::{anyhow, Context, Result};
use console::style;
use std::{env, process};

impl Build {
    /// Build the fuzzers
    pub fn build(&self) -> Result<(), anyhow::Error> {
        // No fuzzers for you
        if self.no_afl && self.no_honggfuzz {
            return Err(anyhow!("Pick at least one fuzzer"));
        }

        // The cargo executable
        let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));
        info!("Starting build command");

        if !self.no_afl {
            eprintln!("    {} afl", style("Building").red().bold());

            // Second fuzzer we build: AFL++
            let run = process::Command::new(cargo.clone())
                .args([
                    "afl",
                    "build",
                    "--features=ziggy/afl",
                    "--target-dir=target/afl",
                ])
                .env("AFL_QUIET", "1")
                .env("RUSTFLAGS", env::var("RUSTFLAGS").unwrap_or_default())
                .spawn()?
                .wait()
                .context("Error spawning afl build command")?;

            if !run.success() {
                return Err(anyhow!(
                    "Error building afl fuzzer: Exited with {:?}",
                    run.code()
                ));
            }

            eprintln!("    {} afl", style("Finished").cyan().bold());
        }

        if !self.no_honggfuzz {
            eprintln!("    {} honggfuzz", style("Building").red().bold());

            // Third fuzzer we build: Honggfuzz
            let run = process::Command::new(cargo)
                .args(["hfuzz", "build"])
                .env("CARGO_TARGET_DIR", "./target/honggfuzz")
                .env("HFUZZ_BUILD_ARGS", "--features=ziggy/honggfuzz")
                .env("RUSTFLAGS", env::var("RUSTFLAGS").unwrap_or_default())
                .stdout(process::Stdio::piped())
                .spawn()?
                .wait()
                .context("Error spawning hfuzz build command")?;

            if !run.success() {
                return Err(anyhow!(
                    "Error building honggfuzz fuzzer: Exited with {:?}",
                    run.code()
                ));
            }

            eprintln!("    {} honggfuzz", style("Finished").cyan().bold());
        }
        Ok(())
    }
}
