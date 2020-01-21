mod util;
mod helper;

use helper::helper_utils;
// If we use this, we can call helper utils functions like this
// helper_utils::helper_caller();

fn main() {
    util::util_caller();
    
    // Two ways of calling the helper_utils functions
    helper::helper_utils::helper_caller();
    // We need the use above to do this
    helper_utils::deep_caller();

    helper::mod_main();
}
