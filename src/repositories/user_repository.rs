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

  if let Some(user) = user {
    let role: Option<{String}> = sqlx::query_as(
      r#"
      SELECT r.name FROM public.user_role ur
      JOIN public.roles r ON ur.role_id = r.id
      WHERE ur.user_id = $1
      "#,
    ).bind(user.id).fetch_optional(pool).await?;

    Ok(Some((user, role.map(|t| t.0))));
  } else {
    Ok(None);
  }
}