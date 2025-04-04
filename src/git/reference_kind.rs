/// Represents the kind of a reference
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ReferenceKind {
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
	use crate::test_helpers::{JAN_2021_EPOCH, with_temp_repository};

	#[test]
	fn from_git2_reference_branch() {
		with_temp_repository(|repository| {
			assert_eq!(
				ReferenceKind::from(&repository.repository().find_reference("refs/heads/main").unwrap()),
				ReferenceKind::Branch
			);
		});
	}

	#[test]
	fn from_git2_reference_note() {
		with_temp_repository(|repository| {
			let repo = repository.repository();
			let sig = git2::Signature::new("name", "name@example.com", &git2::Time::new(JAN_2021_EPOCH, 0)).unwrap();
			let head_id = repo.refname_to_id("HEAD").unwrap();
			_ = repo.note(&sig, &sig, None, head_id, "note", false).unwrap();
			assert_eq!(
				ReferenceKind::from(&repo.find_reference("refs/notes/commits").unwrap()),
				ReferenceKind::Note
			);
		});
	}

	#[test]
	fn from_git2_reference_remote() {
		with_temp_repository(|repository| {
			let repo = repository.repository();
			let mut remote = repo.remote("origin", repo.path().to_str().unwrap()).unwrap();
			remote.fetch(&["main"], None, None).unwrap();
			assert_eq!(
				ReferenceKind::from(&repo.find_reference("refs/remotes/origin/main").unwrap()),
				ReferenceKind::Remote
			);
		});
	}

	#[test]
	fn from_git2_reference_tag() {
		with_temp_repository(|repository| {
			let repo = repository.repository();
			let sig = git2::Signature::new("name", "name@example.com", &git2::Time::new(JAN_2021_EPOCH, 0)).unwrap();
			let head_id = repo.revparse_single("HEAD").unwrap();
			_ = repo.tag("tag", &head_id, &sig, "note", false).unwrap();
			assert_eq!(
				ReferenceKind::from(&repo.find_reference("refs/tags/tag").unwrap()),
				ReferenceKind::Tag
			);
		});
	}

	#[test]
	fn from_git2_reference_other() {
		with_temp_repository(|repository| {
			let repo = repository.repository();
			let blob = repo.blob(b"foo").unwrap();
			_ = repo.reference("refs/blob", blob, false, "blob").unwrap();
			assert_eq!(
				ReferenceKind::from(&repo.find_reference("refs/blob").unwrap()),
				ReferenceKind::Other
			);
		});
	}
}
