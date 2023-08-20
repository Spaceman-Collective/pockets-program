use crate::constant::*;
use anchor_lang::prelude::*;

#[account]
pub struct Faction {
    pub id: String,
    pub max_voting_power: u64,
    pub threshold_to_pass: u64,
    pub unallocated_voting_power: u64,
}

impl MaxSize for Faction {
    fn get_max_size() -> usize {
        // String is 4 + len, nanoid() churns out 21 character ids
        return (4 + 21) + 8 + 8 + 8;
    }
}

#[account]
pub struct Citizen {
    pub mint: Pubkey,
    pub faction: Option<Pubkey>,
    pub delegated_voting_power: u64,
    pub granted_voting_power: u64,
    pub total_voting_power: u64,
    pub max_pledged_voting_power: u64,
}

impl MaxSize for Citizen {
    fn get_max_size() -> usize {
        return 32 + 8 + 8 + 33 + 8 + 8;
    }
}

#[account]
pub struct Proposal {
    pub id: String, // nanoid() in Pockets DB
    pub faction: Pubkey,
    pub vote_amt: u64,
    pub status: ProposalStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    VOTING,
    PASSED,
    CLOSED,
}

impl MaxSize for Proposal {
    fn get_max_size() -> usize {
        return (4 + 21) + 8 + (1 + 4) + 32;
    }
}

#[account]
pub struct ProposalVote {
    pub citizen: Pubkey,
    pub vote_amt: u64,
}

impl MaxSize for ProposalVote {
    fn get_max_size() -> usize {
        return 32 + 8;
    }
}

#[account]
pub struct VoteDelegation {
    pub citizen: Pubkey,
    pub delegate: Pubkey,
    pub vote_amt: u64,
}

impl MaxSize for VoteDelegation {
    fn get_max_size() -> usize {
        return 32 + 32 + 8;
    }
}

#[account]
pub struct ResourceField {
    pub id: String, //(4+21)
    pub harvest: Option<Harvest>,
    pub refresh_seconds: Option<u64>,
    pub is_harvestable: bool,
    pub initial_claimant: Option<Pubkey>,
    pub times_developed: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Harvest {
    pub resource: String,
    pub harvest: u64,
}

impl MaxSize for ResourceField {
    fn get_max_size() -> usize {
        return (4 + 21) + (1 + (4 + LONGEST_RESOURCE_NAME + 8)) + 9 + 1 + 33 + 8 + 8;
        //added 8 bytes because it won't serialize the amount i think it is.
    }
}

pub trait MaxSize {
    fn get_max_size() -> usize;
}
