use git2::{Repository, Signature, Time};

use crate::test_helpers::JAN_2021_EPOCH;

pub(crate) fn create_repository(repo: &Repository) {
	let id = repo.index().unwrap().write_tree().unwrap();
	let tree = repo.find_tree(id).unwrap();
	let sig = Signature::new("name", "name@example.com", &Time::new(JAN_2021_EPOCH, 0)).unwrap();
	_ = repo
		.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[])
		.unwrap();
}
