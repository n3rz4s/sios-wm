use std::{
    fs,
    process::Command,
    path::PathBuf,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

pub struct ConfigCompiler {
    source: PathBuf,
    config_api_source: PathBuf,
    shader_api_source: PathBuf,
    output: PathBuf,
}

impl ConfigCompiler {
    pub fn new() -> Self {
        Self {
            source: PathBuf::from("/etc/home/.config/sios-wm/config.rs"),
            config_api_source: PathBuf::from("/usr/lib/sios-wm-api/libsios_wm_api.rlib"),
            shader_api_source: PathBuf::from("/usr/lib/sios-wm-shader-api/libsios_wm_shader_api.rlib"),
            output: PathBuf::from("/usr/lib/sios-wm-api/config.so"),
        }
    }

    pub fn compile(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_nanos();

        let build_dir = PathBuf::from(format!(
            "/tmp/sios-wm-config-{}",
            timestamp,
        ));

        fs::create_dir_all(build_dir.join("src"))?;

        let cargo_toml = r#"
        [package]
        name = "sios-wm-config"
        version = "0.1.0"
        edition = "2024"

        [lib]
        crate-type = ["cdylib"]

        [dependencies]
        "#;

        fs::write(
            build_dir.join("Cargo.toml"),
            cargo_toml,
        )?;

        fs::copy(
            &self.source,
            build_dir.join("src/lib.rs"),
        )?;

        let output = Command::new("cargo")
            .arg("rustc")
            .arg("--release")
            .arg("--")
            .arg("--extern")
            .arg(
                format!(
                    "sios_wm_api={}",
                    self.config_api_source.display(),
                ),
            )
            .arg("--extern")
            .arg(
                format!(
                    "sios_wm_shader_api={}",
                    self.shader_api_source.display(),
                ),
            )
            .current_dir(&build_dir)
            .output()?;

        if !output.status.success() {
            let _ = fs::remove_dir_all(&build_dir);

            println!("cargo stdout: \n");
            println!("{}", String::from_utf8_lossy(&output.stdout));

            println!("cargo stderr: \n");
            println!("{}", String::from_utf8_lossy(&output.stderr));

            return Err(
                "SIOS Config compilation failed.".into(),
            );
        }

        let build_library = build_dir
            .join("target/release/libsios_wm_config.so");

        if !build_library.exists() {
            let _ = fs::remove_dir_all(&build_dir);
            return Err(
                "SIOS Config library was not produced.".into(),
            );
        }

        fs::create_dir_all(
            self.output.parent().unwrap(),
        )?;

        fs::copy(
            &build_library,
            &self.output,
        )?;

        fs::remove_dir_all(&build_dir)?;

        Ok(self.output.clone())
    }
}