use std::io::Read;

use crate::config::Config;
use subprocess::{Popen, PopenConfig, Redirection};

pub fn clone(cfg: &Config) -> Result<String, String> {
    let mut proc = match Popen::create(
        &["git", "clone", &cfg.repo.remote, "--depth=1"],
        PopenConfig {
            cwd: Some(cfg.directory.to_owned().into()),
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    ) {
        Ok(val) => val,
        Err(_) => return Err("Could Not Start Clone!!".into()),
    };

    let exit_code = match proc.wait() {
        Ok(val) => val,
        Err(_) => return Err("Clone Faied Unexpectly!!".into()),
    };

    if !exit_code.success() {
        return Err("Clone exited unsucessfully".into());
    }

    // Git outputs "Cloning into 'dir'..." message into stderr
    let mut stderr = match proc.stderr.as_ref() {
        Some(val) => val,
        None => todo!(),
    };
    let mut name = String::new();
    stderr.read_to_string(&mut name).unwrap();
    let name: String = name.splitn(3, "'").collect::<Vec<&str>>()[1].into();
    return Ok(name);
}
