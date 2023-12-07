use std::ops::Add;

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	hands: Vec<Hand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hand {
	cards: [Card; 5],
	hand_type: HandType,
	bid: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JokerHand {
	cards: [Card; 5],
	joker_hand_type: HandType,
	bid: u32,
}

impl From<Hand> for JokerHand {
	fn from(value: Hand) -> Self {
		let Hand {
			cards,
			hand_type,
			bid,
		} = value;
		let cards = cards.map(|c| if c == J { Joker } else { c });
		let jokers = cards.into_iter().filter(|&c| c == Joker).count();
		Self {
			cards,
			joker_hand_type: hand_type + jokers,
			bid,
		}
	}
}

impl Ord for JokerHand {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.joker_hand_type
			.cmp(&other.joker_hand_type)
			.then_with(|| self.cards.cmp(&other.cards))
	}
}

impl PartialOrd for JokerHand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Hand {
	fn new(cards: [Card; 5], bid: u32) -> Self {
		let hand_type = Self::hand_type(cards);
		Self {
			cards,
			hand_type,
			bid,
		}
	}

	fn hand_type(cards: [Card; 5]) -> HandType {
		let mut ns = cards.map(|card| cards.into_iter().filter(|&c| c == card).count());
		ns.sort_unstable();

		match ns {
			[.., 5] => FiveOfAKind,
			[.., 4] => FourOfAKind,
			[.., 2, 3, 3, 3] => FullHouse,
			[.., 3] => ThreeOfAKind,
			[.., 2, 2, 2] => TwoPair,
			[.., 2] => OnePair,
			_ => High,
		}
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.hand_type
			.cmp(&other.hand_type)
			.then_with(|| self.cards.cmp(&other.cards))
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

use Card::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
	Joker,
	N2,
	N3,
	N4,
	N5,
	N6,
	N7,
	N8,
	N9,
	T,
	J,
	Q,
	K,
	A,
}

impl FromBytes for Card {
	fn from_bytes(bytes: &[u8]) -> Option<Self> {
		let c = match bytes {
			[b'A'] => A,
			[b'K'] => K,
			[b'Q'] => Q,
			[b'J'] => J,
			[b'T'] => T,
			[b'9'] => N9,
			[b'8'] => N8,
			[b'7'] => N7,
			[b'6'] => N6,
			[b'5'] => N5,
			[b'4'] => N4,
			[b'3'] => N3,
			[b'2'] => N2,
			_ => return None,
		};
		Some(c)
	}
}

use HandType::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
	High,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

impl Add<usize> for HandType {
	type Output = HandType;

	fn add(self, jokers: usize) -> Self::Output {
		match jokers {
			5 => FiveOfAKind,
			4 => FiveOfAKind,
			3 => match self {
				FullHouse => FiveOfAKind,
				_ => FourOfAKind,
			},
			2 => match self {
				FullHouse => FiveOfAKind,
				TwoPair => FourOfAKind,
				OnePair => ThreeOfAKind,
				_ => unreachable!(),
			},
			1 => match self {
				FourOfAKind => FiveOfAKind,
				ThreeOfAKind => FourOfAKind,
				TwoPair => FullHouse,
				OnePair => ThreeOfAKind,
				High => OnePair,
				_ => unreachable!(),
			},
			0 => self,
			_ => unreachable!(),
		}
	}
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let hands = file
			.lines()
			.map(|line| {
				let (cards, bid) = line.split_at(6);
				let cards = &cards[..5];
				let cards = cards
					.iter()
					.map(|&c| [c].as_slice().parse().unwrap())
					.array()
					.unwrap();
				let bid = bid.parse().unwrap();
				Hand::new(cards, bid)
			})
			.collect();
		Self { hands }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.hands.sort_unstable();

		self.hands
			.iter()
			.enumerate()
			.map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
			.sum_self()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut joker_hands: Vec<JokerHand> = self.hands.iter().map(|&hand| hand.into()).collect();
		joker_hands.sort_unstable();

		joker_hands
			.iter()
			.enumerate()
			.map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
			.sum_self()
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
