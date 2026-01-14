-- Add migration script here
CREATE TABLE IF NOT EXISTS public.users (
  id bigserial PRIMARY KEY,
  name varchar(100) NOT NULL,
  email varchar(100) NOT NULL,
  password text NOT NULL,
  photo text,
  phone text,
  created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE public.users
ADD CONSTRAINT uni_users_email UNIQUE (email);

CREATE INDEX IF NOT EXISTS idx_users_email ON public.users (email);

CREATE INDEX IF NOT EXISTS idx_users_created_at ON public.users (created_at);