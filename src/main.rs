use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use cycle::{App, Project};

async fn init_db() -> Result<Pool<MySql>> {
    let url = std::env::var("DATABASE_URL")
        .with_context(|| "DATABASE_URL not set")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await?;

    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let pool = init_db().await?;
    let app = App::new(pool);
    let projects = app.get_projects().await?;

    for project in projects {
        let timers = app.get_timers(Some(project.id)).await?;
        println!("Project {}: {}", project.id, project.name);
        println!("------------------------");
        for timer in timers {
            println!("{}: {}", timer.id, timer.name);
        }
        println!("\n");
    }

    Ok(())
}
