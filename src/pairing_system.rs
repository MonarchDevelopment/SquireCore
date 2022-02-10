pub use crate::error::TournamentError;
pub use crate::player_registry::PlayerRegistry;
pub use crate::round_registry::RoundRegistry;
pub use crate::player::PlayerId;

pub use std::collections::HashMap;

pub trait PairingSystem
where
    Self: Send + Sync
{
    fn new(players_per_match: u8) -> Self
    where
        Self: Sized;

    // This bool communitates if pairings should be created
    fn ready_player(&mut self, plyr: PlayerId) -> bool;

    fn update_settings(&mut self, settings: HashMap<String, String>) -> String;

    fn suggest_pairings(
        &self,
        size: u8,
        players: &PlayerRegistry,
        matches: &RoundRegistry,
    ) -> Option<Vec<Vec<PlayerId>>>;

    fn rollback_pairings(
        &self,
        players: &mut PlayerRegistry,
        matches: &mut RoundRegistry,
    ) -> Result<(), TournamentError>;
}
