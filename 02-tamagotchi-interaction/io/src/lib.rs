#![no_std]

use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use gstd::prelude::*;
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
    // TODO: 1️⃣ Add new fields
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
    // TODO: 2️⃣ Add new actions
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
    // TODO: 3️⃣ Add new events
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Tamagotchi;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
