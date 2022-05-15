use std::path::PathBuf;

use directories::ProjectDirs;

lazy_static! {
    pub static ref DIRS: ProjectDirs = {
        ProjectDirs::from("", "jewelexx", "fan-control")
            .expect("failed to find project directories")
    };
    pub static ref CONFIG_DIR: PathBuf = DIRS.config_dir().into();
}
