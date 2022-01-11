// mod gussing_game;
// mod common_concepts;
mod control_flow;
mod ownership;

// use control_flow::looping::loop_returns_value;
use ownership::{slice_type,reference_borrowing};

fn main() {
    // gussing_game::game::start();
    // control_flow::looping::start();
    // loop_returns_value();
    reference_borrowing::ref_borrow();
    slice_type::run();
    // common_concepts::variables_and_mutibility::run();
}
