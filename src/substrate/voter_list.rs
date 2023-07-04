use crate::polkadot::voter_list::events::*;

use subxt::{events::EventDetails, Error, PolkadotConfig};

#[repr(u8)]
pub enum VoterListEvent {
    Rebagged(Rebagged) = 0,
    ScoreUpdated(ScoreUpdated) = 1,
}

pub fn voter_list_process_event(
    event: EventDetails<PolkadotConfig>,
) -> Result<VoterListEvent, Error> {
    match event.variant_index() {
        0 => Ok(VoterListEvent::Rebagged(
            event.as_event()?.ok_or(Error::Other("error".to_string()))?,
        )),
        1 => Ok(VoterListEvent::ScoreUpdated(event.as_event()?.unwrap())),
        _ => Err(Error::Other("error".to_string())),
    }
}
