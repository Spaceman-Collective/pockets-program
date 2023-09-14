pub const SERVER_PUBKEY: &str = "pockmZEzU9m8bkgHMtA3bmHASDFKv8N3kwkEbvQdd9K";
pub const SEEDS_FACTION: &[u8; 7] = b"faction";
pub const SEEDS_CITIZEN: &[u8; 7] = b"citizen";
pub const SEEDS_PROPOSAL: &[u8; 8] = b"proposal";
pub const SEEDS_VOTE: &[u8; 4] = b"vote";
pub const SEEDS_DELEGATION: &[u8; 8] = b"delegate";
pub const SEEDS_RF: &[u8; 2] = b"rf";

pub const MAX_HARVEST_TYPES: usize = 1; // Max number of harvest types you can have in a resource field
pub const LONGEST_RESOURCE_NAME: usize = 8; // character count of the longest resource name
pub const RESOURCES: [&str; 6] = ["Cables", "Soil", "Flowers", "Bandages", "Ingots", "Stone"];
pub const RF_CHANCE: u64 = 50000; // 1 in 75000 chance to find a RF, improved with every time it's developed
pub const RF_MIN_YIELD: u64 = 5;
pub const RF_MAX_YIELD: u64 = 15;
pub const RF_MAX_TIMER: u64 = 7200000; // 2 hrs
pub const RF_MIN_TIMER: u64 = 300000; // 5 m
