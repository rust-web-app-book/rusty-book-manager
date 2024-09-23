INSERT INTO users(user_id, name, email, password_hash, role_id)
SELECT
    '9582f9de-0fd1-4892-b20c-70139a7eb95b',
    'Sebastian Sallow',
    'sebastian.sallow@example.com',
    '$2b$12$5OYqkGyR9fAwLZJjERI.ROrbdQEd5VCaZ6yy2.TyDkGhWmzdnpv.2',
    role_id
FROM roles WHERE name = 'User';

INSERT INTO users(user_id, name, email, password_hash, role_id)
SELECT
    '050afe56-c3da-4448-8e4d-6f44007d2ca5',
    'Poppy Sweeting',
    'poppy.sweeting@example.com',
    '$2b$12$5OYqkGyR9fAwLZJjERI.ROrbdQEd5VCaZ6yy2.TyDkGhWmzdnpv.2',
    role_id
FROM roles WHERE name = 'User';

INSERT INTO
  books (
    book_id,
    title,
    author,
    isbn,
    description,
    user_id,
    created_at,
    updated_at
  )
VALUES
  (
    '9890736e-a4e4-461a-a77d-eac3517ef11b',
    '実践Rustプログラミング入門',
    '初田直也他',
    '978-4798061702',
    'C/C++の代わりとなるべき最新言語その独特な仕様をわかりやすく解説。',
    '5b4c96ac-316a-4bee-8e69-cac5eb84ff4c',
    now(),
    now()
  ) ON CONFLICT DO NOTHING;

