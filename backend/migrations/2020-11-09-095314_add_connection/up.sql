CREATE TABLE connections (
    user_id INT REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
    summoner_id INT REFERENCES summoners (id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY(user_id, summoner_id)
)