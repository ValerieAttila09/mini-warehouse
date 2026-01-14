-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.roles (
  id bigserial PIRMARY KEY,
  name varchar(100) NOT NULL,
  created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE public.roles
ADD CONSTRAINT uni_roles_name UNIQUE (name);

CREATE INDEX IF NOT EXISTS idx_roles_name ON public.roles (name);

CREATE INDEX IF NOT EXISTS idx_roles_created_at ON public.roles (created_at);