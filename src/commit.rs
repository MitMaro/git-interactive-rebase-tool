#[derive(PartialEq, Debug)]
pub struct FileStat {
	name: String,
	added: String,
	removed: String
}

#[derive(PartialEq, Debug)]
pub struct Commit {
	author_name: String,
	author_email: String,
	date: String,
	subject: String,
	body: String,
	file_stats: Vec<FileStat>
}

impl FileStat {
	pub fn new(file_stats: &str) -> Result<Self, String> {
		let input: Vec<&str> = file_stats.splitn(3, '\t').collect();
		
		match input.len() {
			3 => Ok(FileStat {
				name: String::from(input[2]),
				added: String::from(input[0]),
				removed: String::from(input[1])
			}),
			_ => Err(format!(
				"Invalid file stat result:\n{}", file_stats
			))
		}
	}
	
	pub fn get_added(&self) -> &String {
		&self.added
	}
	
	pub fn get_removed(&self) -> &String {
		&self.removed
	}
	
	pub fn get_name(&self) -> &String {
		&self.name
	}
}

impl Commit {
	pub fn new(commit_stat: &str) -> Result<Self, String> {
		let input: Vec<&str> = commit_stat.splitn(6, '').collect();
		match input.len() {
			6 => {
				let file_stats = input[5].lines()
					.filter(|l| !l.is_empty())
					.map(|l| FileStat::new(l))
					.collect();
				match file_stats {
					Ok(stats) => Ok(Commit {
						author_name: String::from(input[0]),
						author_email: String::from(input[1]),
						date: String::from(input[2]),
						subject: String::from(input[3]),
						body: String::from(input[4]),
						file_stats: stats
					}),
					Err(e) => Err(e)
				}
			},
			_ => Err(format!(
				"Invalid stat result:\n{}\n{}", commit_stat, input.len()
			))
		}
	}
	
	pub fn get_author_name(&self) -> &String {
		&self.author_name
	}
	
	pub fn get_author_email(&self) -> &String {
		&self.author_email
	}
	
	pub fn get_date(&self) -> &String {
		&self.date
	}
	
	pub fn get_subject(&self) -> &String {
		&self.subject
	}
	
	pub fn get_body(&self) -> &String {
		&self.body
	}
	
	pub fn get_file_stats(&self) -> &Vec<FileStat> {
		&self.file_stats
	}
}
