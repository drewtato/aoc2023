use std::time::Instant;

use ferris_elf::{run, DAY, PART};

fn main() {
	let input_file = format!("inputs/day{DAY:02}/input.txt");
	let input = std::fs::read_to_string(input_file).unwrap();

	let answer_file = format!("inputs/day{DAY:02}/answer.txt");
	let answer_lines = std::fs::read_to_string(answer_file).unwrap();
	let expected_ans = answer_lines.lines().nth(PART as usize - 1).unwrap();
	let ans = run(input.as_ref());

	let mut times: Vec<_> = std::iter::repeat_with(|| {
		let t = Instant::now();
		let ans = run(input.as_ref());
		let time = t.elapsed();
		let ans = format!("{ans}");
		assert_eq!(ans, expected_ans);
		time
	})
	.take(1000)
	.collect();

	times.sort_unstable();
	let median = times[500];

	println!("{ans} ({median:?})");
}