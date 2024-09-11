use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Deck {
    pub deck_rules: DeckRules,
    pub default_card_rules: DefaultCardRules,
    pub cards: Vec<Card>,
    pub tables: Vec<Table>,
    pub states: Vec<State>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeckRules {
    pub win_condition: WinCondition,
    pub max_cards: i64,
    pub max_players: i64,
    pub scoreboard: Scoreboard,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WinConditionType {
    GreatestValue,
    LowestValue,
    ClosestToValue,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WinConditionCheck {
    EndOfGame,
    EachFullTurn,
    Constant,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortingAlgorithm {
    Descending,
    ClosestToValue,
    Rising,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WinCondition {
    pub state_refrence: String,
    pub condition: WinConditionType,
    pub check: WinConditionCheck,
    pub value: i64,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Scoreboard {
    pub state_refrence: String,
    pub algorithm: SortingAlgorithm,
    pub value: i64,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateDisplay {
    Ordered,
    Random,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DefaultCardRules {
    pub state_display: StateDisplay,
    pub occurance_limit: i64,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CardRules {
    pub state_display: StateDisplay,
    pub occurance_limit: i64,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Card {
    pub id: Uuid,
    pub raw: String,
    pub rules: CardRules,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Value {
    pub value: String,
    pub weight: i64,
    pub tags: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Table {
    pub name: String,
    pub values: Vec<Value>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct State {
    pub name: String,
    pub min: i64,
    pub max: i64,
    pub value: i64,
}

pub type Tables = Vec<Table>;
