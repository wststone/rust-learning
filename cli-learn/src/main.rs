use clap::Parser;

// struct example
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Too Complicated
    // struct Cli {
    //     pattern: String,
    //     path: std::path::PathBuf,
    // }
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern,
    //     path: std::path::PathBuf::from(path),
    // };
    let mut user_1 = User {
        active: true,
        username: String::from("foo"),
        email: String::from("bar"),
        sign_in_count: 4,
    };

    let user_2 = User {
        active: false,
        ..user_1
    };

    // Tuple Struct

    struct Cli {
        pattern: String,
    }
}
