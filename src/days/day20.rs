use std::fmt::Write;

use arrayvec::ArrayVec;

use crate::helpers::*;

pub type A1 = usize;
pub type A2 = usize;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	configuration: Configuration,
	p1: bool,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let configuration = Configuration::from_file(&file).unwrap();

		Self {
			configuration,
			p1: false,
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut low = 0;
		let mut high = 0;
		for _ in 0..1000 {
			let (l, h) = self.configuration.run1();
			low += l;
			high += h;
		}
		self.p1 = true;
		low * high
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		if self.p1 {
			self.configuration.reset();
		}
		let Some(cycle_count) = self.configuration.cycle_count() else {
			return 0;
		};
		let mut cycles: ArrayVec<usize, 8> = repeat_iter(0).take(cycle_count).collect();
		let mut cycles_found = 0;

		'f: for presses in 1.. {
			let mut bits = self.configuration.run2();
			if bits != 0 {
				// println!("{bits:04b} {presses}");
				for cycle in &mut cycles {
					if (*cycle == 0) && (bits & 1 == 1) {
						*cycle = presses;
						cycles_found += 1;
						if cycles_found == cycle_count {
							break 'f;
						}
					}
					bits >>= 1;
				}
			}
		}

		cycles.into_iter().product()
		// 211490449619904 too low
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

#[derive(Debug, Clone, Default)]
struct Configuration {
	map: HashMap<Name, Module>,
	broadcaster: Module,
	cycle_module: Option<Name>,
}

impl Configuration {
	fn from_file(file: &[u8]) -> Option<Self> {
		let mut map: HashMap<Name, Module> = HashMap::default();
		let mut outputs: HashMap<Name, Inputs> = HashMap::default();
		let mut broadcaster = None;
		let mut cycle_module = None;

		for line in file.lines() {
			let mut module = Module::from_line(line)?;
			if let Some(inputs) = outputs.remove(&module.name) {
				if let Conjunction(_, inp) = &mut module.mod_type {
					*inp = inputs;
				}
			}

			if module.outputs.as_slice() == [RECEIVER] {
				cycle_module = Some(module.name);
			}

			for out in &module.outputs {
				if let Some(out_module) = map.get_mut(out) {
					if let Conjunction(_, inputs) = &mut out_module.mod_type {
						inputs.push(module.name);
					}
				} else {
					outputs.entry(*out).or_default().push(module.name)
				}
			}

			match module.mod_type {
				Broadcaster => broadcaster = Some(module),
				_ => {
					map.insert(module.name, module);
				}
			}
		}

		if cfg!(debug_assertions) {
			for module in map.values_mut() {
				if let Conjunction(_, inputs) = &mut module.mod_type {
					inputs.sort_unstable();
				}
			}
		}

		Some(Self {
			map,
			broadcaster: broadcaster?,
			cycle_module,
		})
	}

	fn run1(&mut self) -> (usize, usize) {
		let mut low = 0;
		let mut high = 0;

		// Button module
		low += 1;

		let mut queue = VecDeque::new();
		// Broadcaster
		for &out in &self.broadcaster.outputs {
			queue.push_back((Name::default(), Low, out));
		}

		// Process
		while let Some((from, pulse, to)) = queue.pop_front() {
			match pulse {
				Low => low += 1,
				High => high += 1,
			}

			if let Some(module) = self.map.get_mut(&to) {
				queue.extend(module.receive(from, pulse, to));
			}
		}

		(low, high)
	}

	fn run2(&mut self) -> u16 {
		let cycle_module = self.cycle_module.unwrap();
		let mut queue = VecDeque::new();
		// Broadcaster
		for &out in &self.broadcaster.outputs {
			queue.push_back((Name::default(), Low, out));
		}
		let mut bits = 0;

		// Process
		while let Some((from, pulse, to)) = queue.pop_front() {
			if let Some(module) = self.map.get_mut(&to) {
				queue.extend(module.receive(from, pulse, to));
				if to == cycle_module && pulse == Low {
					let Conjunction(state, _) = module.mod_type else {
						unreachable!()
					};
					bits |= state;
				}
			}
		}

		bits
	}

	fn cycle_count(&self) -> Option<usize> {
		let Conjunction(_, inputs) = &self.map.get(&self.cycle_module?)?.mod_type else {
			return None;
		};
		Some(inputs.len())
	}

	fn reset(&mut self) {
		for module in self.map.values_mut() {
			module.reset();
		}
	}
}

const NAME_LEN: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct Name([u8; NAME_LEN]);

const RECEIVER: Name = Name(*b"rx");

impl Name {
	fn from_slice(slice: &[u8]) -> Self {
		let mut arr = [0; NAME_LEN];
		for (a, &b) in arr.iter_mut().zip(slice) {
			*a = b;
		}
		Name(arr)
	}
}

impl Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&DisplaySlice(self.0), f)
	}
}

type Outputs = ArrayVec<Name, 16>;
type Inputs = ArrayVec<Name, 16>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct Module {
	name: Name,
	mod_type: ModType,
	outputs: Outputs,
}

impl Module {
	fn new(name: Name, mod_type: ModType, outputs: Outputs) -> Self {
		Self {
			name,
			mod_type,
			outputs,
		}
	}

	fn from_line(line: &[u8]) -> Option<Self> {
		let (before, after) = line.split_once(is(b' '))?;
		let after = &after[3..];
		let outputs = after.delimiter(", ").map(Name::from_slice).collect();
		match before {
			b"broadcaster" => Some(Self::new(Default::default(), Broadcaster, outputs)),
			[b'%', rest @ ..] => {
				let name = Name::from_slice(rest);
				Some(Self::new(name, FlipFlop(false), outputs))
			}
			[b'&', rest @ ..] => {
				let name = Name::from_slice(rest);
				Some(Self::new(name, Conjunction(0, Default::default()), outputs))
			}
			_ => None,
		}
	}

	fn receive(
		&mut self,
		from: Name,
		pulse: Pulse,
		to: Name,
	) -> impl IntoIterator<Item = (Name, Pulse, Name)> + '_ {
		gen_iter(move || {
			let Some(pulse) = self.mod_type.receive(from, pulse) else {
				return;
			};

			for &out in &self.outputs {
				yield (to, pulse, out)
			}
		})
	}

	fn reset(&mut self) {
		self.mod_type.reset();
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
enum ModType {
	#[default]
	Broadcaster,
	FlipFlop(bool),
	Conjunction(u16, Inputs),
}
use ModType::*;

impl ModType {
	fn receive(&mut self, from: Name, pulse: Pulse) -> Option<Pulse> {
		match (self, pulse) {
			(FlipFlop(on), Low) => {
				let pulse = if *on { Low } else { High };
				*on = !*on;
				Some(pulse)
			}
			(FlipFlop(..), High) => None,
			(Conjunction(bits, inputs), _) => {
				let i = inputs.iter().position(|&n| n == from).unwrap();
				match pulse {
					Low => *bits &= u16::MAX ^ (1 << i),
					High => *bits |= 1 << i,
				}

				if bits.count_ones() as usize == inputs.len() {
					Some(Low)
				} else {
					Some(High)
				}
			}
			_ => unreachable!(),
		}
	}

	fn reset(&mut self) {
		match self {
			Broadcaster => (),
			FlipFlop(b) => *b = false,
			Conjunction(b, _) => *b = 0,
		}
	}
}

impl Debug for ModType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Broadcaster => write!(f, "Broadcaster"),
			Self::FlipFlop(on) => f.debug_tuple("FlipFlop").field(on).finish(),
			Self::Conjunction(bits, inputs) => f
				.debug_tuple("Conjunction")
				.field(&{
					let mut s = String::new();
					let mut bits = *bits;
					for i in inputs {
						write!(s, "{}:{} ", DisplaySlice(&i.0), bits % 2).unwrap();
						bits >>= 1;
					}
					s.pop();
					s
				})
				.finish(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pulse {
	Low,
	High,
}
use Pulse::*;
