use crate::models::{users::User};
use crate::configs::db::DBPool;

pub async fn find_user_with_role_by_email(pool: &DBPool, email: &str) -> sqlx::Result<Option<User, Option<String>>> {
  let user = sqlx::query_as!(
    User,
    r#"
    SELECT id, name, email, password, photo, phone
    FROM public.users
    WHERE email = $1
    "#,
    email
  ).fetch_optional(pool).await?;
}