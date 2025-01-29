use rusqlite::{params, params_from_iter, Connection, Error, Result};

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
pub fn update_mentions_table(database_filename: &str, fieldname: String) -> Result<()> {
    // Open the database connection
    let conn = match Connection::open(database_filename) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return Err(e);
        }
    };

    // Get today's date
    let today = chrono::Local::now().date_naive(); // Format: YYYY-MM-DD

    // Create the `mentions` table if it does not exist
    let create_table_query = "
        CREATE TABLE IF NOT EXISTS mentions (
            date TEXT PRIMARY KEY
        )
    ";
    conn.execute(create_table_query, [])?;

    // Check if the fieldname already exists in the table
    let pragma_query = "PRAGMA table_info(mentions);";
    let mut stmt = conn.prepare(pragma_query)?;
    let mut column_exists = false;
    let column_iter = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(1)?) // Get column name (second column in PRAGMA)
    })?;

    for column in column_iter {
        if column? == fieldname {
            column_exists = true;
            break;
        }
    }

    // If the fieldname does not exist, add it as an INTEGER column
    if !column_exists {
        let alter_table_query = format!(
            "ALTER TABLE mentions ADD COLUMN {} INTEGER DEFAULT 0",
            fieldname
        );
        conn.execute(&alter_table_query, [])?;
        println!("Added new column: {}", fieldname);
    } else {
        println!("Column '{}' already exists.", fieldname);
    }

    // Insert today's date into the table if it does not already exist
    let insert_query = "
        INSERT INTO mentions (date)
        SELECT ?
        WHERE NOT EXISTS (SELECT 1 FROM mentions WHERE date = ?)
    ";
    conn.execute(insert_query, [today.to_string(), today.to_string()])?;

    println!("Database updated successfully.");
    Ok(())
}
pub fn update_field_values(
    database_filename: &str,
    vec_fieldname: Vec<String>,
    vec_values: Vec<i64>,
) -> Result<()> {
    if vec_fieldname.len() != vec_values.len() {
        return Err(rusqlite::Error::InvalidQuery); // Ensure the field names and values match in length
    }

    // Open the database connection
    let conn = Connection::open(database_filename)?;

    // Get today's date
    let today = chrono::Local::now().date_naive(); // Format: YYYY-MM-DD

    // Dynamically construct the UPDATE query
    let update_query = format!(
        "UPDATE mentions SET {} WHERE date = ?",
        vec_fieldname
            .iter()
            .map(|field| format!("{} = ?", field))
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Combine values and today's date into a single vector
    let mut query_values: Vec<Box<dyn rusqlite::ToSql>> = vec![];
    query_values.extend(
        vec_values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn rusqlite::ToSql>),
    );
    query_values.push(Box::new(today.format("%Y-%m-%d").to_string()));

    // Use params_from_iter to bind the query values
    let rows_affected = conn.execute(&update_query, params_from_iter(query_values.iter()))?;

    if rows_affected == 0 {
        println!("No rows were updated. Ensure the date exists in the table.");
    } else {
        println!("Successfully updated {} row(s).", rows_affected);
    }

    Ok(())
}
