use riven::{consts::Region, RiotApi};

pub fn create_riot_api() -> RiotApi {
    let api_key = std::env!("RGAPI_KEY");
    RiotApi::with_key(api_key)
}

pub async fn get_masteries(riot_api: &RiotApi, username: String) -> usize {
    // Get summoner data.
    let summoner = riot_api
        .summoner_v4()
        .get_by_summoner_name(Region::EUW, username.as_str())
        .await
        .expect("Get summoner failed.")
        .expect("There is no summoner with that name.");

    // Print summoner name.
    println!("{} Champion Masteries:", summoner.name);

    // Get champion mastery data.
    let masteries = riot_api
        .champion_mastery_v4()
        .get_all_champion_masteries(Region::EUW, &summoner.id)
        .await
        .expect("Get champion masteries failed.");

    // Print champioon masteries.
    for (i, mastery) in masteries.iter().enumerate() {
        println!(
            "{: >2}) {: <9}    {: >7} ({})",
            i + 1,
            mastery.champion_id.to_string(),
            mastery.champion_points,
            mastery.champion_level
        );
    }
    masteries.len()
}
