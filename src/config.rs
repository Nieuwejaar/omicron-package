// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Configuration for a package.

use crate::package::Package;
use serde_derive::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;
use thiserror::Error;

/// Describes the configuration for a set of packages.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// Path to the packages to be installed.
    #[serde(default, rename = "package")]
    pub packages: BTreeMap<String, Package>,
}

/// Errors which may be returned when parsing the server configuration.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Cannot parse toml: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Parses a manifest into a package [`Config`].
pub fn parse_manifest(manifest: &str) -> Result<Config, ParseError> {
    let cfg = toml::from_str::<Config>(manifest)?;
    Ok(cfg)
}
/// Parses a path in the filesystem into a package [`Config`].
pub fn parse<P: AsRef<Path>>(path: P) -> Result<Config, ParseError> {
    let contents = std::fs::read_to_string(path.as_ref())?;
    parse_manifest(&contents)
}
