use super::{
    dyncode::DynCodeSegment,
    refrences::{StateIdent, TableRefrence},
};

pub enum OperationModifer {
    Filter(String),
    Display(DynCodeSegment),
}

pub enum Operation {
    Random {
        table_refrence: TableRefrence,
        modifiers: Vec<OperationModifer>,
    },
    Weighted {
        table_refrence: TableRefrence,
        modifiers: Vec<OperationModifer>,
    },
    Add {
        state_refrence: StateIdent,
        modifiers: Vec<OperationModifer>,
        value: String,
        auto_next: bool,
    },
    Subtract {
        state_refrence: StateIdent,
        modifiers: Vec<OperationModifer>,
        value: String,
        auto_next: bool,
    },
    Multiply {
        state_refrence: StateIdent,
        modifiers: Vec<OperationModifer>,
        value: String,
        auto_next: bool,
    },
    Divide {
        state_refrence: StateIdent,
        modifiers: Vec<OperationModifer>,
        value: String,
        auto_next: bool,
    },
    AddRandom {
        state_refrence: StateIdent,
        modifiers: Vec<OperationModifer>,
        min: String,
        max: String,
        auto_next: bool,
    },
    SubtractRandom {
        state_refrence: StateIdent,
        modifiers: Vec<OperationModifer>,
        min: String,
        max: String,
        auto_next: bool,
    },
    MultiplyRandom {
        state_refrence: StateIdent,
        modifiers: Vec<OperationModifer>,
        min: String,
        max: String,
        auto_next: bool,
    },
    DivideRandom {
        state_refrence: StateIdent,
        modifiers: Vec<OperationModifer>,
        min: String,
        max: String,
        auto_next: bool,
    },
    Current,
    Previous,
    Next,
    Passed,
    Left,
    RawText,
}
