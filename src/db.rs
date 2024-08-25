use rusqlite::{Result, Error};
use crate::structs::Person;

pub async fn create_blame_table(conn: &mut tokio::sync::Mutex<rusqlite::Connection>) -> Result<()> {
    conn.get_mut().execute("
    CREATE TABLE IF NOT EXISTS blame (
        name TEXT NOT NULL,
        rotation_id INTEGER NOT NULL
    )",
    ()
    )?;
    Ok(())
}

pub async fn insert_blame(conn: &mut tokio::sync::Mutex<rusqlite::Connection>, person: Person) -> Result<(), Error> {
    conn.get_mut().execute("INSERT INTO blame (rotation_id, name) VALUES (?1, ?2)",
        (&person.rotation_id, &person.name),
    )?;
    Ok(())
}

pub async fn query_blame_table(conn: &mut tokio::sync::Mutex<rusqlite::Connection>) -> rusqlite::Result<(Vec<String>, Vec<i32>)> {
    let mut names: Vec<String> = Vec::new();
    let mut ids: Vec<i32> = Vec::new();

    let mut stmt = conn.get_mut().prepare("SELECT name, rotation_id FROM blame")?;
    let mut rows: rusqlite::Rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        names.push(row.get(0)?);
        ids.push(row.get(1)?);
    };

    Ok((names, ids))
//    let rows_iter = stmt.query_map([], |row| {
//        Ok(Person {
//            name: row.get(0)?,
//            rotation_id: row.get(1)?,
//        })
//    })?;
//
//
//    for row in rows_iter {
//        names.push(row.as_ref().unwrap().name.clone());
//        ids.push(row.as_ref().unwrap().rotation_id);
//    }
//    Ok((names, ids))

}

pub async fn get_blame_list(conn: &mut tokio::sync::Mutex<rusqlite::Connection>) -> rusqlite::Result<Vec<String>> {
    let mut stmt: rusqlite::Statement = conn.get_mut().prepare("SELECT name FROM blame")?;
    let mut rows: rusqlite::Rows = stmt.query([])?;

    let mut names = Vec::new();
    
    while let Some(row) = rows.next()? {
        names.push(row.get(0)?);
    }

    Ok(names)
}
