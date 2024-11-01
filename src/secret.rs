use napi;
use virt;

#[napi]
pub struct Secret {
	secret: virt::secret::Secret
}

impl Secret {
	pub fn get(&self) -> &virt::secret::Secret {
		&self.secret
	}

	pub fn from_secret(secret: virt::secret::Secret) -> Self {
		Self { secret: secret }
	}
}