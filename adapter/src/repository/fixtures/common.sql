INSERT INTO roles(name) VALUES ('Admin'), ('User');

INSERT INTO
    users (user_id, name, email, password_hash, role_id)
SELECT
    '5b4c96ac-316a-4bee-8e69-cac5eb84ff4c', 'Flip451', 'flipflap451@gmail.com', '$2b$12$hYF2CCJeGdxhrAv7yAlnyuqNG8kJM7FxfQOrUbxEbG.RIhYusziC2', role_id
FROM
    roles
WHERE
    name LIKE 'Admin';