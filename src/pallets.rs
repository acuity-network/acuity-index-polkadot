#[macro_export]
macro_rules! index_claims_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Claimed { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
        }
    };
}

#[macro_export]
macro_rules! index_paras_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::CurrentCodeUpdated(para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            <$event_enum>::CurrentHeadUpdated(para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            <$event_enum>::CodeUpgradeScheduled(para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            <$event_enum>::NewHeadNoted(para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            <$event_enum>::ActionQueued(para_id, session_index) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
                $indexer.index_event_session_index(session_index, $block_number, $event_index);
            }
            <$event_enum>::PvfCheckStarted(_, para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            <$event_enum>::PvfCheckAccepted(_, para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            <$event_enum>::PvfCheckRejected(_, para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
        }
    };
}

#[macro_export]
macro_rules! index_hrmp_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::OpenChannelRequested(sender, recipient, ..) => {
                $indexer.index_event_para_id(sender.0, $block_number, $event_index);
                $indexer.index_event_para_id(recipient.0, $block_number, $event_index);
            }
            <$event_enum>::OpenChannelCanceled(by_parachain, ..) => {
                $indexer.index_event_para_id(by_parachain.0, $block_number, $event_index);
            }
            <$event_enum>::OpenChannelAccepted(sender, recipient) => {
                $indexer.index_event_para_id(sender.0, $block_number, $event_index);
                $indexer.index_event_para_id(recipient.0, $block_number, $event_index);
            }
            <$event_enum>::ChannelClosed(by_parachain, ..) => {
                $indexer.index_event_para_id(by_parachain.0, $block_number, $event_index);
            }
            <$event_enum>::HrmpChannelForceOpened(sender, recipient, ..) => {
                $indexer.index_event_para_id(sender.0, $block_number, $event_index);
                $indexer.index_event_para_id(recipient.0, $block_number, $event_index);
            }
        }
    };
}

#[macro_export]
macro_rules! index_disputes_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::DisputeInitiated(candidate_hash, ..) => {
                $indexer.index_event_candidate_hash(
                    candidate_hash.0.into(),
                    $block_number,
                    $event_index,
                );
            }
            <$event_enum>::DisputeConcluded(candidate_hash, ..) => {
                $indexer.index_event_candidate_hash(
                    candidate_hash.0.into(),
                    $block_number,
                    $event_index,
                );
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_paras_registrar_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Registered { para_id, manager } => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
                $indexer.index_event_account_id(manager, $block_number, $event_index);
            }
            <$event_enum>::Deregistered { para_id } => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            <$event_enum>::Reserved { para_id, who } => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
                $indexer.index_event_account_id(who, $block_number, $event_index);
            } /*
              <$event_enum>::Swapped { para_id, other_id } => {
                  $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
                  $indexer.index_event_para_id(other_id.0, $block_number, $event_index);
              }*/
        }
    };
}

#[macro_export]
macro_rules! index_slots_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Leased {
                para_id, leaser, ..
            } => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
                $indexer.index_event_account_id(leaser, $block_number, $event_index);
            }
            _ => {}
        }
    };
}
