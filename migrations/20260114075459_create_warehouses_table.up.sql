-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.warehouses (
  id bigserial PRIMARY KEY,
  name varchar(100) NOT NULL,
  address text NOT NULL,
  photo text NOT NULL,
  created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
  deleted_at timestamp with time zone
);