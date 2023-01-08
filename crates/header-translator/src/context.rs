use std::collections::HashMap;
use std::ops;
use std::path::{Path, PathBuf};

use apple_sdk::SdkPath;
use clang::source::Location;
use clang::Entity;

use crate::config::Config;

pub struct Context<'a> {
    config: &'a Config,
    pub macro_invocations: HashMap<Location<'a>, String>,
    framework_dir: PathBuf,
    nsobject_dir: PathBuf,
}

impl<'a> Context<'a> {
    pub fn new(config: &'a Config, sdk: &SdkPath) -> Self {
        Self {
            config,
            macro_invocations: Default::default(),
            framework_dir: sdk.path.join("System/Library/Frameworks"),
            nsobject_dir: sdk.path.join("usr/include/objc/NSObject.h"),
        }
    }

    pub fn get_library_and_file_name(&self, entity: &Entity<'_>) -> Option<(String, String)> {
        if let Some(location) = entity.get_location() {
            if let Some(file) = location.get_file_location().file {
                let path = file.get_path();
                if let Ok(path) = path.strip_prefix(&self.framework_dir) {
                    return Some(split_path(path));
                } else if path == self.nsobject_dir {
                    return Some(("Foundation".to_string(), "NSObject".to_string()));
                }
            }
        }
        None
    }
}

impl ops::Deref for Context<'_> {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.config
    }
}

fn split_path(path: &Path) -> (String, String) {
    let mut components = path.components();
    let library_name = components
        .next()
        .expect("components next")
        .as_os_str()
        .to_str()
        .expect("component to_str")
        .strip_suffix(".framework")
        .expect("framework fileending")
        .to_string();

    let path = components.as_path();
    let file_name = path
        .file_stem()
        .expect("path file stem")
        .to_string_lossy()
        .to_string();

    (library_name, file_name)
}