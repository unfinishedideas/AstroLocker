-- Make an admin
INSERT INTO users (email, password, is_banned) VALUES('admin@astrolocker.com', '12345', FALSE);
INSERT INTO admins (admin_user_id) VALUES(1);

-- Make a banned user
INSERT INTO users (email, password, is_banned) VALUES('troll@astrolocker.com', '12345', TRUE);

-- Make a normal user
INSERT INTO users (email, password, is_banned) VALUES('joe@astrolocker.com', '12345', FALSE);

-- Make a test post
INSERT INTO posts (title, explanation, query_string, img_url, apod_date) VALUES('A Test Post', 'This is a test post to test the db', 'An image of a test post', 'www.ultrafakeurl42.com/testpost', '1996');

-- Add some votes to it
INSERT INTO votes (user_id, post_id) VALUES(1,1);
INSERT INTO votes (user_id, post_id) VALUES(2,1);