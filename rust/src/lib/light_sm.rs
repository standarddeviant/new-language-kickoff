// LightSm.rs
include!("./Light.rs"); // this is a bit of a hack...

#[allow(dead_code)]
#[allow(non_snake_case)] // turn off opinionated rust-analyzer warnings
#[allow(non_camel_case_types)] // turn off opinionated rust-analyzer warnings
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum EventId {
    #[default]
    DIM = 0,
    INC = 1,
    OFF = 2,
}

#[allow(non_upper_case_globals)] // turn off opinionated rust-analyzer warnings
pub const EventId_Count: usize = 3;

#[allow(dead_code)]
#[allow(non_snake_case)] // turn off opinionated rust-analyzer warnings
#[allow(non_camel_case_types)] // turn off opinionated rust-analyzer warnings
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum StateId {
    #[default]
    ROOT = 0,
    OFF = 1,
    ON_GROUP = 2,
    ON1 = 3,
    ON2 = 4,
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)] // turn off opinionated rust-analyzer warnings
pub const StateId_Count: usize = 3;

// Generated state machine
// forward declaration
// typedef struct LightSm LightSm;

// State machine variables. Can be used for inputs, outputs, user variables...
#[allow(dead_code)]
#[derive(Default)]
pub struct LightSm {
    state_id: StateId,
    event_id: EventId,
    // vars
    pub count: isize,
    pub light: Light,
}

#[allow(dead_code)]
impl LightSm {
    // State machine constructor. Must be called before start or dispatch event functions. Not thread safe.
    pub fn new() -> LightSm {
        LightSm {
            state_id: StateId::ROOT,
            event_id: EventId::DIM,
            count: 0 as isize,
            light: Light::default(),
        }
    }

    // Starts the state machine. Must be called before dispatching events. Not thread safe.
    pub fn start(&mut self) {
        self.ROOT_enter();
        // ROOT behavior
        // uml: TransitionTo(ROOT.<InitialState>)
        {
            // Step 1: Exit states until we reach `ROOT` state (Least Common Ancestor for transition). Already at LCA, no exiting required.

            // Step 2: Transition action: ``.

            // Step 3: Enter/move towards transition target `ROOT.<InitialState>`.
            // ROOT.<InitialState> is a pseudo state and cannot have an `enter` trigger.

            // ROOT.<InitialState> behavior
            // uml: TransitionTo(OFF)
            {
                // Step 1: Exit states until we reach `ROOT` state (Least Common Ancestor for transition). Already at LCA, no exiting required.

                // Step 2: Transition action: ``.

                // Step 3: Enter/move towards transition target `OFF`.
                self.OFF_enter();

                // Step 4: complete transition. Ends event dispatch. No other behaviors are checked.
                return;
            } // end of behavior for ROOT.<InitialState>
        } // end of behavior for ROOT
    }

    // Dispatches an event to the state machine. Not thread safe.
    // Note! This function assumes that the `event_id` parameter is valid.
    pub fn dispatch_event(&mut self, event_id: EventId) {
        match self.state_id {
            // STATE: LightSm
            StateId::ROOT => {}
            // No events handled by this state (or its ancestors).

            // STATE: OFF
            StateId::OFF => {
                match event_id {
                    EventId::INC => self.OFF_inc(),
                    _ => {} // to avoid "unused enumeration value in match" warning
                }
            }

            // STATE: ON_GROUP
            StateId::ON_GROUP => match event_id {
                EventId::OFF => self.ON_GROUP_off(),
                #[allow(unreachable_patterns)]
                _ => {} // to avoid "unused enumeration value in match" warning
            },

            // STATE: ON1
            StateId::ON1 => match event_id {
                EventId::INC => self.ON1_inc(),
                EventId::DIM => self.ON1_dim(),
                EventId::OFF => self.ON_GROUP_off(), // First ancestor handler for this event
                #[allow(unreachable_patterns)]
                _ => {} // to avoid "unused enumeration value in match" warning
            },

            // STATE: ON2
            StateId::ON2 => match event_id {
                EventId::INC => self.ON2_inc(),
                EventId::DIM => self.ON2_dim(),
                EventId::OFF => self.ON_GROUP_off(),
                #[allow(unreachable_patterns)]
                _ => {} // to avoid "unused enumeration value in match" warning
            },
        }
    }

    // This function is used when StateSmith doesn't know what the active leaf state is at
    // compile time due to sub states or when multiple states need to be exited.
    fn exit_up_to_state_handler(&mut self, desired_state: StateId) {
        while self.state_id != desired_state {
            match self.state_id {
                StateId::OFF => self.OFF_exit(),

                StateId::ON_GROUP => self.ON_GROUP_exit(),

                StateId::ON1 => self.ON1_exit(),

                StateId::ON2 => self.ON2_exit(),

                _ => {
                    break;
                } // Just to be safe. Prevents infinite loop if state ID memory is somehow corrupted.
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // event handlers for state ROOT
    ////////////////////////////////////////////////////////////////////////////////

    #[allow(non_snake_case)]
    fn ROOT_enter(&mut self) {
        self.state_id = StateId::ROOT;
    }

    ////////////////////////////////////////////////////////////////////////////////
    // event handlers for state OFF
    ////////////////////////////////////////////////////////////////////////////////

    #[allow(non_snake_case)]
    fn OFF_enter(&mut self) {
        self.state_id = StateId::OFF;

        // OFF behavior
        // uml: enter / { Light_off(); }
        {
            // Step 1: execute action `Light_off();`
            self.light.off();
        } // end of behavior for OFF
    }

    #[allow(non_snake_case)]
    fn OFF_exit(&mut self) {
        self.state_id = StateId::ROOT;
    }

    #[allow(non_snake_case)]
    fn OFF_inc(&mut self) {
        // OFF behavior
        // uml: INC TransitionTo(ON1)
        {
            // Step 1: Exit states until we reach `ROOT` state (Least Common Ancestor for transition).
            self.OFF_exit();

            // Step 2: Transition action: ``.

            // Step 3: Enter/move towards transition target `ON1`.
            self.ON_GROUP_enter();
            self.ON1_enter();

            // Step 4: complete transition. Ends event dispatch. No other behaviors are checked.
            return;
        } // end of behavior for OFF

        // No ancestor handles this event.
    }

    ////////////////////////////////////////////////////////////////////////////////
    // event handlers for state ON_GROUP
    ////////////////////////////////////////////////////////////////////////////////

    #[allow(non_snake_case)] // turn off opinionated rust-analyzer warnings
    fn ON_GROUP_enter(&mut self) {
        self.state_id = StateId::ON_GROUP;
    }

    #[allow(non_snake_case)] // turn off opinionated rust-analyzer warnings
    fn ON_GROUP_exit(&mut self) {
        self.state_id = StateId::ROOT;
    }

    #[allow(non_snake_case)] // turn off opinionated rust-analyzer warnings
    fn ON_GROUP_off(&mut self) {
        // ON_GROUP behavior
        // uml: OFF TransitionTo(OFF)
        {
            // Step 1: Exit states until we reach `ROOT` state (Least Common Ancestor for transition).
            self.exit_up_to_state_handler(StateId::ROOT);

            // Step 2: Transition action: ``.

            // Step 3: Enter/move towards transition target `OFF`.
            self.OFF_enter();

            // Step 4: complete transition. Ends event dispatch. No other behaviors are checked.
            return;
        } // end of behavior for ON_GROUP

        // No ancestor handles this event.
    }

    ////////////////////////////////////////////////////////////////////////////////
    // event handlers for state ON1
    ////////////////////////////////////////////////////////////////////////////////

    #[allow(non_snake_case)]
    fn ON1_enter(&mut self) {
        self.state_id = StateId::ON1;

        // ON1 behavior
        // uml: enter / { Light_blue(); }
        {
            // Step 1: execute action `Light_blue();`
            self.light.blue();
        } // end of behavior for ON1
    }

    #[allow(non_snake_case)]
    fn ON1_exit(&mut self) {
        self.state_id = StateId::ON_GROUP;
    }

    #[allow(non_snake_case)]
    fn ON1_dim(&mut self) {
        // ON1 behavior
        // uml: DIM TransitionTo(OFF)
        {
            // Step 1: Exit states until we reach `ROOT` state (Least Common Ancestor for transition).
            self.exit_up_to_state_handler(StateId::ROOT);

            // Step 2: Transition action: ``.

            // Step 3: Enter/move towards transition target `OFF`.
            self.OFF_enter();

            // Step 4: complete transition. Ends event dispatch. No other behaviors are checked.
            return;
        } // end of behavior for ON1

        // No ancestor handles this event.
    }

    #[allow(non_snake_case)]
    fn ON1_inc(&mut self) {
        // ON1 behavior
        // uml: INC TransitionTo(ON2)
        {
            // Step 1: Exit states until we reach `ON_GROUP` state (Least Common Ancestor for transition).
            self.ON1_exit();

            // Step 2: Transition action: ``.

            // Step 3: Enter/move towards transition target `ON2`.
            self.ON2_enter();

            // Step 4: complete transition. Ends event dispatch. No other behaviors are checked.
            return;
        } // end of behavior for ON1

        // No ancestor handles this event.
    }

    ////////////////////////////////////////////////////////////////////////////////
    // event handlers for state ON2
    ////////////////////////////////////////////////////////////////////////////////

    #[allow(non_snake_case)]
    fn ON2_enter(&mut self) {
        self.state_id = StateId::ON2;

        // ON2 behavior
        // uml: enter / { Light_yellow(); }
        {
            // Step 1: execute action `Light_yellow();`
            self.light.yellow();
        } // end of behavior for ON2

        // ON2 behavior
        // uml: enter / { count = 0; }
        {
            // Step 1: execute action `count = 0;`
            self.count = 0;
        } // end of behavior for ON2
    }

    #[allow(non_snake_case)]
    fn ON2_exit(&mut self) {
        self.state_id = StateId::ON_GROUP;
    }

    #[allow(non_snake_case)]
    fn ON2_dim(&mut self) {
        // ON2 behavior
        // uml: DIM TransitionTo(ON1)
        {
            // Step 1: Exit states until we reach `ON_GROUP` state (Least Common Ancestor for transition).
            self.ON2_exit();

            // Step 2: Transition action: ``.

            // Step 3: Enter/move towards transition target `ON1`.
            self.ON1_enter();

            // Step 4: complete transition. Ends event dispatch. No other behaviors are checked.
            return;
        } // end of behavior for ON2

        // No ancestor handles this event.
    }

    #[allow(non_snake_case)]
    fn ON2_inc(&mut self) {
        // ON2 behavior
        // uml: 1. INC / { count+=1; }
        {
            // Step 1: execute action `count+=1;`
            self.count += 1;
        } // end of behavior for ON2

        // ON2 behavior
        // uml: 2. INC [count >= 3] TransitionTo(OFF)
        if self.count >= 3 {
            // Step 1: Exit states until we reach `ROOT` state (Least Common Ancestor for transition).
            self.exit_up_to_state_handler(StateId::ROOT);

            // Step 2: Transition action: ``.

            // Step 3: Enter/move towards transition target `OFF`.
            self.OFF_enter();

            // Step 4: complete transition. Ends event dispatch. No other behaviors are checked.
            return;
        } // end of behavior for ON2

        // No ancestor handles this event.
    }

    // Thread safe.
    fn state_id_to_string(id: StateId) -> String {
        match id {
            StateId::ROOT => "ROOT".into(),
            StateId::OFF => "OFF".into(),
            StateId::ON_GROUP => "ON_GROUP".into(),
            StateId::ON1 => "ON1".into(),
            StateId::ON2 => "ON2".into(),
            #[allow(unreachable_patterns)]
            _ => "?".into(),
        }
    }

    // Thread safe.
    fn event_id_to_string(id: EventId) -> String {
        match id {
            EventId::DIM => return "DIM".into(),
            EventId::INC => return "INC".into(),
            EventId::OFF => return "OFF".into(),
            #[allow(unreachable_patterns)]
            _ => "?".into(),
        }
    }
}
