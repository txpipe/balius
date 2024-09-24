pub trait Handler: Send + Sync {
    fn handle(
        &self,
        evt: crate::balius::odk::driver::Event,
    ) -> Result<crate::balius::odk::driver::Response, crate::balius::odk::driver::HandleError>;
}

struct State {}

impl From<crate::balius::odk::driver::Event> for State {
    fn from(evt: crate::balius::odk::driver::Event) -> Self {
        State {}
    }
}

use paste::paste;

#[macro_export]
macro_rules! handler_wrapper {
    ($func:ident) => {
        paste! {
            struct [<$func Handler>];

            impl Handler for [<$func Handler>] {
                fn handle(
                    &self,
                    evt: crate::balius::odk::driver::Event,
                ) -> Result<crate::balius::odk::driver::Response, crate::balius::odk::driver::HandleError> {
                    let state = State::from(evt);
                    $func(state).map(|result_state| crate::balius::odk::driver::Response::from(result_state))
                }
            }
        }
    };
}
