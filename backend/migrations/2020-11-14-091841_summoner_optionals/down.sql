UPDATE summoners
SET revision_date = 0
WHERE revision_date is NULL;
UPDATE summoners
SET summoner_level = 0
WHERE summoner_level is NULL;
UPDATE summoners
SET puuid = ''
WHERE puuid is NULL;
UPDATE summoners
SET summoner_id = ''
WHERE summoner_id is NULL;
ALTER TABLE summoners
ALTER COLUMN revision_date
SET NOT NULL;
ALTER TABLE summoners
ALTER COLUMN summoner_level
SET NOT NULL;
ALTER TABLE summoners
ALTER COLUMN puuid
SET NOT NULL;
ALTER TABLE summoners
ALTER COLUMN summoner_id
SET NOT NULL;