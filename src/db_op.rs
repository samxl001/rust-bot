use rusqlite::{params, Connection, Error, Result};

pub fn populate_table(filename: &str, table_name: &str, post_vec: Vec<&str>) -> Result<()> {
    let conn = match Connection::open(filename) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to connect to database {}", e);
            return Err(e);
        }
    };

    // Dynamically construct the CREATE TABLE SQL query
    let create_table_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL
        )",
        table_name
    );

    conn.execute(&create_table_query, [])?;

    // Dynamically construct the INSERT query
    let insert_query = format!(
        "INSERT INTO {} (content)
        SELECT ?1
        WHERE NOT EXISTS
        (SELECT 1 FROM {} WHERE content = ?1)",
        table_name, table_name
    );

    for value in &post_vec {
        conn.execute(&insert_query, params![value])?;
    }

    Ok(())
}
pub fn query_values(filename: &str, table_name: &str) -> Result<Vec<String>> {
    let conn = match Connection::open(filename) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to connect to database {}", e);
            return Err(e);
        }
    };

    let query = format!("SELECT content FROM {}", table_name);
    let mut stmt = conn.prepare("SELECT content FROM posts")?;
    // Map each row to extract only the `content` field
    let post_iter = stmt.query_map([], |row| {
        row.get::<_, String>(0) // Get the first column (`content`)
    })?;

    // Collect the content into a vector
    let mut contents = Vec::new();
    for post in post_iter {
        match post {
            Ok(c) => contents.push(c),
            Err(e) => return Err(e),
        }
    }

    Ok(contents)
}
pub fn delete_table(filename: &str, table_name: &str) -> Result<()> {
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
