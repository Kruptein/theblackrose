alter table match_references drop constraint match_references_pkey;
alter table match_references
add constraint match_references_pkey primary key (game_id, summoner_id);