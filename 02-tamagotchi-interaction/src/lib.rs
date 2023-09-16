#![no_std]

use gstd::{debug, exec, msg, prelude::*};
use tmg2_io::{Tamagotchi, TmgAction, TmgEvent};

// TODO: 4️⃣ Define constants
static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern "C" fn init() {
    let name: String = msg::load().expect("Can't decode tamagotchi name");
    let date_of_birth: u64 = exec::block_timestamp();
    debug!(
        "Tamagotchi was initialized with name={:?} and date_of_birth={:?}",
        name, date_of_birth
    );
    unsafe { TAMAGOTCHI = Some(Tamagotchi { name, date_of_birth }) }
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
            debug!("TmgAction::Name {:?}", tamagotchi.name);
            msg::reply(TmgEvent::Name(tamagotchi.name.clone()), 0)
                .expect("Error in sending reply TmgEvent::Name");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tamagotchi.date_of_birth;
            debug!("TmgAction::Age {:?}", age);
            msg::reply(TmgEvent::Age(age), 0).expect("Error in sending reply TmgEvent::Age");
        }
    }
    // TODO: 5️⃣ Add new logic for calculating the `fed`, `entertained` and `slept` levels
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
