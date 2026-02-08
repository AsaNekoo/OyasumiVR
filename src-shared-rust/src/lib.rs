use std::{fs, path::PathBuf, sync::LazyLock};

use xdg::BaseDirectories;
pub static XR_BINDING_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| get_config_path().join(PathBuf::from("bindings_config.toml")));
static BASE_DIRS: LazyLock<BaseDirectories> = LazyLock::new(|| BaseDirectories::new());
pub fn get_config_path() -> PathBuf {
    let mut path = BASE_DIRS
        .get_config_home()
        .expect("failed to get XDG_CONFIG_HOME");
    path.push("oyasumi");
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }
    path
}
pub fn get_log_path() -> PathBuf {
    let mut path = BASE_DIRS
        .get_data_home()
        .expect("failed to get XDG_DATA_HOME");
    path.push("co.raphii.oyasumi/logs/");
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
    path
}
pub static RESOURCES_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    if PathBuf::from("resources/sidecars/cef/libcef.so").exists() {
        let path = fs::canonicalize(PathBuf::from("resources")).unwrap();
        log::info!("using resource path: {:#?}", path);
        return path;
    }
    if let Some(mut path) = BASE_DIRS.get_data_home() {
        path=path.join("co.raphii.oyasumi/resources");
        if path.exists() {
            log::info!("using resource path: {:#?}", path);
            return path;
        }
    }
    if PathBuf::from("/usr/share/oyasumi/sidecars/cef/libcef.so").exists() {
        let path = PathBuf::from("/usr/share/oyasumi");
        log::info!("using resource path: {:#?}", path);
        return path;
    }

    panic!(
        "can not find the resources folder it has to be either next to executable or /usr/share/oyasumi or $XDG_DATA_HOME/co.raphii.oyasumi/resources"
    );
});
