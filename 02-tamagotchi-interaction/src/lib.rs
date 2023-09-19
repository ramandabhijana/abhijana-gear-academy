#![no_std]

use gstd::{exec, msg, prelude::*};
use tmg2_io::{Tamagotchi, TmgAction, TmgEvent};

const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;
const MIN_MOOD_VALUE: u64 = 1;
const MAX_MOOD_VALUE: u64 = 10_000;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern "C" fn init() {
    let name: String = msg::load().expect("Can't decode tamagotchi name");
    let date_of_birth: u64 = exec::block_timestamp();
    let owner = msg::source();
    let current_block: u64 = exec::block_height().into();

    unsafe {
        TAMAGOTCHI = Some(Tamagotchi {
            name,
            date_of_birth,
            owner,
            fed: 1000,
            fed_block: current_block,
            entertained: 1000,
            entertained_block: current_block,
            slept: 1000,
            slept_block: current_block,
        })
    }

    msg::reply("Successfully Initialized", 0).expect("Initialization failed");
}

#[no_mangle]
extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Error in loading TmgAction");
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };

    update_mood(tamagotchi);

    match action {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tamagotchi.name.clone()), 0)
                .expect("Error in sending reply TmgEvent::Name");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tamagotchi.date_of_birth;
            msg::reply(TmgEvent::Age(age), 0).expect("Error in sending reply TmgEvent::Age");
        }
        TmgAction::Feed => {
            let fed = tamagotchi.fed + FILL_PER_FEED;
            let fed = fed.clamp(MIN_MOOD_VALUE, MAX_MOOD_VALUE);
            tamagotchi.fed = fed;
            msg::reply(TmgEvent::Fed, 0).expect("Error in sending reply TmgEvent::Fed");
        }
        TmgAction::Entertain => {
            let entertained = tamagotchi.entertained + FILL_PER_ENTERTAINMENT;
            let entertained = entertained.clamp(MIN_MOOD_VALUE, MAX_MOOD_VALUE);
            tamagotchi.entertained = entertained;
            msg::reply(TmgEvent::Entertained, 0)
                .expect("Error in sending reply TmgEvent::Entertained");
        }
        TmgAction::Sleep => {
            let slept = tamagotchi.slept + FILL_PER_SLEEP;
            let slept = slept.clamp(MIN_MOOD_VALUE, MAX_MOOD_VALUE);
            tamagotchi.slept = slept;
            msg::reply(TmgEvent::Slept, 0).expect("Error in sending reply TmgEvent::Slept");
        }
    }
}

fn update_mood(tamagotchi: &mut Tamagotchi) {
    let current_block: u64 = exec::block_height().into();

    // update fed
    tamagotchi.fed = tamagotchi
        .fed
        .saturating_sub((current_block - tamagotchi.fed_block) * HUNGER_PER_BLOCK);

    // update entertained
    tamagotchi.entertained = tamagotchi
        .entertained
        .saturating_sub((current_block - tamagotchi.entertained_block) * BOREDOM_PER_BLOCK);

    // update slept
    tamagotchi.slept = tamagotchi
        .slept
        .saturating_sub((current_block - tamagotchi.slept_block) * ENERGY_PER_BLOCK);

    tamagotchi.fed_block = current_block;
    tamagotchi.entertained_block = current_block;
    tamagotchi.slept_block = current_block;
}

#[no_mangle]
extern "C" fn state() {
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_ref()
            .expect("The contract is not initialized")
    };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}
