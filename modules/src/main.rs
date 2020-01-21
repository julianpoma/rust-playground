mod util;
mod helper;

use helper::helper_utils;

fn main() {
    util::util_caller();
    helper_utils::helper_caller();
    helper_utils::deep_caller();
    helper::mod_main();
}
