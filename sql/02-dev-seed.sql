-- Dev seed
INSERT INTO chef (firebase_id, username) VALUES ('firebase_auth_123', 'Goombah!');


INSERT INTO recipe (id, cid, title, steps, tags) VALUES (1, 'firebase_auth_123', 'Hunter`s Stew', '{"Tenderize the Boy", "Put him in the stew"}', '{"Weeknight-Dinner", "Easy"}');
INSERT INTO recipe (id, cid, title, steps, tags) VALUES (2, 'firebase_auth_123', 'Lemon Pound Cake', '{"He is a pound of cake!"}', '{"Weeknight-Dinner", "Easy"}');
INSERT INTO recipe (id, cid, title, steps, tags) VALUES (3, 'firebase_auth_123', 'Roast Beast', '{"Make sure to slice him thin!"}', '{"Weeknight-Dinner", "Hard"}');
