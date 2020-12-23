CREATE TABLE notifications (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
    title TEXT NOT NULL,
    message TEXT NOT NULL
);
