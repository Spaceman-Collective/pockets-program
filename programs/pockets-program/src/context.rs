use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::TokenAccount;

use crate::account::*;
use crate::constant::*;

#[derive(Accounts)]
#[instruction(id: String, starting_voting_power: u64, threshold: u64)]
pub struct CreateFaction<'info> {
    #[account(
      mut,
      address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
    )]
    pub server: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
      init,
      payer=server,
      seeds=[
        SEEDS_FACTION,
        id.as_bytes(),
      ],
      bump,
      space=8+Faction::get_max_size()
    )]
    pub faction: Account<'info, Faction>,

    #[account(
      mut,
      constraint = first_citizen.faction == None
    )]
    pub first_citizen: Account<'info, Citizen>,
}

#[derive(Accounts)]
pub struct DeleteFactionAccount<'info>{
  #[account(
    mut,
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,

  #[account(
    mut,
    close=server
  )]
  pub faction: Account<'info, Faction>,
}

#[derive(Accounts)]
#[instruction(id: String, starting_voting_power: u64, threshold: u64)]
pub struct UpdateFaction<'info> {
    #[account(
      address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
    )]
    pub server: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
      mut,
      seeds=[
        SEEDS_FACTION,
        id.as_bytes(),
      ],
      bump,
    )]
    pub faction: Account<'info, Faction>,
}

#[derive(Accounts)]
pub struct CreateCitizenRecord<'info> {
    #[account(
      mut,
      address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
    )]
    pub server: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
      init, 
      payer=server,
      seeds=[
        SEEDS_CITIZEN,
        mint.key().as_ref(),
      ],
      bump,
      space=8+Citizen::get_max_size()
    )]
    pub citizen: Account<'info, Citizen>,
    pub mint: Account<'info, Mint>, 
}

#[derive(Accounts)]
pub struct DeleteCitizenAccount<'info>{
  #[account(
    mut,
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,

  #[account(
    mut,
    close=server
  )]
  pub citizen: Account<'info, Citizen>,
}


#[derive(Accounts)]
pub struct JoinFaction<'info>{
  #[account(
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,

  #[account(mut)]
  pub citizen: Account<'info, Citizen>,
  pub faction: Account<'info, Faction>,
}

#[derive(Accounts)]
pub struct LeaveFaction<'info>{
  #[account(
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,

  #[account(mut)]
  pub citizen: Account<'info, Citizen>,
  #[account(mut)]
  pub faction: Account<'info, Faction>,
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateProposal<'info> {
  #[account(
    mut,
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,

  #[account(
    init, 
    payer=server,
    seeds=[
      SEEDS_PROPOSAL,
      id.as_bytes(),
    ],
    bump,
    space=8+Proposal::get_max_size()
  )]
  pub proposal: Account<'info, Proposal>,
  pub faction: Account<'info, Faction>,
}

#[derive(Accounts)]
pub struct DeleteProposalAccount<'info>{
  #[account(
    mut,
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,

  #[account(
    mut,
    close=server
  )]
  pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct UpdateProposal<'info> {
  #[account(
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,
  pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
  #[account(mut)]
  pub wallet: Signer<'info>,
  #[account(
    token::authority = wallet,
    token::mint = citizen.mint.key()
  )]
  pub wallet_ata: Account<'info, TokenAccount>,

  pub system_program: Program<'info, System>,

  #[account(mut)]
  pub citizen: Account<'info, Citizen>,

  #[account(
    init, 
    payer=wallet,
    space=8+ProposalVote::get_max_size(),
    seeds=[
      SEEDS_VOTE,
      citizen.key().as_ref(),
      proposal.key().as_ref(),
    ],
    bump,
  )]
  pub vote: Account<'info, ProposalVote>,

  #[account(
    constraint = (proposal.status == ProposalStatus::VOTING) && (citizen.faction == Some(proposal.faction))
  )]
  pub proposal: Account<'info, Proposal>,

  #[account(
    constraint = proposal.faction == faction.key()
  )]
  pub faction: Account<'info, Faction>,
}

#[derive(Accounts)]
pub struct UpdateVote<'info> {
  pub wallet: Signer<'info>,
  #[account(
    token::authority = wallet,
    token::mint = citizen.mint.key()
  )]
  pub wallet_ata: Account<'info, TokenAccount>,
  pub system_program: Program<'info, System>,

  #[account(mut)]
  pub citizen: Account<'info, Citizen>,

  #[account(
    mut,
    constraint = vote.citizen == citizen.key()
  )]
  pub vote: Account<'info, ProposalVote>,

  #[account(
    constraint = (proposal.status == ProposalStatus::VOTING) && (citizen.faction == Some(proposal.faction))
  )]
  pub proposal: Account<'info, Proposal>,

  #[account(
    constraint = proposal.faction == faction.key()
  )]
  pub faction: Account<'info, Faction>,
}

#[derive(Accounts)]
pub struct CloseVoteAccount<'info> {
  pub wallet: Signer<'info>,
  #[account(
    token::authority = wallet,
    token::mint = citizen.mint.key()
  )]
  pub wallet_ata: Account<'info, TokenAccount>,
  pub system_program: Program<'info, System>,

  #[account(mut)]
  pub citizen: Account<'info, Citizen>,

  #[account(
    mut,
    close = wallet,
    constraint = vote.citizen == citizen.key()
  )]
  pub vote: Account<'info, ProposalVote>,

  #[account(
    constraint = (proposal.status == ProposalStatus::VOTING) && (citizen.faction == Some(proposal.faction))
  )]
  pub proposal: Account<'info, Proposal>,

  #[account(
    constraint = proposal.faction == faction.key()
  )]
  pub faction: Account<'info, Faction>,
}

#[derive(Accounts)]
pub struct TransferVotes<'info> {
  pub wallet: Signer<'info>,
  #[account(
    token::authority = wallet,
    token::mint = citizen.mint.key()
  )]
  pub wallet_ata: Account<'info, TokenAccount>,
  pub system_program: Program<'info, System>,

  #[account(mut)]
  pub citizen: Account<'info, Citizen>,
  
  #[account(
    mut,
    constraint = citizen.faction == vote_recepient.faction
  )]
  pub vote_recepient: Account<'info, Citizen>,
}

#[derive(Accounts)]
pub struct DelegateVote<'info> {
  #[account(mut)]
  pub wallet: Signer<'info>,
  #[account(
    token::authority = wallet,
    token::mint = citizen.mint.key()
  )]
  pub wallet_ata: Account<'info, TokenAccount>,
  pub system_program: Program<'info, System>,

  #[account(mut)]
  pub citizen: Account<'info, Citizen>,
  
  #[account(
    mut,
    constraint = citizen.faction == vote_recepient.faction
  )]
  pub vote_recepient: Account<'info, Citizen>,

  #[account(
    init,
    payer=wallet,
    seeds=[
      SEEDS_DELEGATION,
      citizen.key().as_ref(),
      vote_recepient.key().as_ref()
    ],
    bump,
    space=8+VoteDelegation::get_max_size(),
  )]
  pub delegation_record: Account<'info, VoteDelegation>,
}

#[derive(Accounts)]
pub struct DeleteVoteDelegation<'info>{
  #[account(
    mut,
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,

  #[account(
    mut,
    close=server
  )]
  pub delegation: Account<'info, VoteDelegation>,
}

#[derive(Accounts)]
pub struct AdjustDelegation<'info> {
  #[account(mut)]
  pub wallet: Signer<'info>,
  #[account(
    token::authority = wallet,
    token::mint = citizen.mint.key()
  )]
  pub wallet_ata: Account<'info, TokenAccount>,
  pub system_program: Program<'info, System>,

  #[account(mut)]
  pub citizen: Account<'info, Citizen>,
  
  #[account(
    mut,
    constraint = citizen.faction == vote_recepient.faction
  )]
  pub vote_recepient: Account<'info, Citizen>,

  #[account(
    mut,
    constraint = (delegation_record.citizen == citizen.key()) && (delegation_record.delegate == vote_recepient.key())
  )]
  pub delegation_record: Account<'info, VoteDelegation>,
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct DiscoverRF<'info> {
  #[account(
    mut,
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,

  #[account(
    init,
    payer=server,
    seeds=[
      SEEDS_RF,
      id.as_bytes(),
    ],
    bump,
    space=8+ResourceField::get_max_size()
  )]
  pub rf: Account<'info, ResourceField>,
}

#[derive(Accounts)]
pub struct DeleteResourceField<'info>{
  #[account(
    mut,
    address = Pubkey::from_str(SERVER_PUBKEY).unwrap()
  )]
  pub server: Signer<'info>,
  pub system_program: Program<'info, System>,

  #[account(
    mut,
    close=server
  )]
  pub resource_field: Account<'info, ResourceField>,
}


#[derive(Accounts)]
pub struct DevelopRF<'info> {
  #[account(mut)]
  pub wallet: Signer<'info>,
  #[account(
    token::authority = wallet,
    token::mint = citizen.mint.key()
  )]
  pub wallet_ata: Account<'info, TokenAccount>,
  pub system_program: Program<'info, System>,
  pub citizen: Account<'info, Citizen>,

  #[account(mut)]
  pub rf: Account<'info, ResourceField>,
  #[account(
    constraint = citizen.faction == Some(faction.key())
  )]
  pub faction: Account<'info, Faction>,
}