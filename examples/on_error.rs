use rust_utils::errors::OnError as _;

fn main() {
    let _thing_guard = Thing;
}

struct Thing;

impl Drop for Thing {
    fn drop(&mut self) {
        // 1. This is annoying, because the result forces us to use the return value. But we don't need it.
        // let _ = shutdown_handler().inspect_err(|error| println!("{error:?} error shutting down"));

        // 2. This is a lot of work just to log a message on failure
        // if let Err(error) = shutdown_handler() {
        //     println!("{error:?} error shutting down");
        // }

        // 3. This is much cleaner. No if/match, no unused assignments
        shutdown_handler().on_error(|error| println!("error shutting down: {error}"));
    }
}

// Some fallible shutdown handler
fn shutdown_handler() -> Result<(), Box<dyn std::error::Error>> {
    println!("shutting down");
    Err("something went wrong".into())
}
