ALTER TABLE summoners
ALTER COLUMN revision_date DROP NOT NULL;
ALTER TABLE summoners
ALTER COLUMN summoner_level DROP NOT NULL;
ALTER TABLE summoners
ALTER COLUMN puuid DROP NOT NULL;
ALTER TABLE summoners
ALTER COLUMN summoner_id DROP NOT NULL;
UPDATE summoners
SET revision_date = NULL
WHERE revision_date = 0;
UPDATE summoners
SET summoner_level = NULL
WHERE summoner_level = 0;
UPDATE summoners
SET puuid = NULL
WHERE puuid = '';
UPDATE summoners
SET summoner_id = NULL
WHERE summoner_id = '';