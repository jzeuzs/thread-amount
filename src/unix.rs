use std::num::NonZeroUsize;
use std::process::{self, Command, Stdio};

pub(crate) fn thread_amount() -> Option<NonZeroUsize> {
    let pid = process::id();
    let command = Command::new("ps")
        .args(["-L", "-o", "pid=", "-p", &format!("{}", pid), "|", "wc", "-l"])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed getting amount");

    NonZeroUsize::new(
        String::from_utf8_lossy(&command.stdout)
            .parse::<usize>()
            .expect("Failed converting into usize"),
    )
}
