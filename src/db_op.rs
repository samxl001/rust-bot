use rusqlite::{params, Connection, Result};

pub fn populate_table(post_vec: Vec<&str>) -> Result<()> {
    let conn = Connection::open("data.db3")?;

    println!("post_vec1: {}", post_vec.len());
    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts
            (id INTEGER PRIMARY KEY,
            content TEXT NOT NULL)",
        [],
    )?;

    println!("post_vec: {}", post_vec.len());
    for value in &post_vec {
        conn.execute(
            "INSERT INTO posts (content)
            SELECT ?1
            WHERE NOT EXISTS
            (SELECT 1 FROM posts WHERE content = ?1);",
            params![value],
        )?;
    }
    Ok(())
}
pub fn query_table() -> Result<()> {
    let conn = Connection::open("data.db3")?;
    let mut stmt = conn.prepare("SELECT id, content FROM posts")?;
    // Map each row to a tuple (id, content)
    let post_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    // Iterate through the rows and print the data
    for post in post_iter {
        let (id, content) = post?;
        println!("ID: {}, Content: {}", id, content);
    }
    Ok(())
}
