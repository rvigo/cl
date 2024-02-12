use cargo_metadata::{semver::Version as CargoVersion, MetadataCommand, Package as CargoPackage};
use once_cell::sync::Lazy;

static METADATA: Lazy<Metadata> = Lazy::new(Metadata::load);
pub static MAIN_PACKAGE_METADATA: Lazy<Package> = Lazy::new(|| METADATA.package.to_owned());

const PKG_NAME: &str = "cl";

pub struct Metadata {
    package: Package,
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
    pub fn load() -> Metadata {
        Metadata {
            package: Self::extract_package(),
        }
    }

    fn extract_package() -> Package {
        let metadata = MetadataCommand::new()
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .exec()
            .expect("Failed to retrieve metadata");

        let packages = metadata
            .workspace_packages()
            .into_iter()
            .filter(|package| PKG_NAME == package.name.as_str())
            .map(|package| Package::from(package.to_owned()))
            .collect::<Vec<_>>();

        // TODO since Package::default doesnt work, it should be lazy, but how?
        let default = Package {
            name: PKG_NAME.to_owned(),
            version: Version::default(),
        };

        let packages = packages.first().unwrap_or(&default).to_owned();

        packages.to_owned()
    }
}
