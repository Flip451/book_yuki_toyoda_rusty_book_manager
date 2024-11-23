INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Flip451', 'flipflap451@gmail.com', '$2b$12$hYF2CCJeGdxhrAv7yAlnyuqNG8kJM7FxfQOrUbxEbG.RIhYusziC2', role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
