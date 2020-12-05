use openssl::{
  hash::MessageDigest,
  sha::{Sha1, Sha224, Sha256, Sha384, Sha512},
};

macro_rules! impl_sha {
  ($sha:ident, $arg:ident, $hasher:ident) => {
    impl Hasher
    {
      pub fn $sha(mut self, $arg: Option<String>) -> Self
      {
        if let Some(hash) = $arg {
          self.$sha = Some(($hasher::new(), hash));
        }
        self
      }
    }
  };
}

macro_rules! finish_sha {
  ($verified:ident, $hasher:expr) => {
    $hasher.map(|(hasher, expected)| {
      if $verified {
        $verified = hasher.finish().as_ref().eq(hex::decode(&expected).as_ref().unwrap())
      }
    })
  };
}

pub struct Hasher
{
  md5: Option<(openssl::hash::Hasher, String)>,
  sha1: Option<(Sha1, String)>,
  sha224: Option<(Sha224, String)>,
  sha256: Option<(Sha256, String)>,
  sha384: Option<(Sha384, String)>,
  sha512: Option<(Sha512, String)>,
}

impl Hasher
{
  pub fn new() -> Self
  {
    Self {
      md5: None,
      sha1: None,
      sha224: None,
      sha256: None,
      sha384: None,
      sha512: None,
    }
  }

  pub fn md5(mut self, expected_md5: Option<String>) -> Self
  {
    if let Some(hash) = expected_md5 {
      self.md5 = Some((openssl::hash::Hasher::new(MessageDigest::md5()).unwrap(), hash));
    }
    self
  }

  pub fn update<D: AsRef<[u8]>>(&mut self, data: D)
  {
    self
      .md5
      .as_mut()
      .map(|(hasher, _)| hasher.update(data.as_ref()).unwrap());
    self.sha1.as_mut().map(|(hasher, _)| hasher.update(data.as_ref()));
    self.sha224.as_mut().map(|(hasher, _)| hasher.update(data.as_ref()));
    self.sha256.as_mut().map(|(hasher, _)| hasher.update(data.as_ref()));
    self.sha384.as_mut().map(|(hasher, _)| hasher.update(data.as_ref()));
    self.sha512.as_mut().map(|(hasher, _)| hasher.update(data.as_ref()));
  }

  pub fn finish(mut self) -> bool
  {
    let mut verified = true;

    self.md5.as_mut().map(|(hasher, expected)| {
      verified = hasher
        .finish()
        .unwrap()
        .as_ref()
        .eq(hex::decode(&expected).as_ref().unwrap())
    });

    finish_sha!(verified, self.sha1);
    finish_sha!(verified, self.sha224);
    finish_sha!(verified, self.sha256);
    finish_sha!(verified, self.sha384);
    finish_sha!(verified, self.sha512);

    verified
  }
}

impl_sha!(sha1, expected_sha1, Sha1);
impl_sha!(sha224, expected_sha224, Sha224);
impl_sha!(sha256, expected_sha256, Sha256);
impl_sha!(sha384, expected_sha384, Sha384);
impl_sha!(sha512, expected_sha512, Sha512);
