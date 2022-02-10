use crate::error::TournamentError;
use crate::tournament::{Tournament, TournamentId};

use mtgjson::model::deck::Deck;

use std::{
    collections::{hash_map::Iter, HashMap},
    slice::SliceIndex,
};

#[derive(Debug, Clone)]
pub enum TournIdentifier {
    Id(TournamentId),
    Name(String),
}

pub struct TournamentRegistry {
    tourns: HashMap<TournamentId, Tournament>,
}

impl Default for TournamentRegistry {
    fn default() -> Self {
        TournamentRegistry::new()
    }
}

impl TournamentRegistry {
    pub fn new() -> Self {
        TournamentRegistry {
            tourns: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.tourns.len()
    }

    pub fn iter(&self) -> Iter<TournamentId, Tournament> {
        self.tourns.iter()
    }

    pub fn get_mut_tourn(
        &mut self,
        ident: TournIdentifier,
    ) -> Result<&mut Tournament, TournamentError> {
        let id = self.get_player_id(ident)?;
        // Saftey check, we just verified that the id was valid
        Ok(self.tourns.get_mut(&id).unwrap())
    }

    pub fn get_player(&self, ident: TournIdentifier) -> Result<&Tournament, TournamentError> {
        let id = self.get_player_id(ident)?;
        // Saftey check, we just verified that the id was valid
        Ok(self.tourns.get(&id).unwrap())
    }

    pub fn get_player_id(&self, ident: TournIdentifier) -> Result<TournamentId, TournamentError> {
        match ident {
            TournIdentifier::Id(id) => {
                if self.verify_identifier(&TournIdentifier::Id(id)) {
                    Ok(id)
                } else {
                    Err(TournamentError::PlayerLookup)
                }
            }
            TournIdentifier::Name(name) => {
                let ids: Vec<TournamentId> = self
                    .tourns
                    .iter()
                    .filter(|(_, t)| t.name == name)
                    .map(|(i, _)| *i)
                    .collect();
                if ids.len() != 1 {
                    Err(TournamentError::PlayerLookup)
                } else {
                    Ok(ids[0])
                }
            }
        }
    }

    pub fn verify_identifier(&self, ident: &TournIdentifier) -> bool {
        match ident {
            TournIdentifier::Id(id) => self.tourns.contains_key(id),
            TournIdentifier::Name(name) => self.tourns.iter().any(|(_, p)| p.name == *name),
        }
    }
}
