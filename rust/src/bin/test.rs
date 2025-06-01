#[allow(non_snake_case)] // turn off opinionated rust-analyzer warnings

//
use LightSmLib::LightSm;

// #include "Light.h"
// #include "LightSm.h"
// #include <assert.h>
// #include <string.h>
// #include <stdio.h>

// we use a macro here (instead of a helper function) so that we get easy assert messages with line numbers
#[allow(non_snake_case)]
fn ASSERT_LIGHT_COLOR(sm: &LightSm, expected: String) {
    assert!(sm.light.get_color().eq(&expected))
}

// prototypes
// fn test_inc_events();
// fn test_dim_events();
// fn test_off_events();

fn main() {
    test_inc_events();
    test_dim_events();
    test_off_events();

    println!("all tests passed!!!");
}

fn test_inc_events() {
    println!("test inc events");

    // LightSm_ctor(&sm);
    let mut sm: LightSm = LightSm::default();
    sm.start();

    ASSERT_LIGHT_COLOR(&sm, "off".into());

    sm.dispatch_event(LightSmLib::EventId::INC);
    ASSERT_LIGHT_COLOR(&sm, "blue".into());

    sm.dispatch_event(LightSmLib::EventId::INC);
    ASSERT_LIGHT_COLOR(&sm, "yellow".into());
    assert!(sm.count == 0);

    // count starts going up
    sm.dispatch_event(LightSmLib::EventId::INC);
    ASSERT_LIGHT_COLOR(&sm, "yellow".into());
    assert!(sm.count == 1);

    sm.dispatch_event(LightSmLib::EventId::INC);
    ASSERT_LIGHT_COLOR(&sm, "yellow".into());
    assert!(sm.count == 2);

    // count reaches 3, so we go back to off
    sm.dispatch_event(LightSmLib::EventId::INC);
    ASSERT_LIGHT_COLOR(&sm, "off".into());
    assert!(sm.count == 3);
}

fn test_dim_events() {
    println!("test dim events");

    let mut sm: LightSm = LightSm::default();
    // LightSm_ctor(&sm);
    sm.start();

    ASSERT_LIGHT_COLOR(&sm, "off".into());

    sm.dispatch_event(LightSmLib::EventId::INC);
    ASSERT_LIGHT_COLOR(&sm, "blue".into());

    sm.dispatch_event(LightSmLib::EventId::DIM);
    ASSERT_LIGHT_COLOR(&sm, "off".into());
}

fn test_off_events() {
    println!("test off events");

    let mut sm: LightSm = LightSm::default();
    // LightSm_ctor(&sm);
    sm.start();

    sm.dispatch_event(LightSmLib::EventId::INC);
    ASSERT_LIGHT_COLOR(&sm, "blue".into());

    sm.dispatch_event(LightSmLib::EventId::OFF);
    ASSERT_LIGHT_COLOR(&sm, "off".into());
}
