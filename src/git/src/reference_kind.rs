/// Represents the kind of a reference
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum ReferenceKind {
	/// Reference is a branch.
	Branch,
	/// Reference is a note.
	Note,
	/// Reference is a remote.
	Remote,
	/// Reference is a tag.
	Tag,
	/// Reference is another kind.
	Other,
}

impl ReferenceKind {
	pub(crate) fn from(reference: &git2::Reference<'_>) -> Self {
		if reference.is_branch() {
			Self::Branch
		}
		else if reference.is_note() {
			Self::Note
		}
		else if reference.is_remote() {
			Self::Remote
		}
		else if reference.is_tag() {
			Self::Tag
		}
		else {
			Self::Other
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testutil::{with_temp_repository, JAN_2021_EPOCH};

	#[test]
	fn from_git2_reference_branch() {
		with_temp_repository(|repository| {
			assert_eq!(
				ReferenceKind::from(&repository.repository().lock().find_reference("refs/heads/main")?),
				ReferenceKind::Branch
			);
			Ok(())
		});
	}

	#[test]
	fn from_git2_reference_note() {
		with_temp_repository(|repository| {
			let git2_repository = repository.repository();
			let git2_lock = git2_repository.lock();
			let sig = git2::Signature::new("name", "name@example.com", &git2::Time::new(JAN_2021_EPOCH, 0))?;
			let head_id = git2_lock.refname_to_id("HEAD")?;
			let _ = git2_lock.note(&sig, &sig, None, head_id, "note", false)?;
			assert_eq!(
				ReferenceKind::from(&git2_lock.find_reference("refs/notes/commits")?),
				ReferenceKind::Note
			);
			Ok(())
		});
	}

	#[test]
	fn from_git2_reference_remote() {
		with_temp_repository(|repository| {
			let git2_repository = repository.repository();
			let git2_lock = git2_repository.lock();
			let mut remote = git2_lock.remote("origin", git2_lock.path().to_str().unwrap())?;
			let _ = remote.fetch(&["main"], None, None)?;
			assert_eq!(
				ReferenceKind::from(&git2_lock.find_reference("refs/remotes/origin/main")?),
				ReferenceKind::Remote
			);
			Ok(())
		});
	}

	#[test]
	fn from_git2_reference_tag() {
		with_temp_repository(|repository| {
			let git2_repository = repository.repository();
			let git2_lock = git2_repository.lock();
			let sig = git2::Signature::new("name", "name@example.com", &git2::Time::new(JAN_2021_EPOCH, 0))?;
			let head_id = git2_lock.revparse_single("HEAD")?;
			let _ = git2_lock.tag("tag", &head_id, &sig, "note", false)?;
			assert_eq!(
				ReferenceKind::from(&git2_lock.find_reference("refs/tags/tag")?),
				ReferenceKind::Tag
			);
			Ok(())
		});
	}

	#[test]
	fn from_git2_reference_other() {
		with_temp_repository(|repository| {
			let git2_repository = repository.repository();
			let git2_lock = git2_repository.lock();
			let blob = git2_lock.blob("foo".as_bytes())?;
			let _ = git2_lock.reference("refs/blob", blob, false, "blob")?;
			assert_eq!(
				ReferenceKind::from(&git2_lock.find_reference("refs/blob")?),
				ReferenceKind::Other
			);
			Ok(())
		});
	}
}
