use rusqlite::{Connection, Result};

#[allow(dead_code)]
fn todo() {
    // TODO:
    // https://docs.rs/rusqlite/latest/rusqlite/
    unimplemented!();
}

fn main() -> Result<()> {
    let conn = Connection::open("pict.db")?;

    conn.execute(
        "
        create table if not exists students (
            id integer primary key,
            name text not null unique
        )
        ",
        [],
    )?;
    conn.execute(
        "
        create table if not exists teachers (
            id integer primary key,
            name text not null unique,
            department integer not null
        )
        ",
        [],
    )?;
    // tables creation successful now try retrieving data and create some
    // useful application
    Ok(())
}
