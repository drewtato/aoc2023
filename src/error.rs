use chrono::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AocError {
	#[error("Part not found")]
	PartNotFound,
	#[error("Day {day} hasn't released yet. It releases {}:{:02}:{:02}:{:02} from now.",
	.duration.num_days(),
	.duration.num_hours() - .duration.num_days() * 24,
	.duration.num_minutes() - .duration.num_hours() * 60,
	.duration.num_seconds() - .duration.num_minutes() * 60)]
	HasNotReleasedYet { day: u32, duration: Duration },
	#[error("No test input found with the name {path}")]
	NoTestInputFound { path: String },
	#[error(transparent)]
	File {
		#[from]
		source: std::io::Error,
	},
	#[error(transparent)]
	Request { source: Box<ureq::Error> },
	#[error("Couldn't fetch prompt from network. Status {status}, content:\n{response}")]
	PromptResponse { status: u16, response: String },
	#[error(transparent)]
	OtherError {
		#[from]
		source: Box<dyn std::error::Error>,
	},
	#[error("Couldn't fetch input from network. Status: {status}\nContent:\n{response}")]
	InputResponse { status: u16, response: String },
	#[error("No day specified in argument `{arg}`")]
	NoDaySpecified { arg: String },
	#[error("Could not parse `{part}` as integer in argument `{arg}`")]
	Parse { part: String, arg: String },
	#[error("Non-UTF-8 data found in code block on the prompt page")]
	NonUtf8InPromptCodeBlock,
	#[error("Non-UTF-8 data found in solution")]
	NonUtf8InSolution,
	#[error(transparent)]
	FmtError {
		#[from]
		source: std::fmt::Error,
	},
	#[error("Day {0} not found")]
	DayNotFound(u32),
	#[error("Argument was empty")]
	EmptyArgument,
	#[error("Part was empty in {arg}")]
	EmptyPart { arg: String },
	#[error("Too many test cases were generated from the prompt")]
	TooManyTestCases,
	#[error("Answers did not match, exiting run")]
	IncorrectAnswer,
	#[error("{0} answers were incorrect.")]
	MultipleIncorrect(u32),
}

impl From<ureq::Error> for AocError {
	fn from(value: ureq::Error) -> Self {
		Self::Request {
			source: Box::new(value),
		}
	}
}
