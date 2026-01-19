/// Macro to generate boilerplate code for use cases
///
/// This macro takes the following sections:
/// - Input: defines the input struct fields
/// - State: defines the state struct fields
/// - Dependencies: defines the dependencies struct fields
/// - Error: defines the error enum variants
/// - UseCase: defines the method names that should be implemented
#[macro_export]
macro_rules! use_case {
    (
        Input {
            $($input_field:ident: $input_type:ty,)*
        }

        State {
            $($state_field:ident: $state_type:ty,)*
        }

        Dependencies {
            $($dep_field:ident: $dep_type:ty,)*
        }

        Error {
            $($error_variant:ident,)*
        };

        Story {
            $($method_name:ident)*
        }

        Steps {
            $(
                $step_name:ident $step_function:expr
            )*
        }
    ) => {
        use teloc::{inject};

        pub struct Input {
            $(pub $input_field: $input_type,)*
        }

        #[derive(Default)]
        pub struct State {
            $(pub $state_field: Option<$state_type>,)*
        }

        pub struct UseCase {
            input: Option<Input>,
            state: State,
            $($dep_field: $dep_type,)*
        }

        pub enum Error {
            $($error_variant,)*
        }

        #[inject]
        impl UseCase {
            pub fn new(
                $($dep_field: $dep_type,)*
            ) -> Self {
                Self {
                    input: None,
                    state: State::default(),
                    $($dep_field,)*
                }
            }
        }

        impl UseCase {
            pub fn run(&mut self, input: Input) -> Result<(), Error> {
                self.input = Some(input);
                $(self.$method_name()?;)*
                Ok(())
            }

            pub fn state(&self) -> &State {
                &self.state
            }

            fn input(&self) -> &Input {
                self.input.as_ref().unwrap()
            }

            // Generate step methods with inline implementations
            $(
                fn $step_name(&mut self) -> Result<(), Error> {
                    $step_function(self)
                }
            )*
        }
    };
}
