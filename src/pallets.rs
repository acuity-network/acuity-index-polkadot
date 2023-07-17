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
