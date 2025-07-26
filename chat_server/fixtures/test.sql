-- insert workspaces
INSERT INTO workspaces(name, owner_id)
VALUES ('acme', 0), ('fool', 0), ('bar', 0);

-- insert users
INSERT INTO users(ws_id, email, fullname, password_hash)
VALUES (1, 'tchen@acme.org', 'Tyr Chen', '$argon2id$v=19$m=19456,t=2,p=1$S6kH8zNHmo0ttacLjYR3aA$dlFZh0GMBoGm7sQIDWkxAs4TpP9OTtPhzAHgP9CbXWE'),
(1, 'jdoe@acme.org', 'John Doe', '$argon2id$v=19$m=19456,t=2,p=1$S6kH8zNHmo0ttacLjYR3aA$dlFZh0GMBoGm7sQIDWkxAs4TpP9OTtPhzAHgP9CbXWE'),
(1, 'asmith@fool.com', 'Alice Smith', '$argon2id$v=19$m=19456,t=2,p=1$S6kH8zNHmo0ttacLjYR3aA$dlFZh0GMBoGm7sQIDWkxAs4TpP9OTtPhzAHgP9CbXWE'),
(1, 'bjackson@bar.com', 'Bob Jackson', '$argon2id$v=19$m=19456,t=2,p=1$S6kH8zNHmo0ttacLjYR3aA$dlFZh0GMBoGm7sQIDWkxAs4TpP9OTtPhzAHgP9CbXWE'),
(1, 'dfs@fool.com', 'Dfs', '$argon2id$v=19$m=19456,t=2,p=1$S6kH8zNHmo0ttacLjYR3aA$dlFZh0GMBoGm7sQIDWkxAs4TpP9OTtPhzAHgP9CbXWE');

-- insert 4 chats
-- insert public/private channel
INSERT INTO chats (ws_id, name, type, members)
            VALUES (1, 'general', 'public_channel', '{1, 2, 3, 4, 5}'),
            (1, 'private', 'private_channel', '{1, 2, 3}');

-- insert unamed chat
INSERT INTO chats(ws_id, type, members)
VALUES (1, 'single', '{1, 2}'),
(1, 'group', '{1, 3, 4}');
