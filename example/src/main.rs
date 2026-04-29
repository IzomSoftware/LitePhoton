use LitePhoton::input::Input;
use LitePhoton::input::print_input;
use std::path::PathBuf;

fn main() {
    let input = Input::File(PathBuf::from("./test_run/test.txt"));

    unsafe {
        print_input::input(
            LitePhoton::common::Method::Rayon,
            input,
            true,
            "test".into(),
        )
        .expect("Cannot iterate over file");
    }
}
