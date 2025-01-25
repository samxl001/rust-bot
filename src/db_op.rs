use rusqlite::{params, Connection, Result, Error};

pub fn populate_table(filename:&str, post_vec: Vec<&str>) -> Result<()> {
    let conn = match Connection::open(filename) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to open connection to {}", e);
            return Err(e);
        }
    };

    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts
            (id INTEGER PRIMARY KEY,
            content TEXT NOT NULL)",
        [],
    )?;

    for value in &post_vec {
        conn.execute(
            "INSERT INTO posts (content)
            SELECT ?1
            WHERE NOT EXISTS
            (SELECT 1 FROM posts WHERE content = ?1)",
            params![value],
        )?;
    }
    Ok(())
}
pub fn query_rows(filename: &str) -> Result<()> {
    let conn = match Connection::open(filename) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to open connection to {}", e);
            return Err(e);
        }
    };
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
pub fn delete_table(filename:&str, table_name: &str) -> Result<()> {
    // Validate table name to prevent SQL injection
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(Error::InvalidQuery);
    }
    let query = format!("DROP TABLE IF EXISTS {}", table_name);
    let conn_result = Connection::open(filename);
    match conn_result {
        Ok(conn) => match conn.execute(&query, []) {
            Ok(_) => {
                println!("Table '{}' dropped successfully.", table_name);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to drop table '{}': {}", table_name, e);
                Err(e)
            }
        },
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            Err(e)
        }
    }
}