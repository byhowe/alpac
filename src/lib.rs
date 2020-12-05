mod hash;

pub use hash::Hasher;

use curl::easy::Easy;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::path::Path;
use url::Url;

#[derive(Debug)]
pub enum RecipeVersion
{
  Latest,
  Version(String),
}

#[derive(Debug, Deserialize)]
pub struct Source
{
  /// Url of the file to download.
  url: Url,

  /// Size of the file in bytes. Used to initialize the vector with enough capacity so it does not
  /// allocate during runtime.
  size: Option<u64>,

  md5: Option<String>,
  sha1: Option<String>,
  sha224: Option<String>,
  sha256: Option<String>,
  sha384: Option<String>,
  sha512: Option<String>,
}

impl Source
{
  /// Extracts filename from the url.
  pub fn get_filename(&self) -> &str
  {
    self.url.path_segments().unwrap().last().unwrap()
  }

  /// Download the file from the url specified in the `url` field. It tries to calculate the
  /// hashes for this file while downloading.
  pub fn download(&self) -> Option<Vec<u8>>
  {
    // NOTE: Some files might be really big. We may not want to store all that data in memory.
    // Maybe try to write some of that data to disk when it gets too much?
    let mut data = if let Some(size) = self.size {
      Vec::with_capacity(size as usize)
    } else {
      Vec::new()
    };

    let mut hasher = Hasher::new()
      .md5(self.md5.clone())
      .sha1(self.sha1.clone())
      .sha224(self.sha224.clone())
      .sha256(self.sha256.clone())
      .sha384(self.sha384.clone())
      .sha512(self.sha512.clone());

    let mut handle = Easy::new();
    handle.url(self.url.as_str()).unwrap();
    {
      let mut transfer = handle.transfer();
      transfer
        .write_function(|new_data| {
          data.extend_from_slice(new_data);

          hasher.update(new_data);

          Ok(new_data.len())
        })
        .unwrap();
      transfer.perform().unwrap();
    }

    if hasher.finish() {
      Some(data)
    } else {
      None
    }
  }

  /// Download and save the file in the specified out directory.
  pub fn download_to_dir<P: AsRef<Path>>(&self, out: P) -> bool
  {
    if let Some(data) = self.download() {
      fs::write(&out.as_ref().join(self.get_filename()), &data).unwrap();
      true
    } else {
      false
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct Ingredients
{
  name: String,
  description: Option<String>,
  homepage: Option<String>,
  license: Option<String>,
  topics: Option<Vec<String>>,
  sources: Option<BTreeMap<String, Source>>,
}

impl Ingredients
{
  /// Read the `ingredients.yaml` file and convert it to a struct.
  pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, serde_yaml::Error>
  {
    serde_yaml::from_reader(File::open(path).unwrap())
  }

  /// Latest version of this package that this recipe can provide.
  pub fn get_latest_version(&self) -> Option<&str>
  {
    self
      .sources
      .as_ref()
      .map(|s| s.keys().last())
      .flatten()
      .map(|e| e.as_str())
  }

  /// All the versions that this recipe can install.
  pub fn available_versions(&self) -> Vec<&str>
  {
    // For now, we can determine available versions by looking at sources.
    self
      .sources
      .as_ref()
      .map(|s| s.keys().map(|k| k.as_str()).collect())
      .unwrap_or(Vec::new())
  }

  /// Get source information for a specific version.
  pub fn get_source(&self, recipe_version: &RecipeVersion) -> Option<&Source>
  {
    match recipe_version {
      RecipeVersion::Latest => {
        if let Some(version) = self.get_latest_version() {
          self.sources.as_ref().map(|s| s.get(version)).flatten()
        } else {
          None
        }
      }
      RecipeVersion::Version(version) => self.sources.as_ref().map(|s| s.get(version)).flatten(),
    }
  }

  /// Get source information for all versions that this recipe knows.
  pub fn get_all_sources(&self) -> Vec<&Source>
  {
    self
      .sources
      .as_ref()
      .map(|s| s.values().collect())
      .unwrap_or(Vec::new())
  }
}
