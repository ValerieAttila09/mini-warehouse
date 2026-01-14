DB_URL=${1:-"postgres://postgres:root@localhost:5432/mini_warehouse"}

echo "Menjalankan seeders untuk database"
echo "=================================="

run_sql_file() {
  local file=$1
  local description=$2

  echo "Menjalankan: $description"
  if psql "$DB_URL" -f "$file" > /dev/null 2>&1; then
    echo "Berhasil: $description"
  else
    echo "Gagal: $description:"
    exit 1
  fi
  echo ""
}

echo "Menjalankan Seeders..."
run_sql_file "seeders/insert_default_roles.sql" "Insert default roles"
run_sql_file "seeders/insert_sample_users.sql" "Insert default roles"

echo "Semua seeders berhasil dijalankan!"

echo ""
echo "Verifikasi Data:"
psql "$DB_URL" -c "
  SELECT
    u.id,
    u.name,
    u.email,
    r.name as role_name
  FROM public.users u
  LEFT JOIN public.user_role ur ON u.id = ur.user_id
  LEFT JOIN public.roles ur ON u.role_id = r.id
  ORDER BY u.id;
"