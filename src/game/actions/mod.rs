pub use self::action::Action;
pub use self::action_impl::ActionImpl;
pub use self::action_type::ActionType;

mod action;
mod action_impl;
mod action_type;
pub mod implements;

pub enum ActionPossibility {
    Yes(u32),   // length in ticks
    No(String), // reason why not
}
