use std::vec;

use serde_json::error;

use super::{
    abstraction::TableRegister,
    deserialization::{Deck, Table},
    dyncode::{DynCodeSegment, ParserSegment},
    error::{CompilerError, EditorError},
};

fn extract_parts(input: &str) -> Vec<ParserSegment> {
    let mut segments = Vec::new();
    let mut start_idx = None;
    let mut in_braces = false;

    for (i, c) in input.char_indices() {
        match c {
            '{' if !in_braces => {
                if let Some(start) = start_idx {
                    segments.push(ParserSegment::RawText(input[start..i].to_string()));
                }
                start_idx = Some(i + 1);
                in_braces = true;
            }
            '}' if in_braces => {
                if let Some(start) = start_idx {
                    segments.push(ParserSegment::DynCode(input[start..i].to_string()));
                }
                start_idx = Some(i + 1);
                in_braces = false;
            }
            _ => {}
        }
    }

    if let Some(start) = start_idx {
        if in_braces {
            segments.push(ParserSegment::DynCode(input[start..].to_string()));
        } else {
            segments.push(ParserSegment::RawText(input[start..].to_string()));
        }
    }

    segments
}

pub struct Compiler {
    deck: Option<Deck>,
    table_register: Option<TableRegister>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            deck: None,
            table_register: None,
        }
    }

    pub fn deck(mut self, deck: Deck) {
        self.deck = Some(deck)
    }

    pub fn generate_registers(mut self) {
        if let Some(deck) = self.deck {
            self.table_register = Some(TableRegister::generate_register(deck.tables));
        }
    }

    pub fn compile_segments(self, raw: String) -> Result<Vec<DynCodeSegment>, EditorError> {
        let raw_string: String = raw.clone();
        let segments: Vec<ParserSegment> = extract_parts(&raw[..]);
        let mut code_segments: Vec<DynCodeSegment> = Vec::new();

        self.generate_registers();

        for segment in segments {
            match segment {
                ParserSegment::DynCode(raw) => {
                    let mut segment_type;
                    let raw_string: String = raw.clone();

                    let split: Vec<&str> = raw_string.split('.').collect();

                    let raw_operation = split.pop().unwrap();
                    let raw_target = split.pop().unwrap();
                    let raw_root = split.pop().unwrap();

                    let root = match raw_root {
                        "tables" => super::dyncode::DynCodeRoot::Tables,
                        "game" => super::dyncode::DynCodeRoot::Game,
                        "rules" => super::dyncode::DynCodeRoot::Rules,
                        "state" => super::dyncode::DynCodeRoot::State,
                        _ => return Err(EditorError::InvalidCodeRoot(String::from(raw_root))),
                    };

                    match root {
                        super::dyncode::DynCodeRoot::Tables => {
                            if let Some(register) = self.table_register {
                                if let Ok(refrence) =
                                    register.get_table_refrence(String::from(raw_target))
                                {
                                    let query = register.query_table(refrence);
                                } else {
                                    return Err(EditorError::UnknownTableRefrence(String::from(
                                        raw_target,
                                    )));
                                }
                            }
                        }
                        super::dyncode::DynCodeRoot::Game => todo!(),
                        super::dyncode::DynCodeRoot::State => todo!(),
                        super::dyncode::DynCodeRoot::Rules => todo!(),
                        super::dyncode::DynCodeRoot::RawText => todo!(),
                    }

                    code_segments.push(DynCodeSegment::new(
                        segment_type,
                        raw_string,
                        root,
                        target,
                        operation,
                        error,
                    ))
                }
                ParserSegment::RawText(raw) => code_segments.push(DynCodeSegment::new(
                    super::dyncode::DynCodeSegmentType::RawText,
                    raw_string,
                    super::dyncode::DynCodeRoot::RawText,
                    super::refrences::Target::RawText,
                    super::operations::Operation::RawText,
                    super::error::EditorError::Null,
                )),
            }
        }

        Ok(code_segments)
    }

    // pub fn generate_dyncode() -> CardDynCode {}
}
