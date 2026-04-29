use std::path::PathBuf;
use LitePhoton::input::Input;
use LitePhoton::print_input;
use LitePhoton::print_input::Mode;

fn main() {
    let input = Input::File(
        PathBuf::from("./test_run/test.txt")
    );

    print_input::read_input(Mode::Chunk, input, true, "test");
}
