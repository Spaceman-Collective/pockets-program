use anchor_lang::prelude::*;

declare_id!("GEUwNbnu9jkRMY8GX5Ar4R11mX9vXR8UDFnKZMn5uWLJ");

pub mod account;
pub mod constant;
pub mod context;
pub mod error;

use crate::account::*;
use crate::context::*;
use crate::error::*;

#[program]
pub mod pockets_program {
    use super::*;

    // Faction
    // Create Faction (Server Only)
    pub fn create_faction(
        ctx: Context<CreateFaction>,
        id: String,
        starting_voting_power: u64,
        threshold: u64,
    ) -> Result<()> {
        ctx.accounts.faction.id = id;
        ctx.accounts.faction.max_voting_power = starting_voting_power;
        ctx.accounts.faction.threshold_to_pass = threshold;
        Ok(())
    }
    // Update Faction (Server Only)
    pub fn update_faction(
        ctx: Context<UpdateFaction>,
        id: String,
        starting_voting_power: u64,
        threshold: u64,
    ) -> Result<()> {
        ctx.accounts.faction.id = id;
        ctx.accounts.faction.max_voting_power = starting_voting_power;
        ctx.accounts.faction.threshold_to_pass = threshold;
        Ok(())
    }

    // Create Citizen Record (Server Only -- Created for MINTs not for Wallets)
    pub fn create_citizen(ctx: Context<CreateCitizenRecord>) -> Result<()> {
        ctx.accounts.citizen.mint = ctx.accounts.mint.key();
        // Rest are set to None, 0, 0 by default
        Ok(())
    }

    // Join Faction (Server Only - Need to Record in DB)
    pub fn join_faction(ctx: Context<JoinFaction>) -> Result<()> {
        ctx.accounts.citizen.faction = Some(ctx.accounts.faction.key());
        Ok(())
    }

    // Leave Faction (Server Only - Need to Record in DB)
    /**
     * Requires
     *  1. This user has 0 voting power delegated to other users
     *  2. This user has 0 voting power delegated by other users
     *  3. This user has 0 voting power locked up in proposals
     *
     * User should close the above accounts before leaving a faction
     */
    pub fn leave_faction(ctx: Context<LeaveFaction>) -> Result<()> {
        if ctx.accounts.citizen.max_pledged_voting_power != 0
            || ctx.accounts.citizen.delegated_voting_power != 0
        {
            return err!(PocketErrors::CitizenHasOutstandingVotes);
        } else {
            ctx.accounts.faction.unallocated_voting_power +=
                ctx.accounts.citizen.granted_voting_power;

            ctx.accounts.citizen.faction = None;
            ctx.accounts.citizen.delegated_voting_power = 0;
            ctx.accounts.citizen.granted_voting_power = 0;
            ctx.accounts.citizen.total_voting_power = 0;
        }
        Ok(())
    }

    // Proposals
    // Create Proposal (Server Only - Need to Validate on Server and provide ID)
    pub fn create_proposal(ctx: Context<CreateProposal>, id: String) -> Result<()> {
        ctx.accounts.proposal.id = id;
        ctx.accounts.proposal.status = ProposalStatus::VOTING;
        ctx.accounts.proposal.faction = ctx.accounts.faction.key();
        Ok(())
    }
    // Update Proposal Status (Server Only)
    pub fn update_proposal(ctx: Context<UpdateProposal>, new_status: ProposalStatus) -> Result<()> {
        ctx.accounts.proposal.status = new_status;
        Ok(())
    }
    // Vote on Proposal
    pub fn vote_on_proposal(ctx: Context<Vote>, vote_amt: u64) -> Result<()> {
        // Check that the Vote Amt is something that the Citizen has
        if ctx.accounts.citizen.total_voting_power - ctx.accounts.citizen.max_pledged_voting_power
            < vote_amt
        {
            return err!(PocketErrors::CitizenLacksVotingPower);
        } else {
            ctx.accounts.citizen.max_pledged_voting_power += vote_amt;
            ctx.accounts.vote.citizen = ctx.accounts.citizen.key();
            ctx.accounts.vote.vote_amt = vote_amt;
            ctx.accounts.proposal.vote_amt += vote_amt;

            // Check if vote passed, is so, set it to passed
            if ctx.accounts.proposal.vote_amt >= ctx.accounts.faction.threshold_to_pass {
                ctx.accounts.proposal.status = ProposalStatus::PASSED;
            }
        }
        Ok(())
    }

    // Increment/Decerement Vote Amount
    pub fn update_vote(ctx: Context<UpdateVote>, vote_amt: u64, is_increment: bool) -> Result<()> {
        if is_increment {
            if ctx.accounts.citizen.total_voting_power
                - ctx.accounts.citizen.max_pledged_voting_power
                < vote_amt
            {
                return err!(PocketErrors::CitizenLacksVotingPower);
            } else {
                ctx.accounts.citizen.max_pledged_voting_power += vote_amt;
                ctx.accounts.proposal.vote_amt += vote_amt;
                ctx.accounts.vote.vote_amt += vote_amt;

                // Check if vote passed, is so, set it to passed
                if ctx.accounts.proposal.vote_amt >= ctx.accounts.faction.threshold_to_pass {
                    ctx.accounts.proposal.status = ProposalStatus::PASSED;
                }
            }
        } else {
            // Decrement
            if vote_amt > ctx.accounts.vote.vote_amt {
                return err!(PocketErrors::InvalidVotingPowerDecrement);
            }

            ctx.accounts.citizen.max_pledged_voting_power -= vote_amt;
            ctx.accounts.proposal.vote_amt -= vote_amt;
            ctx.accounts.vote.vote_amt -= vote_amt;
        }
        Ok(())
    }

    // Close Vote for Finished Proposals
    pub fn close_vote_account(ctx: Context<CloseVoteAccount>) -> Result<()> {
        ctx.accounts.citizen.max_pledged_voting_power -= ctx.accounts.vote.vote_amt;
        ctx.accounts.proposal.vote_amt -= ctx.accounts.vote.vote_amt;
        Ok(())
    }

    // Don't want close proposal account because there might be outstanding vote accounts and we'd leave them stranded

    // Vote
    // Transfer Vote
    pub fn transfer_votes(ctx: Context<TransferVotes>, vote_amt: u64) -> Result<()> {
        if ctx.accounts.citizen.total_voting_power - ctx.accounts.citizen.max_pledged_voting_power
            < vote_amt
            || ctx.accounts.citizen.granted_voting_power < vote_amt
        {
            return err!(PocketErrors::InvalidVotingPowerDecrement);
        }

        ctx.accounts.citizen.total_voting_power -= vote_amt;
        ctx.accounts.citizen.granted_voting_power -= vote_amt;

        ctx.accounts.vote_recepient.granted_voting_power += vote_amt;
        ctx.accounts.vote_recepient.total_voting_power += vote_amt;
        Ok(())
    }

    // Delegate Vote
    // Can only delegate granted power
    pub fn delegate_votes(ctx: Context<DelegateVote>, vote_amt: u64) -> Result<()> {
        if ctx.accounts.citizen.total_voting_power - ctx.accounts.citizen.max_pledged_voting_power
            < vote_amt
            || ctx.accounts.citizen.granted_voting_power < vote_amt
        {
            return err!(PocketErrors::InvalidVotingPowerDecrement);
        }

        ctx.accounts.citizen.max_pledged_voting_power += vote_amt;
        ctx.accounts.vote_recepient.delegated_voting_power += vote_amt;
        ctx.accounts.vote_recepient.total_voting_power += vote_amt;

        ctx.accounts.delegation_record.citizen = ctx.accounts.citizen.key();
        ctx.accounts.delegation_record.delegate = ctx.accounts.vote_recepient.key();
        ctx.accounts.delegation_record.vote_amt = vote_amt;
        Ok(())
    }
    // Adjust Delegate Vote
    pub fn adjust_vote_delegation(
        ctx: Context<AdjustDelegation>,
        vote_amt: u64,
        is_increment: bool,
    ) -> Result<()> {
        if is_increment {
            if ctx.accounts.citizen.total_voting_power
                - ctx.accounts.citizen.max_pledged_voting_power
                < vote_amt
                || ctx.accounts.citizen.granted_voting_power < vote_amt
            {
                return err!(PocketErrors::InvalidVotingPowerDecrement);
            }
            ctx.accounts.citizen.max_pledged_voting_power += vote_amt;
            ctx.accounts.vote_recepient.delegated_voting_power += vote_amt;
            ctx.accounts.vote_recepient.total_voting_power += vote_amt;
            ctx.accounts.delegation_record.vote_amt += vote_amt;
        } else {
            // Decrement
            if ctx.accounts.vote_recepient.total_voting_power
                - ctx.accounts.vote_recepient.max_pledged_voting_power
                < vote_amt
                || ctx.accounts.vote_recepient.delegated_voting_power < vote_amt
            {
                return err!(PocketErrors::DelegatePendingVotes);
            }

            ctx.accounts.citizen.max_pledged_voting_power -= vote_amt;
            ctx.accounts.vote_recepient.delegated_voting_power -= vote_amt;
            ctx.accounts.vote_recepient.total_voting_power -= vote_amt;
            ctx.accounts.delegation_record.vote_amt -= vote_amt;
        }
        Ok(())
    }

    // Return Vote Delegation
    // Decrement from the Delgation's side
    pub fn return_vote_delegation(ctx: Context<AdjustDelegation>, vote_amt: u64) -> Result<()> {
        if ctx.accounts.citizen.total_voting_power - ctx.accounts.citizen.delegated_voting_power
            < vote_amt
            || ctx.accounts.citizen.delegated_voting_power < vote_amt
        {
            return err!(PocketErrors::InvalidVotingPowerDecrement);
        }

        ctx.accounts.citizen.total_voting_power -= vote_amt;
        ctx.accounts.citizen.delegated_voting_power -= vote_amt;
        ctx.accounts.vote_recepient.max_pledged_voting_power -= vote_amt;

        ctx.accounts.delegation_record.vote_amt -= vote_amt;
        Ok(())
    }

    // RC7: Resource Fields
    // Mine Resource Field

    // RC6+ Pocket AMM
}
