use paseo_metadata::paseo_metadata::{
    Event,
    runtime_types::{
        frame_system::pallet::Event as SystemEvent,
        pallet_bags_list::pallet::Event as BagsListEvent,
        pallet_balances::pallet::Event as BalancesEvent,
        pallet_bounties::pallet::Event as BountiesEvent,
        pallet_child_bounties::pallet::Event as ChildBountiesEvent,
        pallet_conviction_voting::pallet::Event as ConvictionVotingEvent,
        pallet_delegated_staking::pallet::Event as DelegatedStakingEvent,
        pallet_election_provider_multi_phase::pallet::Event as ElectionProviderMultiPhaseEvent,
        pallet_fast_unstake::pallet::Event as FastUnstakeEvent,
        pallet_indices::pallet::Event as IndicesEvent,
        pallet_multisig::pallet::Event as MultisigEvent,
        pallet_nomination_pools::pallet::Event as NominationPoolsEvent,
        pallet_preimage::pallet::Event as PreimageEvent,
        pallet_proxy::pallet::Event as ProxyEvent,
        pallet_referenda::pallet::Event as ReferendaEvent,
        pallet_session::pallet::Event as SessionEvent,
        pallet_staking::pallet::pallet::Event as StakingEvent,
        pallet_state_trie_migration::pallet::Event as StateTrieMigrationEvent,
        pallet_sudo::pallet::Event as SudoEvent,
        pallet_transaction_payment::pallet::Event as TransactionPaymentEvent,
        pallet_treasury::pallet::Event as TreasuryEvent,
        pallet_vesting::pallet::Event as VestingEvent,
        polkadot_runtime_common::{
            auctions::pallet::Event as AuctionsEvent, claims::pallet::Event as ClaimsEvent,
            crowdloan::pallet::Event as CrowdloanEvent,
            paras_registrar::pallet::Event as ParasRegistrarEvent,
            slots::pallet::Event as SlotsEvent,
        },
        polkadot_runtime_parachains::{
            disputes::pallet::Event as DisputesEvent, hrmp::pallet::Event as HrmpEvent,
            inclusion::pallet::Event as ParaInclusionEvent,
            on_demand::pallet::Event as OnDemandEvent, paras::pallet::Event as ParasEvent,
        },
    },
};
use subxt::utils::H256;

use crate::*;
use acuity_index_substrate::*;

use hex_literal::hex;

pub struct PaseoIndexer;

impl acuity_index_substrate::shared::RuntimeIndexer for PaseoIndexer {
    type RuntimeConfig = subxt::PolkadotConfig;
    type ChainKey = ChainKey;

    fn get_name() -> &'static str {
        "paseo"
    }

    fn get_genesis_hash() -> H256 {
        hex!["77afd6190f1554ad45fd0d31aee62aacc33c6db0ea801129acb813f913e0764f"].into()
    }

    fn get_versions() -> &'static [u32] {
        &[0]
    }

    fn get_default_url() -> &'static str {
        "wss://paseo-rpc.polkadot.io:443"
    }

    fn process_event(
        indexer: &acuity_index_substrate::substrate::Indexer<Self>,
        block_number: u32,
        event_index: u16,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<u32, IndexError> {
        Ok(match event.as_root_event::<Event>()? {
            // Substrate pallets.
            Event::System(event) => {
                index_system_event![SystemEvent, event, indexer, block_number, event_index]
            }
            Event::Preimage(event) => {
                index_preimage_event![PreimageEvent, event, indexer, block_number, event_index]
            }
            Event::Indices(event) => {
                index_indices_event![IndicesEvent, event, indexer, block_number, event_index]
            }
            Event::Balances(event) => {
                index_balances_event![BalancesEvent, event, indexer, block_number, event_index]
            }
            Event::TransactionPayment(event) => {
                index_transaction_payment_event![
                    TransactionPaymentEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::Session(event) => {
                index_session_event![SessionEvent, event, indexer, block_number, event_index]
            }
            Event::Vesting(event) => {
                index_vesting_event![VestingEvent, event, indexer, block_number, event_index]
            }
            Event::Proxy(event) => {
                index_proxy_event![ProxyEvent, event, indexer, block_number, event_index]
            }
            Event::Multisig(event) => {
                index_multisig_event![MultisigEvent, event, indexer, block_number, event_index]
            }
            Event::Bounties(event) => {
                index_bounties_event![BountiesEvent, event, indexer, block_number, event_index]
            }
            Event::ChildBounties(event) => {
                index_child_bounties_event![
                    ChildBountiesEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::Sudo(event) => {
                index_sudo_event![SudoEvent, event, indexer, block_number, event_index]
            }
            Event::Staking(event) => {
                index_staking_event![StakingEvent, event, indexer, block_number, event_index]
            }
            Event::Treasury(event) => {
                index_treasury_event![TreasuryEvent, event, indexer, block_number, event_index]
            }
            Event::ConvictionVoting(event) => {
                index_conviction_voting_event![
                    ConvictionVotingEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::Referenda(event) => {
                index_referenda_event![ReferendaEvent, event, indexer, block_number, event_index]
            }
            Event::ElectionProviderMultiPhase(event) => {
                index_election_provider_multi_phase_event![
                    ElectionProviderMultiPhaseEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::VoterList(event) => {
                index_bags_list_event![BagsListEvent, event, indexer, block_number, event_index]
            }
            Event::NominationPools(event) => {
                index_nomination_pools_event![
                    NominationPoolsEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::FastUnstake(event) => {
                index_fast_unstake_event![
                    FastUnstakeEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::DelegatedStaking(event) => {
                index_delegated_staking_event![
                    DelegatedStakingEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            // Polkadot pallets.
            Event::Claims(event) => {
                index_claims_event![ClaimsEvent, event, indexer, block_number, event_index]
            }
            Event::Paras(event) => {
                index_paras_event![ParasEvent, event, indexer, block_number, event_index]
            }
            Event::Hrmp(event) => {
                index_hrmp_event![HrmpEvent, event, indexer, block_number, event_index]
            }
            Event::ParasDisputes(event) => {
                index_disputes_event![DisputesEvent, event, indexer, block_number, event_index]
            }
            Event::Registrar(event) => {
                index_paras_registrar_event![
                    ParasRegistrarEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::Slots(event) => {
                index_slots_event![SlotsEvent, event, indexer, block_number, event_index]
            }
            Event::Auctions(event) => {
                index_auctions_event![AuctionsEvent, event, indexer, block_number, event_index]
            }
            Event::Crowdloan(event) => {
                index_crowdloan_event![CrowdloanEvent, event, indexer, block_number, event_index]
            }
            Event::ParaInclusion(event) => {
                index_para_inclusion_event![
                    ParaInclusionEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::OnDemand(event) => {
                index_on_demand_event![OnDemandEvent, event, indexer, block_number, event_index]
            }
            Event::StateTrieMigration(event) => {
                index_state_trie_migration_event![
                    StateTrieMigrationEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            _ => 0,
        })
    }
}
