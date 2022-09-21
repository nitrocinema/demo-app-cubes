use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};
use near_sdk::serde::{Serialize};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Cubes {
    first: u8,
    second: u8,
    response: CubeResponse
}

impl Default for Cubes {
    fn default() -> Self {
        Self {
            first: 0,
            second: 0,
            response: CubeResponse {
                is_valid: false,
                msg: "".to_string(),
                winner: Winners::Same
            }
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct CubeResponse {
    is_valid: bool,
    msg: String,
    winner: Winners,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
enum Winners {
    Same,
    First,
    Second
}

#[near_bindgen]
impl Cubes {
    pub fn new() -> Self {
        Self {
            first: 0,
            second: 0,
            response: CubeResponse {
                is_valid: false,
                msg: "".to_string(),
                winner: Winners::Same
            }
        }
    }

    #[payable]
    pub fn check(&mut self, first :[u8; 3]) -> u8 {
        first[0]
        /*
        self.check_cubes();

        if !self.response.is_valid {
            return self.response.msg.to_string();
        }

        self.set_winner();

        match self.response.winner {
            Winners::Same => "We have no winner, cubes are same :(".to_string(),
            Winners::First => "First is winner!!!".to_string(),
            Winners::Second => "Second is winner!!!".to_string(),
        }

         */
    }
    /*
    #[private]
    fn is_same(&self) -> bool {
        self.first == self.second
    }

    #[private]
    fn is_first_bigger(&self) -> bool {
        self.first > self.second
    }

    #[private]
    fn check_cubes(&mut self) {
        self.response = CubeResponse {
            is_valid: true,
            msg: "All is ok!".to_string(),
            winner: Winners::Same
        };

        if self.first > 6 || self.first < 1 {
            self.response.is_valid = false;
            self.response.msg = "First cube is not valid!".to_string();
        } else if self.second > 6 || self.second < 1 {
            self.response.is_valid = false;
            self.response.msg = "Second cube is not valid!".to_string();
        }
    }

    #[private]
    fn set_winner(&mut self) {
        if self.is_same() {
            self.response.winner = Winners::Same;
            return;
        }

        if self.is_first_bigger() {
            self.response.winner = Winners::First;
        } else {
            self.response.winner = Winners::Second;
        }
    }
     */
}
