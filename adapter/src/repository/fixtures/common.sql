INSERT INTO roles(name)
VALUES ('Admin'), ('User');

INSERT INTO users(user_id, name, email, password_hash, role_id)
SELECT
    '5b4c96ac-316a-4bee-8e69-cac5eb84ff4c'
    , 'Eleazar Fig'
    , 'eleazar.fig@example.com'
    , 'atodehenkou'
    , role_id
FROM roles WHERE name = 'Admin';
