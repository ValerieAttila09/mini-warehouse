-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.user_role (
  id bigserial PRIMARY KEY,
  user_id bigint NOT NULL,
  role_id bigint NOT NULL,
  created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE public.user_role
ADD CONSTRAINT fk_user_role_user
FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE public.user_role
ADD CONSTRAINT fk_user_role_role
FOREIGN KEY (role_id) REFERENCES public.roles(id) ON DELETE CASCADE;

ALTER TABLE public.user_role
ADD CONSTRAINT uni_user_role_user_id UNIQUE (user_id);

CREATE INDEX IF NOT EXISTS idx_user_role_user_id ON public.user_role (user_id);
CREATE INDEX IF NOT EXISTS idx_user_role_role_id ON public.user_role (role_id);
CREATE INDEX IF NOT EXISTS idx_user_role_created_at ON public.user_role (created_at);