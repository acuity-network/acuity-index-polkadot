#[macro_export]
macro_rules! index_claims_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Claimed { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            _ => {}
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
            <$event_enum>::PvfCheckStarted(code_hash, para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            <$event_enum>::PvfCheckAccepted(code_hash, para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            <$event_enum>::PvfCheckRejected(code_hash, para_id) => {
                $indexer.index_event_para_id(para_id.0, $block_number, $event_index);
            }
            _ => {}
        }
    };
}
