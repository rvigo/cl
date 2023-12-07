use cargo_metadata::{semver::Version as CargoVersion, MetadataCommand, Package as CargoPackage};
use itertools::Itertools;

const PKG_NAME: &str = "cl";
const CORE_PKG_NAME: &str = "cl-core";
const CLI_PKG_NAME: &str = "cl-cli";
const GUI_PKG_NAME: &str = "cl-gui";
const PKGS_NAME: &[&str] = &[PKG_NAME, CORE_PKG_NAME, CLI_PKG_NAME, GUI_PKG_NAME];

pub struct Metadata {
    packages: Vec<Package>,
}

#[derive(Clone)]
pub struct Package {
    name: String,
    version: Version,
}

impl ToString for Package {
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.version.to_string())
    }
}

impl From<CargoPackage> for Package {
    fn from(value: CargoPackage) -> Self {
        Package {
            name: value.name,
            version: value.version.into(),
        }
    }
}

#[derive(Clone, Default)]
pub struct Version {
    patch: u64,
    minor: u64,
    major: u64,
}

impl From<CargoVersion> for Version {
    fn from(value: CargoVersion) -> Self {
        Version {
            patch: value.patch,
            minor: value.minor,
            major: value.major,
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Metadata {
    pub fn main_package_metadata(&self) -> Package {
        self.packages
            .iter()
            .cloned()
            .find_or_first(|package| package.name == PKG_NAME)
            .unwrap_or_else(|| Package {
                name: PKG_NAME.to_owned(),
                version: Version::default(),
            })
    }

    pub fn packages_metadata(&self) -> Vec<Package> {
        self.packages.clone()
    }

    pub fn load() -> Metadata {
        Metadata {
            packages: Self::extract_packages(),
        }
    }

    fn extract_packages() -> Vec<Package> {
        let metadata = MetadataCommand::new()
            .exec()
            .expect("Failed to retrieve metadata");

        let packages: Vec<Package> = metadata
            .packages
            .iter()
            .filter(|package| PKGS_NAME.contains(&package.name.as_str()))
            .map(|package| Package::from(package.to_owned()))
            .collect();

        packages
    }
}

#[macro_export]
macro_rules! metadata {
    () => {{
        use $crate::resource::metadata::Metadata;
        Metadata::load().main_package_metadata()
    }};
}

#[macro_export]
macro_rules! pkgs_metadata {
    () => {{
        use $crate::resource::metadata::Metadata;
        Metadata::load().packages_metadata()
    }};
}
