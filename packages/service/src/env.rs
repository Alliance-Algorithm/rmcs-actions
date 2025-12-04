use crate::constant::env::{ENV_NAME_LOG_DIR, ENV_NAME_STORAGE_DIR};

const REQUIRED_ENV_VARS: &[&str] = &[ENV_NAME_LOG_DIR, ENV_NAME_STORAGE_DIR];

const REQUIRED_PATH_ENV_VARS: &[&str] =
    &[ENV_NAME_LOG_DIR, ENV_NAME_STORAGE_DIR];

pub fn load_env() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    for &var in REQUIRED_ENV_VARS {
        std::env::var(var).map_err(|_| {
            anyhow::anyhow!("Environment variable `{}` is not set", var)
        })?;
    }

    for &var in REQUIRED_PATH_ENV_VARS {
        let path_str = std::env::var(var).map_err(|_| {
            anyhow::anyhow!("Environment variable `{}` is not set", var)
        })?;
        let path = std::path::Path::new(&path_str);
        if !path.exists() {
            std::fs::create_dir_all(path).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to create directory for environment variable `{}` at path `{}`: {}",
                    var,
                    path_str,
                    e
                )
            })?;
        }
    }

    Ok(())
}
