//! Lists contents of a directory.
//!
//! Run with:
//!
//! ```
//! cargo run --example ls .
//! ```

use std::{env, fs, io};

use superpoll_blocking::Unblock;
use futures::{executor, prelude::*};

fn main() -> io::Result<()> {
    let path = env::args().nth(1).unwrap_or(".".into());

    executor::block_on(async {
        let mut dir = Unblock::new(fs::read_dir(path)?);

        while let Some(item) = dir.next().await {
            println!("{}", item?.file_name().to_string_lossy());
        }

        Ok(())
    })
}
