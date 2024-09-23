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
  ),
  (
    'f397b83a-dd2a-4a01-9e77-db1eea7de5b6',
    'ゼロから学ぶRust　システムプログラミングの基礎から線形型システムまで',
    '高野祐輝',
    '978-4065301951',
    '通読して学習する入門書！　単なる文法解説にはとどまらない。実践的なソフトウェア実装と、Rustの安全性を支える理論の学習を通して、ゼロから徹底的にマスターできる！',
    '5b4c96ac-316a-4bee-8e69-cac5eb84ff4c',
    now(),
    now()
  ),
  (
    '17afb850-c786-49c5-a303-a3a443a2212c',
    'RustによるWebアプリケーション開発　設計からリリース・運用まで',
    '豊田優貴他',
    '978-4065369579',
    '「蔵書管理アプリケーション」の実装を通じて、設計、開発、保守、運用までハンズオンで学ぶ！　今こそ現場にRustを！',
    '5b4c96ac-316a-4bee-8e69-cac5eb84ff4c',
    now(),
    now()
  ) ON CONFLICT DO NOTHING;

