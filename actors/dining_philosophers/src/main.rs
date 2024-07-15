mod phil;
mod table;

use actix::prelude::*;
use std::io;
use table::Table;

fn main() -> io::Result<()> {
    let system = System::new();

    system.block_on(async {
        Table::new(5).start();
    });

    system.run()
}
