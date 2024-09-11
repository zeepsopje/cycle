use anyhow::Result;
use sqlx::{MySql, Pool};
use futures::TryStreamExt;

pub struct Project {
    pub id: i32,
    pub name: String,
}

pub struct Timer {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
}

pub struct App {
    pool: Pool<MySql>,
}

impl App {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self {
            pool
        }
    }

    pub async fn create_project(&self, name: &str) -> Result<u64> {
        let result = sqlx::query!("INSERT INTO projects (name) VALUES (?)", name)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_id())
    }

    pub async fn get_project(&self, id: i32) -> Result<Project> {
        let result = sqlx::query_as!(Project, "SELECT * FROM projects WHERE id=?", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>> {
        let mut projects = vec![];
        let mut stream = sqlx::query_as!(Project, "SELECT * FROM projects")
            .fetch(&self.pool);

        while let Some(project) = stream.try_next().await? {
            projects.push(project);
        }

        Ok(projects)
    }

    pub async fn delete_project(&self, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM projects WHERE id=?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_timer(&self, project_id: i32, name: &str) -> Result<u64> {
        let result = sqlx::query!("INSERT INTO timers (project_id, name) VALUES (?, ?)", project_id, name)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_id())
    }

    pub async fn get_timer(&self, id: i32) -> Result<Timer> {
        let result = sqlx::query_as!(Timer, "SELECT * FROM timers WHERE id=?", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn get_timers(&self, project_id: Option<i32>) -> Result<Vec<Timer>> {
        let mut timers = vec![];
        let mut stream;

        if let Some(project_id) = project_id {
            stream = sqlx::query_as!(Timer, "SELECT * FROM timers WHERE project_id=?", project_id)
                .fetch(&self.pool);
        } else {
            stream = sqlx::query_as!(Timer, "SELECT * FROM timers")
                .fetch(&self.pool);
        }

        while let Some(timer) = stream.try_next().await? {
            timers.push(timer);
        }

        Ok(timers)
    }

    pub async fn delete_timer(&self, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM timers WHERE id=?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
