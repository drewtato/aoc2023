use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	games: Vec<Vec<[u64; 3]>>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let games = file
			.lines()
			.map(|line| {
				let (_, line) = line.split_once(is(b' ')).unwrap();
				let (_id, line) = line.split_once(is(b':')).unwrap();
				line.delimiter(';')
					.map(|round| {
						let cubes = round
							.delimiter(',')
							.map(|cubes| cubes[1..].split_once(is(b' ')).unwrap());
						let mut red = 0;
						let mut green = 0;
						let mut blue = 0;
						for (n, c) in cubes {
							let n: u64 = n.parse().unwrap();
							match c {
								b"red" => red += n,
								b"green" => green += n,
								b"blue" => blue += n,
								_ => panic!("bad color {:?}", c.to_display_slice()),
							}
						}
						[red, green, blue]
					})
					.collect()
			})
			.collect();
		Self { games }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		// dbg_small!(self);
		let valid_counts = [12, 13, 14];

		self.games
			.iter()
			.enumerate()
			.map(|(index, game)| {
				let id = index + 1;
				if is_valid(game, valid_counts) {
					id
				} else {
					0
				}
			})
			.sum_self()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut total = 0;
		for game in &self.games {
			let mut red = 0;
			let mut green = 0;
			let mut blue = 0;
			for &[r, g, b] in game {
				red = red.max(r);
				green = green.max(g);
				blue = blue.max(b);
			}
			let power = red * green * blue;
			total += power;
		}
		total
	}

	fn run_any<W: std::fmt::Write>(
		&mut self,
		part: u32,
		_writer: W,
		_: u8,
	) -> Res<std::time::Duration> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}

fn is_valid(game: &[[u64; 3]], valid_counts: [u64; 3]) -> bool {
	let [vr, vg, vb] = valid_counts;
	for &[red, green, blue] in game {
		if red > vr || green > vg || blue > vb {
			return false;
		}
	}
	true
}
