use super::dyncode::{CardRule, GameTarget};

#[derive(Clone)]
pub struct TableRefrence {
    refrence: usize,
}

impl TableRefrence {
    pub fn new(refrence: usize) -> Self {
        Self { refrence }
    }

    pub fn unpack(self) -> usize {
        self.refrence
    }
}
#[derive(Clone)]
pub struct StateIdent {
    name: String,
}

impl StateIdent {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn unpack(self) -> String {
        self.name
    }
}

pub enum Target {
    TablesTarget(TableRefrence),
    GameTarget(GameTarget),
    StateTarget(StateIdent),
    RulesTarget(CardRule),
    RawText,
}
