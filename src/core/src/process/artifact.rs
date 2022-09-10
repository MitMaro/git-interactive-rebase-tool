use anyhow::Error;

use crate::{
	events::Event,
	module::{ExitStatus, State},
};

#[derive(Debug)]
#[allow(variant_size_differences)]
pub(crate) enum Artifact {
	Event(Event),
	ChangeState(State),
	Error(Error, Option<State>),
	ExitStatus(ExitStatus),
	ExternalCommand((String, Vec<String>)),
	EnqueueResize,
}
