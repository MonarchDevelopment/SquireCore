use crate::player::{Player, PlayerStatus};

use uuid::Uuid;
use mtgjson::model::deck::Deck;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum PlayerIdentifier {
    Id(Uuid),
    Name(String),
}

pub struct PlayerRegistry {
    players: HashMap<Uuid, Player>,
}

impl PlayerRegistry {
    pub fn new() -> Self {
        PlayerRegistry {
            players: HashMap::new(),
        }
    }
    
    pub fn add_player(&mut self, name: String) -> Result<(), ()> {
        if self.verify_identifier(&PlayerIdentifier::Name(name.clone())) {
            Err(())
        } else {
            let plyr = Player::new(name);
            self.players.insert(plyr.uuid, plyr);
            Ok(())
        }
    }
    
    pub fn add_deck(&mut self, ident: PlayerIdentifier, deck: Deck) -> Result<(),()> {
        let id = self.get_player_id(ident)?;
        // Saftey check, we just verified that the id was valid
        let plyr = self.players.get_mut(&id).unwrap();
        plyr.add_deck(deck);
        Ok(())
    }

    pub fn get_player_id(&self, ident: PlayerIdentifier) -> Result<Uuid, ()> {
        match ident {
            PlayerIdentifier::Id(id) => {
                if self.verify_identifier(&PlayerIdentifier::Id(id)) {
                    Ok(id)
                } else {
                    Err(())
                }
            }
            PlayerIdentifier::Name(name) => {
                let ids: Vec<Uuid> = self
                    .players
                    .iter()
                    .filter(|(_, p)| p.name == name)
                    .map(|(i, _)| i.clone())
                    .collect();
                if ids.len() != 1 {
                    Err(())
                } else {
                    Ok(ids[0])
                }
            }
        }
    }

    pub fn drop_player(&mut self, ident: PlayerIdentifier) -> Result<(), ()> {
        match self.get_player_id(ident) {
            Ok(id) => {
                let plyr = self.players.get_mut(&id).unwrap();
                plyr.update_status(PlayerStatus::Dropped);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn remove_player(&mut self, ident: PlayerIdentifier) -> Result<(), ()> {
        match self.get_player_id(ident) {
            Ok(id) => {
                let plyr = self.players.get_mut(&id).unwrap();
                plyr.update_status(PlayerStatus::Removed);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn verify_identifier(&self, ident: &PlayerIdentifier) -> bool {
        match ident {
            PlayerIdentifier::Id(id) => self.players.contains_key(id),
            PlayerIdentifier::Name(name) => self.players.iter().any(|(_, p)| p.name == *name),
        }
    }
}
