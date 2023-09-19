use gtest::{Log, Program, System};
use tmg2_io::{TmgAction, TmgEvent};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    let tamagotchi_name = "Jack";
    let res = _program.send(2, String::from(tamagotchi_name));
    assert!(!res.main_failed());

    let res = _program.send(3, TmgAction::Age);
    let expected_log = Log::builder().dest(3).payload(TmgEvent::Age(0));
    assert!(res.contains(&expected_log));

    let res = _program.send(4, TmgAction::Name);
    let expected_log = Log::builder()
        .dest(4)
        .payload(TmgEvent::Name(tamagotchi_name.to_owned()));
    assert!(res.contains(&expected_log));
}

#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    let tamagotchi_name = "Kaori";
    let res = _program.send(2, String::from(tamagotchi_name));
    assert!(!res.main_failed());

    let res = _program.send(3, TmgAction::Feed);
    let expected_log = Log::builder().dest(3).payload(TmgEvent::Fed);
    assert!(res.contains(&expected_log));

    let res = _program.send(3, TmgAction::Entertain);
    let expected_log = Log::builder().dest(3).payload(TmgEvent::Entertained);
    assert!(res.contains(&expected_log));

    let res = _program.send(3, TmgAction::Sleep);
    let expected_log = Log::builder().dest(3).payload(TmgEvent::Slept);
    assert!(res.contains(&expected_log));
}
