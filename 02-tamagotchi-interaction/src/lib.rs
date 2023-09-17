#![no_std]

use gstd::{debug, exec, msg, prelude::*};
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
    let current_block = exec::block_height();

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
            // To determine the number of blocks when the Tamagotchi last ate
            let current_block = exec::block_height();

            // we get to update the fed and fed_block

            // we use saturating_sub so that the subtraction result won't be below 0
            let fed = tamagotchi
                .fed
                .saturating_sub((current_block - tamagotchi.fed_block) * HUNGER_PER_BLOCK);

            // calculate amount needed to feed
            let fed = fed + FILL_PER_FEED;

            // we make sure that amount must be in correct range
            let fed = cmp::min(fed, MAX_MOOD_VALUE);
            let fed = cmp::max(fed, MIN_MOOD_VALUE);

            tamagotchi.fed = fed;
            tamagotchi.fed_block = current_block;
        }
        TmgAction::Entertain => todo!(),
        TmgAction::Sleep => todo!(),
    }
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
