use super::Workspace;
use crate::{AppError, ChatUser};
use sqlx::PgPool;

impl Workspace {
    pub async fn create(name: &str, user_id: u64, pool: &PgPool) -> Result<Self, AppError> {
        let user = sqlx::query_as(
            r#"
            INSERT INTO workspaces (name, owner_id)
            VALUES ($1, $2)
            RETURNING id, name, owner_id, created_at
            "#,
        )
        .bind(name)
        .bind(user_id as i64)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn update_owner(&self, owner_id: u64, pool: &PgPool) -> Result<Self, AppError> {
        // Update owner_id in two cases: when owner_id is 0 and when it owner's ws_id matches the workspace id
        let workspace = sqlx::query_as(
            r#"
            UPDATE workspaces
            SET owner_id = $1
            WHERE id = $2 and (SELECT ws_id FROM users WHERE id = $1) = $2
            RETURNING id, name, owner_id, created_at
            "#,
        )
        .bind(owner_id as i64)
        .bind(self.id)
        .fetch_one(pool)
        .await?;
        Ok(workspace)
    }

    pub async fn find_by_name(name: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let workspace = sqlx::query_as(
            r#"
            SELECT id, name, owner_id, created_at
            FROM workspaces
            WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        Ok(workspace)
    }

    pub async fn find_by_id(id: u64, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let workspace = sqlx::query_as(
            r#"
            SELECT id, name, owner_id, created_at
            FROM workspaces
            WHERE id = $1
            "#,
        )
        .bind(id as i64)
        .fetch_optional(pool)
        .await?;
        Ok(workspace)
    }

    pub async fn fetch_all_chat_users(id: u64, pool: &PgPool) -> Result<Vec<ChatUser>, AppError> {
        let users = sqlx::query_as(
            r#"
            SELECT id, fullname, email
            FROM users
            WHERE ws_id = $1
            ORDER BY id
            "#,
        )
        .bind(id as i64)
        .fetch_all(pool)
        .await?;
        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{CreateUser, User};

    use super::*;
    use anyhow::Result;
    use sqlx_db_tester::TestPg;

    #[tokio::test]
    async fn workspace_should_create_and_set_owner() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://chat:Pyh20010226@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let ws = Workspace::create("test", 0, &pool).await?;

        let input = CreateUser::new(&ws.name, "YP", "yp@example.com", "password");
        let user = User::create(&input, &pool).await?;
        assert_eq!(ws.name, "test");

        assert_eq!(user.ws_id, ws.id);

        let ws = ws.update_owner(user.id as _, &pool).await?;

        assert_eq!(ws.owner_id, user.id);
        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_find_by_name() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://chat:Pyh20010226@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let ws = Workspace::create("test_workspace", 0, &pool).await?;
        let found_ws = Workspace::find_by_name("test_workspace", &pool).await?;
        assert!(found_ws.is_some());
        assert_eq!(found_ws.unwrap().id, ws.id);
        let not_found_ws = Workspace::find_by_name("non_existent_workspace", &pool).await?;
        assert!(not_found_ws.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_fetch_all_chat_users() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://chat:Pyh20010226@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let ws = Workspace::create("test_workspace", 0, &pool).await?;
        let input = CreateUser::new(&ws.name, "Alice", "alice@example.com", "password");
        let user1 = User::create(&input, &pool).await?;
        let input = CreateUser::new(&ws.name, "Bob", "bob@example.com", "password");
        let user2 = User::create(&input, &pool).await?;
        let users = Workspace::fetch_all_chat_users(ws.id as _, &pool).await?;
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].id, user1.id);
        assert_eq!(users[1].id, user2.id);
        Ok(())
    }
}
