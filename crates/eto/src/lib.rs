mod diff;
mod state;

use crate::{diff::Diff, state::State};

pub fn package_diff(old: &str, new: &str, output: &str) {
    let old = State::read_dir(old);
    let new = State::read_dir(new);

    let diff = Diff::from_states(&old, &new);
}
