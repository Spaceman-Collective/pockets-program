use anchor_lang::prelude::*;

#[error_code]
pub enum PocketErrors {
    #[msg(
        "Can't leave a faction til you close all your Delegate Votes and Pending Proposal Accounts"
    )]
    CitizenHasOutstandingVotes,

    #[msg("Citizen doesn't have enough voting power!")]
    CitizenLacksVotingPower,

    #[msg("Invalid Voting Power Loss!")]
    InvalidVotingPowerDecrement,

    #[msg("Delgate has pending votes right now")]
    DelegatePendingVotes,

    #[msg("Resource Field is already developed!")]
    ResourceFieldAlreadyDeveloped,

    #[msg("Faction doesn't have that many unallocated votes")]
    TransferFromFactionErrror,
}
