use crate::game::Game;
use crate::error::TournamentError;

use anyhow::Result;
use uuid::Uuid;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RoundStatus {
    Open,
    Uncertified,
    Certified,
    Dead,
}

pub enum Outcome {
    Game(Game),
    Round(Vec<Game>),
}

// This struct should be able to handle N many games, unlike the Python equiv.
pub struct Round {
    pub(crate) uuid: Uuid,
    pub(crate) match_number: u64,
    pub(crate) players: HashSet<Uuid>,
    confirmations: HashSet<Uuid>,
    games: Vec<Game>,
    status: RoundStatus,
    winner: Option<Uuid>,
    timer: Instant,
    length: Duration,
    extension: Duration,
    is_bye: bool,
}

impl Hash for Round {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let _ = &self.uuid.hash(state);
    }
}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Round {
    pub fn new(match_num: u64, len: Duration) -> Self {
        Round {
            uuid: Uuid::new_v4(),
            match_number: match_num,
            players: HashSet::with_capacity(4),
            confirmations: HashSet::with_capacity(4),
            games: Vec::with_capacity(3),
            status: RoundStatus::Open,
            winner: None,
            timer: Instant::now(),
            length: len,
            extension: Duration::from_secs(0),
            is_bye: false,
        }
    }
    
    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    
    pub fn add_player(&mut self, player: Uuid) {
        self.players.insert(player);
    }

    fn verify_game(&self, game: &Game) -> bool {
        match game.winner {
            Some(p) => self.players.contains(&p),
            None => true,
        }
    }

    pub fn record_outcome(&mut self, outcome: Outcome) -> Result<(), TournamentError> {
        match outcome {
            Outcome::Game(g) => {
                self.record_game(g)?;
            }
            Outcome::Round(r) => {
                for g in r {
                    self.record_game(g)?;
                }
            }
        }
        Ok(())
    }

    pub fn record_game(&mut self, game: Game) -> Result<(), TournamentError> {
        if self.verify_game(&game) {
            self.games.push(game);
            self.confirmations.clear();
            Ok(())
        } else {
            Err(TournamentError::InvalidGame)
        }
    }
    pub fn confirm_round(&mut self, player: Uuid) -> Result<RoundStatus, TournamentError> {
        if !self.players.contains(&player) {
            Err(TournamentError::PlayerNotInRound)
        } else {
            self.confirmations.insert(player);
            if self.confirmations.len() == self.players.len() {
                Ok(RoundStatus::Certified)
            } else {
                Ok(RoundStatus::Uncertified)
            }
        }
    }
    pub fn kill_round(&mut self) {
        self.status = RoundStatus::Dead;
    }
    pub fn record_bye(&mut self) -> Result<(), TournamentError> {
        if self.players.len() != 1 {
            Err(TournamentError::InvalidBye)
        } else {
            self.is_bye = true;
            Ok(())
        }
    }
    pub fn clear_game_record(&mut self) {
        self.games.clear();
    }
    pub fn is_certified(&self) -> bool {
        self.status == RoundStatus::Certified
    }
}

pub fn parse_to_outcome(input: String) -> Result<Outcome, TournamentError> {
    todo!()
}
