CREATE TABLE summoners (
    id SERIAL NOT NULL PRIMARY KEY,
    account_id TEXT NOT NULL,
    profile_icon_id INT NOT NULL,
    revision_date BIGINT NOT NULL,
    name TEXT NOT NULL,
    summoner_id TEXT NOT NULL,
    puuid TEXT NOT NULL,
    summoner_level BIGINT NOT NULL
);