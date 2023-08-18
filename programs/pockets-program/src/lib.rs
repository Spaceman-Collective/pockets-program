use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::*;

declare_id!("GEUwNbnu9jkRMY8GX5Ar4R11mX9vXR8UDFnKZMn5uWLJ");

pub mod account;
pub mod constant;
pub mod context;
pub mod error;

use crate::account::*;
use crate::constant::*;
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

        ctx.accounts.first_citizen.faction = Some(ctx.accounts.faction.key());
        ctx.accounts.first_citizen.granted_voting_power = starting_voting_power;
        ctx.accounts.first_citizen.total_voting_power = starting_voting_power;
        Ok(())
    }
    pub fn delete_faction(_ctx: Context<DeleteFactionAccount>) -> Result<()> {
        Ok(())
    }

    // Update Faction (Server Only)
    pub fn update_faction(
        ctx: Context<UpdateFaction>,
        max_voting_power: u64,
        threshold: u64,
    ) -> Result<()> {
        if max_voting_power > ctx.accounts.faction.max_voting_power {
            ctx.accounts.faction.unallocated_voting_power +=
                max_voting_power - ctx.accounts.faction.max_voting_power
        }

        ctx.accounts.faction.max_voting_power = max_voting_power;
        ctx.accounts.faction.threshold_to_pass = threshold;
        Ok(())
    }

    pub fn transfer_votes_from_faction(
        ctx: Context<TransferFromFaction>,
        amount: u64,
    ) -> Result<()> {
        if amount > ctx.accounts.faction.max_voting_power {
            return err!(PocketErrors::TransferFromFactionErrror);
        }

        ctx.accounts.faction.unallocated_voting_power -= amount;
        ctx.accounts.citizen.granted_voting_power += amount;
        ctx.accounts.citizen.total_voting_power += amount;
        Ok(())
    }

    // Create Citizen Record (Server Only -- Created for MINTs not for Wallets)
    pub fn create_citizen(ctx: Context<CreateCitizenRecord>) -> Result<()> {
        ctx.accounts.citizen.mint = ctx.accounts.mint.key();
        ctx.accounts.citizen.faction = None;
        ctx.accounts.citizen.delegated_voting_power = 0;
        ctx.accounts.citizen.granted_voting_power = 0;
        ctx.accounts.citizen.total_voting_power = 0;
        ctx.accounts.citizen.max_pledged_voting_power = 0;
        Ok(())
    }

    pub fn delete_citizen(_ctx: Context<DeleteCitizenAccount>) -> Result<()> {
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
    pub fn delete_proposal(_ctx: Context<DeleteProposalAccount>) -> Result<()> {
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

    pub fn delete_vote_delegation(_ctx: Context<DeleteVoteDelegation>) -> Result<()> {
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

    // Resource Fields
    // Allocate Resource Field -- Server Only
    pub fn allocate_resource_field(ctx: Context<DiscoverRF>, id: String) -> Result<()> {
        ctx.accounts.rf.id = id;
        ctx.accounts.rf.harvest = None;
        ctx.accounts.rf.refresh_seconds = None;
        ctx.accounts.rf.is_harvestable = false;
        ctx.accounts.rf.inital_claimant = None;
        Ok(())
    }

    pub fn delete_resource_field(_ctx: Context<DeleteResourceField>) -> Result<()> {
        Ok(())
    }

    pub fn develop_resource_field(ctx: Context<DevelopRF>) -> Result<()> {
        if ctx.accounts.rf.is_harvestable {
            return err!(PocketErrors::ResourceFieldAlreadyDeveloped);
        }

        let clock = Clock::get().unwrap();
        let mut hash_inputs: Vec<u8> = vec![];
        hash_inputs.extend(clock.slot.to_be_bytes().clone());
        hash_inputs.extend(ctx.accounts.rf.times_developed.to_be_bytes().clone());

        // [0..8] = Roll to see if it's valid RF
        // [9..16] = TYPE
        // [17..24] = AMT
        // [25..32] = Refresh Seconds
        let hash_bytes = &hash(&hash_inputs).to_bytes();

        // Did it find a resource field?
        // it'll roll a number betwen 1 and 1000, adding times it's previously failed to the roll
        let roll: u64 = (u64::from_be_bytes(hash_bytes[0..8].try_into().unwrap())
            / (u64::MAX / RF_CHANCE))
            + ctx.accounts.rf.times_developed;
        if roll >= RF_CHANCE {
            // YES -> Determine TYPE, AMT, and REFRESH TIME
            let resource_type: u64 = u64::from_be_bytes(hash_bytes[9..16].try_into().unwrap())
                / (u64::MAX / (RESOURCES.len() as u64 - 1));

            let harvest_amt: u64 = RF_MIN_YIELD
                + u64::from_be_bytes(hash_bytes[17..24].try_into().unwrap())
                    / (u64::MAX / (RF_MAX_YIELD - RF_MIN_YIELD));

            ctx.accounts.rf.harvest = Some(Harvest {
                resource: String::from(RESOURCES[resource_type as usize]),
                harvest: harvest_amt,
            });
            ctx.accounts.rf.refresh_seconds = Some(
                RF_MIN_TIMER
                    + u64::from_be_bytes(hash_bytes[25..32].try_into().unwrap())
                        / (u64::MAX / (RF_MAX_TIMER - RF_MIN_TIMER)),
            );

            msg!("Resource Field Developed!");
            // Set to be harvestable
            ctx.accounts.rf.is_harvestable = true;
            ctx.accounts.rf.inital_claimant = Some(ctx.accounts.wallet.key());
        }
        // NO -> Increment Times Developed
        ctx.accounts.rf.times_developed += 1;
        Ok(())
    }

    // RC6+ Pocket AMM
}
