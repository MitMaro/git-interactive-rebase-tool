use crate::{git::Repository, test_helpers::JAN_2021_EPOCH};

pub(crate) fn create_repository(repo: git2::Repository) -> Repository {
	{
		let id = repo.index().unwrap().write_tree().unwrap();
		let tree = repo.find_tree(id).unwrap();
		let sig = git2::Signature::new("name", "name@example.com", &git2::Time::new(JAN_2021_EPOCH, 0)).unwrap();
		_ = repo
			.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[])
			.unwrap();
	};
	Repository::from(repo)
}
