use libc::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;

lazy_static! {
    static ref SIGNALS: Signals = Signals::new(&[SIGTERM, SIGINT]).unwrap();
}

pub fn exit_if_needed() {
    for signal in SIGNALS.pending() {
        match signal {
            SIGINT => {
                println!("received SIGINT, exiting");
                ::std::process::exit(1);
            }
            SIGTERM => {
                println!("received SIGTERM, exiting");
                ::std::process::exit(0);
            }
            sig => panic!("we didn't register for it but we received {}", sig),
        }
    }
}
