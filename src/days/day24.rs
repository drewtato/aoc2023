#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use itertools::izip;
use z3::ast::{Ast, Int};
use z3::{Config, Context};

use crate::helpers::*;

pub type A1 = u32;
pub type A2 = i64;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	hailstones: Vec<Hailstone>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let hailstones = file.lines().map(Hailstone::from_line).collect();
		Self { hailstones }
	}

	fn part_one(&mut self, d: u8) -> Self::AnswerOne {
		let (min, max) = match d {
			0 => (MIN, MAX),
			1 => (7.0, 27.0),
			_ => unimplemented!(),
		};

		let mut intersects = 0;
		for (i, ha) in self.hailstones[..self.hailstones.len() - 1]
			.iter()
			.enumerate()
		{
			for hb in &self.hailstones[i + 1..] {
				let tb = (-hb.pos[2] * ha.vel[1] + ha.pos[2] * ha.vel[1] + hb.pos[1] * ha.vel[2]
					- ha.pos[1] * ha.vel[2])
					/ (hb.vel[2] * ha.vel[1] - hb.vel[1] * ha.vel[2]);

				if tb.is_infinite() || tb.is_sign_negative() {
					continue;
				}

				let ta = (hb.pos[2] + tb * hb.vel[2] - ha.pos[2]) / ha.vel[2];

				if ta.is_sign_negative() {
					continue;
				}

				let x = ha.pos[2] + ta * ha.vel[2];

				if x < min || x > max {
					continue;
				}
				if x < min || x > max {
					continue;
				}

				let y = ha.pos[1] + ta * ha.vel[1];

				if y < min || y > max {
					continue;
				}

				// println!("x={x:.3} y={y:.3}");

				intersects += 1;
			}
		}
		intersects
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut config = Config::new();
		config.set_model_generation(true);
		let c = Context::new(&config);

		let ps = ["pz", "py", "px"].map(|p| Int::new_const(&c, p));
		let vs = ["vz", "vy", "vx"].map(|v| Int::new_const(&c, v));
		let ts = ["t1", "t2", "t3"].map(|t| Int::new_const(&c, t));

		let hail: [_; 3] = self.hailstones.array_chunks().next().unwrap().map(|h| {
			(
				h.pos.map(|n| Int::from_i64(&c, n as _)),
				h.vel.map(|n| Int::from_i64(&c, n as _)),
			)
		});

		let solver = z3::Solver::new(&c);
		for ((p1, v1), t1) in izip!(&hail, &ts) {
			for (p1z, v1z, psz, vsz) in izip!(p1, v1, &ps, &vs) {
				solver.assert(&(p1z + v1z * t1)._eq(&(psz + vsz * t1)));
			}
		}

		assert_eq!(solver.check(), z3::SatResult::Sat);
		let model = solver.get_model().unwrap();
		let ps_result = ps.map(|p| model.eval(&p, true).unwrap().as_i64().unwrap());

		ps_result.into_iter().sum()
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

type Unit = f64;
const MIN: f64 = 200_000_000_000_000.0;
const MAX: f64 = 400_000_000_000_000.0;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Hailstone {
	pos: [Unit; 3],
	vel: [Unit; 3],
}

impl Hailstone {
	fn from_line(line: &[u8]) -> Self {
		let (before, after) = line.split_once(is(b'@')).unwrap();
		let pos = before
			.trim_ascii()
			.delimiter(", ")
			.rev()
			.multi_parse()
			.unwrap();
		let vel = after
			.trim_ascii()
			.delimiter(", ")
			.rev()
			.map(|s| s.trim_ascii())
			.multi_parse()
			.unwrap();
		Self { pos, vel }
	}
}
