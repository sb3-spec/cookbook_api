use std::{fs, path::PathBuf, env};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use sea_orm::{DatabaseConnection, Database};
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";


pub async fn init_db() -> Result<DatabaseConnection, super::Error> {
    if true {
        dotenv().ok().expect("Error reading .env file");
    }

    

    // RESET DB dev only
    if false {
        let root_db_url = match env::var("ROOT_DATABASE_URL") {
            Ok(url) => url,
            Err(e) => "Error".to_owned(),
        };

        // let root_db_url = dotenv!("ROOT_DATABASE_URL");
        let db = PgPoolOptions::new()
            .connect(&root_db_url)
            .await?;
        pexec(&db, SQL_RECREATE).await?;
        db.close();
    }
    
    let database_url = match env::var("PRODUCTION_DB_URL") {
        Ok(url) => url,
        Err(e) => "Error".to_owned(),
    };

    let sqlx_db = PgPoolOptions::new()
        .connect(&database_url)
        .await?;

    // -- Run the app sql files
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    // Execute each file
    for path in paths {
        if let Some(path) = path.to_str() {
            // only .sql and not the recreate
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pexec(&sqlx_db, &path).await?;
            }
        }
    }

    let db: DatabaseConnection = Database::connect(database_url).await?;

    Ok(db)
}

async fn pexec(db: &Pool<Postgres>, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|ex| {
        println!("Error reading {} (cause: {:?}", file, ex);
        ex
    })?;

    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        match sqlx::query(&sql).execute(db).await {
            Ok(_) => (),
            Err(ex) => println!("WARNING - pexec - Sql file '{}' FAILED cause: {}", file, ex)
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;