mod cli_parser;

use std::{
    fs::File,
    io::{Result, Write},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use cli_parser::Opt;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let file = File::create(opt.file_path)?;
    let file = Arc::new(Mutex::new(file));
    let delay = Arc::new(opt.seconds);

    shared_memory_writer(file, delay, opt.thread_count)?;

    Ok(())
}

/// This function spawns multiple threads, as per the `thread_count`,
/// which write a simple message to the shared file given by `file`.
/// After writing the message, thread sleeps for some seconds given by `delay`.
fn shared_memory_writer(
    file: Arc<Mutex<File>>,
    delay: Arc<Duration>,
    thread_count: usize,
) -> Result<()> {
    // Here we iterate over thread count times and
    // for each iteration a `map()` produces a JoinHandle
    // which is collected in a Vec<>
    let threads = (1usize..=thread_count)
        .map(|index| {
            let file = Arc::clone(&file);
            let delay = Arc::clone(&delay);

            thread::Builder::new()
                .name(format!("thread-{}", index))
                .spawn(move || {
                    let mut file = file.lock().unwrap();

                    // can be unwrapped safely
                    let current_thread = thread::current();
                    let thread_name = current_thread.name().unwrap();

                    let thread_msg = format!("Hello from {}\n", thread_name);

                    file.write_all(thread_msg.as_bytes())
                        .expect(format!("{} failed to write to file", thread_name).as_str());

                    thread::sleep(*delay);
                })
        })
        .collect::<Vec<_>>();

    // Wait for all threads to complete
    for handle in threads {
        handle?.join().unwrap()
    }

    Ok(())
}
