INSERT INTO public.users (name, email, password, phone, created_at, updated_at)
VALUES
  (
    'John Keeper',
    'keeper@warehouse.com',
    '$2b$12$TrNGk/crerFeNK0lQWiY3uqe1PJqMIkuLPEzMd4q9QfjxEursKnlO', -- keeper123
    '+6281234567890',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  )
ON CONFLICT (email) DO NOTHING;


INSERT INTO public.users (name, email, password, phone, created_at, updated_at)
VALUES
  (
    'Jane Manager',
    'manager@warehouse.com',
    '$2b$12$jhn4tVzrMiCitmF.DvRwg.5pbXphh0TE2XUC39i/BsxSwDv77/4n.', -- manager123
    '+6281234567890',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  )
ON CONFLICT (email) DO NOTHING;


INSERT INTO public.user_role (user_id, role_id, created_at, updated_at)
SELECT 
  u.id,
  r.id,
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
FROM public.users u, public.roles r
WHERE u.email = 'keeper@warehouse.com'
  AND r.name = 'keeper'
ON CONFLICT (user_id) DO NOTHING; 


INSERT INTO public.user_role (user_id, role_id, created_at, updated_at)
SELECT 
  u.id,
  r.id,
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
FROM public.users u, public.roles r
WHERE u.email = 'manager@warehouse.com'
  AND r.name = 'manager'
ON CONFLICT (user_id) DO NOTHING; 


SELECT 
  u.id,
  u.name,
  u.email,
  u.phone,
  r.name as role_name,
  ur.created_at as role_assigned_at
FROM public.users u
LEFT JOIN public.user_role ur ON u.id = ur.user_id
LEFT JOIN public.roles r ON ur.role_id = r.id
WHERE u.email IN ('keeper@warehouse.com', 'manager@warehouse.com')
ORDER BY u.id;