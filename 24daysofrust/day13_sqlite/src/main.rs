use rusqlite::{Connection, Result};

#[allow(dead_code)]
fn todo() {
    // TODO:
    // https://docs.rs/rusqlite/latest/rusqlite/
    unimplemented!();
}

#[derive(Debug)]
struct Teacher {
    id: u32,
    name: String,
    department: String,
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

    let _teachers = vec![
        [String::from("ptk"), String::from("CS")],
        [String::from("vsg"), String::from("ENTC")],
        [String::from("yip"), String::from("IT")],
    ];
    // for [name, dept] in teachers {
    //     conn.execute(
    //         "INSERT INTO teachers (name, department) values (?1, ?2)",
    //         &[&name, &dept],
    //     )?;
    // }
    let mut stmt = conn.prepare("SELECT * from teachers")?;

    let outputs = stmt.query_map([], |row| {
        Ok(Teacher {
            id: row.get(0)?,
            name: row.get(1)?,
            department: row.get(2)?,
        })
    })?;

    for output in outputs {
        dbg!(output);
    }
    Ok(())
}
