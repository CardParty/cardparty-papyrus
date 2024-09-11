// dyncode is a ir of operations rendering and practicly all operations in the cardparty architecture.

// pub struct CardDynCode {
//   cards:
// }

use super::{error::EditorError, operations::Operation, refrences::Target};

pub enum ParserSegment {
    DynCode(String),
    RawText(String),
}

pub enum DynCodeSegmentType {
    RawText,
    Code,
    Error,
    Recursive,
}

pub enum DynCodeRoot {
    Tables,
    Game,
    State,
    Rules,
    RawText,
}

pub enum GameTarget {
    Players,
    Cards,
    Null,
}

pub enum StateDisplayOptions {
    Random,
    Ordered,
    ByState,
    Null,
}

pub enum OccuraneLimitOptions {
    Finite(i32),
    Infinite,
}

pub struct Rules {
    state_display: StateDisplayOptions,
    display_amount: OccuraneLimitOptions,
}

pub enum CardRule {
    StateDisplay(StateDisplayOptions),
    DisplayAmount(OccuraneLimitOptions),
}

pub struct DynCodeSegment {
    segment_type: DynCodeSegmentType,
    raw_string: String,
    root: DynCodeRoot,
    target: Target,
    operation: Operation,
    error: EditorError,
}

impl DynCodeSegment {
    pub fn new(
        segment_type: DynCodeSegmentType,
        raw_string: String,
        root: DynCodeRoot,
        target: Target,
        operation: Operation,
        error: EditorError,
    ) -> Self {
        Self {
            segment_type,
            raw_string,
            root,
            target,
            operation,
            error,
        }
    }
}

pub struct CardDynCode {
    rules: Rules,
    segments: Vec<DynCodeSegment>,
}

pub enum DynCodeError {
    None,
}
