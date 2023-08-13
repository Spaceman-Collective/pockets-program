## Pockets Faction Program

- Hard code in Server Key

- Create a Faction PDA
- Citizen PDA sits on top of Faction PDA and tracks Voting Power
- Faction PDA has voting power threshold
- Voting Power can be transferred between Faction PDAs given they are of the same faction
- When leaving a faction, Voting Power is burned, and Faction PDA has max power reduced by voting power that left.

## Vote

-> Citizen.TOTAL
-> CitizenVote PDA + Proposal PDA
-> Citizen can transfer any Voting Power not in CitizenVote PDA
-> Citizen can vote up to their total voting power on any proposal
-> Citizen can transfer total - inprogress to people
-> "current transferable voting power" is total voting power - max pleged voting power to any proposal
-> Can citizens delegate voting power?
  -> Delgate PDA, lock up like proposal. 