enum Queue {
    // Games on Custom games
    Custom = 0,
    // 5v5 Blind Pick games on Summoner's Rift
    // Deprecated in patch 7.19 in favor of queueId 430
    SUMMONERS_RIFT_5V5_BLIND_PICK_DEPRECATED_2 = 2,
    // 5v5 Ranked Solo games on Summoner's Rift
    // Deprecated in favor of queueId 420
    SUMMONERS_RIFT_5V5_RANKED_SOLO_DEPRECATED_4 = 4,
    // 5v5 Ranked Premade games on Summoner's Rift
    // Game mode deprecated
    SUMMONERS_RIFT_5V5_RANKED_PREMADE = 6,
    // Co-op vs AI games on Summoner's Rift
    // Deprecated in favor of queueId 32 and 33
    SUMMONERS_RIFT_CO_OP_VS_AI = 7,
    // 3v3 Normal games on Twisted Treeline
    // Deprecated in patch 7.19 in favor of queueId 460
    TWISTED_TREELINE_3V3_NORMAL = 8,
    // 3v3 Ranked Flex games on Twisted Treeline
    // Deprecated in patch 7.19 in favor of queueId 470
    TWISTED_TREELINE_3V3_RANKED_FLEX_DEPRECATED_9 = 9,
    // 5v5 Draft Pick games on Summoner's Rift
    // Deprecated in favor of queueId 400
    SUMMONERS_RIFT_5V5_DRAFT_PICK_DEPRECATED_14 = 14,
    // 5v5 Dominion Blind Pick games on Crystal Scar
    // Game mode deprecated
    CRYSTAL_SCAR_5V5_DOMINION_BLIND_PICK = 16,
    // 5v5 Dominion Draft Pick games on Crystal Scar
    // Game mode deprecated
    CRYSTAL_SCAR_5V5_DOMINION_DRAFT_PICK = 17,
    // Dominion Co-op vs AI games on Crystal Scar
    // Game mode deprecated
    CRYSTAL_SCAR_DOMINION_CO_OP_VS_AI = 25,
    // Co-op vs AI Intro Bot games on Summoner's Rift
    // Deprecated in patch 7.19 in favor of queueId 830
    SUMMONERS_RIFT_CO_OP_VS_AI_INTRO_BOT_DEPRECATED_31 = 31,
    // Co-op vs AI Beginner Bot games on Summoner's Rift
    // Deprecated in patch 7.19 in favor of queueId 840
    SUMMONERS_RIFT_CO_OP_VS_AI_BEGINNER_BOT_DEPRECATED_32 = 32,
    // Co-op vs AI Intermediate Bot games on Summoner's Rift
    // Deprecated in patch 7.19 in favor of queueId 850
    SUMMONERS_RIFT_CO_OP_VS_AI_INTERMEDIATE_BOT_DEPRECATED_33 = 33,
    // 3v3 Ranked Team games on Twisted Treeline
    // Game mode deprecated
    TWISTED_TREELINE_3V3_RANKED_TEAM = 41,
    // 5v5 Ranked Team games on Summoner's Rift
    // Game mode deprecated
    SUMMONERS_RIFT_5V5_RANKED_TEAM = 42,
    // Co-op vs AI games on Twisted Treeline
    // Deprecated in patch 7.19 in favor of queueId 800
    TWISTED_TREELINE_CO_OP_VS_AI = 52,
    // 5v5 Team Builder games on Summoner's Rift
    // Game mode deprecated
    SUMMONERS_RIFT_5V5_TEAM_BUILDER = 61,
    // 5v5 ARAM games on Howling Abyss
    // Deprecated in patch 7.19 in favor of queueId 450
    HOWLING_ABYSS_5V5_ARAM_DEPRECATED_65 = 65,
    // ARAM Co-op vs AI games on Howling Abyss
    // Game mode deprecated
    HOWLING_ABYSS_ARAM_CO_OP_VS_AI = 67,
    // One for All games on Summoner's Rift
    // Deprecated in patch 8.6 in favor of queueId 1020
    SUMMONERS_RIFT_ONE_FOR_ALL_DEPRECATED_70 = 70,
    // 1v1 Snowdown Showdown games on Howling Abyss
    HOWLING_ABYSS_1V1_SNOWDOWN_SHOWDOWN = 72,
    // 2v2 Snowdown Showdown games on Howling Abyss
    HOWLING_ABYSS_2V2_SNOWDOWN_SHOWDOWN = 73,
    // 6v6 Hexakill games on Summoner's Rift
    SUMMONERS_RIFT_6V6_HEXAKILL = 75,
    // Ultra Rapid Fire games on Summoner's Rift
    SUMMONERS_RIFT_ULTRA_RAPID_FIRE = 76,
    // One For All: Mirror Mode games on Howling Abyss
    HOWLING_ABYSS_ONE_FOR_ALL_MIRROR_MODE = 78,
    // Co-op vs AI Ultra Rapid Fire games on Summoner's Rift
    SUMMONERS_RIFT_CO_OP_VS_AI_ULTRA_RAPID_FIRE = 83,
    // Doom Bots Rank 1 games on Summoner's Rift
    // Deprecated in patch 7.19 in favor of queueId 950
    SUMMONERS_RIFT_DOOM_BOTS_RANK_1 = 91,
    // Doom Bots Rank 2 games on Summoner's Rift
    // Deprecated in patch 7.19 in favor of queueId 950
    SUMMONERS_RIFT_DOOM_BOTS_RANK_2 = 92,
    // Doom Bots Rank 5 games on Summoner's Rift
    // Deprecated in patch 7.19 in favor of queueId 950
    SUMMONERS_RIFT_DOOM_BOTS_RANK_5 = 93,
    // Ascension games on Crystal Scar
    // Deprecated in patch 7.19 in favor of queueId 910
    CRYSTAL_SCAR_ASCENSION_DEPRECATED_96 = 96,
    // 6v6 Hexakill games on Twisted Treeline
    TWISTED_TREELINE_6V6_HEXAKILL = 98,
    // 5v5 ARAM games on Butcher's Bridge
    BUTCHERS_BRIDGE_5V5_ARAM = 100,
    // Legend of the Poro King games on Howling Abyss
    // Deprecated in patch 7.19 in favor of queueId 920
    HOWLING_ABYSS_LEGEND_OF_THE_PORO_KING_DEPRECATED_300 = 300,
    // Nemesis games on Summoner's Rift
    SUMMONERS_RIFT_NEMESIS = 310,
    // Black Market Brawlers games on Summoner's Rift
    SUMMONERS_RIFT_BLACK_MARKET_BRAWLERS = 313,
    // Nexus Siege games on Summoner's Rift
    // Deprecated in patch 7.19 in favor of queueId 940
    SUMMONERS_RIFT_NEXUS_SIEGE_DEPRECATED_315 = 315,
    // Definitely Not Dominion games on Crystal Scar
    CRYSTAL_SCAR_DEFINITELY_NOT_DOMINION = 317,
    // ARURF games on Summoner's Rift
    // Deprecated in patch 7.19 in favor of queueId 900
    SUMMONERS_RIFT_ARURF = 318,
    // All Random games on Summoner's Rift
    SUMMONERS_RIFT_ALL_RANDOM = 325,
    // 5v5 Draft Pick games on Summoner's Rift
    SUMMONERS_RIFT_5V5_DRAFT_PICK = 400,
    // 5v5 Ranked Dynamic games on Summoner's Rift
    // Game mode deprecated in patch 6.22
    SUMMONERS_RIFT_5V5_RANKED_DYNAMIC = 410,
    // 5v5 Ranked Solo games on Summoner's Rift
    SUMMONERS_RIFT_5V5_RANKED_SOLO = 420,
    // 5v5 Blind Pick games on Summoner's Rift
    SUMMONERS_RIFT_5V5_BLIND_PICK = 430,
    // 5v5 Ranked Flex games on Summoner's Rift
    SUMMONERS_RIFT_5V5_RANKED_FLEX = 440,
    // 5v5 ARAM games on Howling Abyss
    HOWLING_ABYSS_5V5_ARAM = 450,
    // 3v3 Blind Pick games on Twisted Treeline
    // Deprecated in patch 9.23
    TWISTED_TREELINE_3V3_BLIND_PICK = 460,
    // 3v3 Ranked Flex games on Twisted Treeline
    // Deprecated in patch 9.23
    TWISTED_TREELINE_3V3_RANKED_FLEX_DEPRECATED_470 = 470,
    // Blood Hunt Assassin games on Summoner's Rift
    SUMMONERS_RIFT_BLOOD_HUNT_ASSASSIN = 600,
    // Dark Star: Singularity games on Cosmic Ruins
    COSMIC_RUINS_DARK_STAR_SINGULARITY = 610,
    // Clash games on Summoner's Rift
    SUMMONERS_RIFT_CLASH = 700,
    // Co-op vs. AI Intermediate Bot games on Twisted Treeline
    // Deprecated in patch 9.23
    TWISTED_TREELINE_CO_OP_VS_AI_INTERMEDIATE_BOT = 800,
    // Co-op vs. AI Intro Bot games on Twisted Treeline
    // Deprecated in patch 9.23
    TWISTED_TREELINE_CO_OP_VS_AI_INTRO_BOT = 810,
    // Co-op vs. AI Beginner Bot games on Twisted Treeline
    TWISTED_TREELINE_CO_OP_VS_AI_BEGINNER_BOT = 820,
    // Co-op vs. AI Intro Bot games on Summoner's Rift
    SUMMONERS_RIFT_CO_OP_VS_AI_INTRO_BOT = 830,
    // Co-op vs. AI Beginner Bot games on Summoner's Rift
    SUMMONERS_RIFT_CO_OP_VS_AI_BEGINNER_BOT = 840,
    // Co-op vs. AI Intermediate Bot games on Summoner's Rift
    SUMMONERS_RIFT_CO_OP_VS_AI_INTERMEDIATE_BOT = 850,
    // URF games on Summoner's Rift
    SUMMONERS_RIFT_URF = 900,
    // Ascension games on Crystal Scar
    CRYSTAL_SCAR_ASCENSION = 910,
    // Legend of the Poro King games on Howling Abyss
    HOWLING_ABYSS_LEGEND_OF_THE_PORO_KING = 920,
    // Nexus Siege games on Summoner's Rift
    SUMMONERS_RIFT_NEXUS_SIEGE = 940,
    // Doom Bots Voting games on Summoner's Rift
    SUMMONERS_RIFT_DOOM_BOTS_VOTING = 950,
    // Doom Bots Standard games on Summoner's Rift
    SUMMONERS_RIFT_DOOM_BOTS_STANDARD = 960,
    // Star Guardian Invasion: Normal games on Valoran City Park
    VALORAN_CITY_PARK_STAR_GUARDIAN_INVASION_NORMAL = 980,
    // Star Guardian Invasion: Onslaught games on Valoran City Park
    VALORAN_CITY_PARK_STAR_GUARDIAN_INVASION_ONSLAUGHT = 990,
    // PROJECT: Hunters games on Overcharge
    OVERCHARGE_PROJECT_HUNTERS = 1000,
    // Snow ARURF games on Summoner's Rift
    SUMMONERS_RIFT_SNOW_ARURF = 1010,
    // One for All games on Summoner's Rift
    SUMMONERS_RIFT_ONE_FOR_ALL = 1020,
    // Odyssey Extraction: Intro games on Crash Site
    CRASH_SITE_ODYSSEY_EXTRACTION_INTRO = 1030,
    // Odyssey Extraction: Cadet games on Crash Site
    CRASH_SITE_ODYSSEY_EXTRACTION_CADET = 1040,
    // Odyssey Extraction: Crewmember games on Crash Site
    CRASH_SITE_ODYSSEY_EXTRACTION_CREWMEMBER = 1050,
    // Odyssey Extraction: Captain games on Crash Site
    CRASH_SITE_ODYSSEY_EXTRACTION_CAPTAIN = 1060,
    // Odyssey Extraction: Onslaught games on Crash Site
    CRASH_SITE_ODYSSEY_EXTRACTION_ONSLAUGHT = 1070,
    // Teamfight Tactics games on Convergence
    CONVERGENCE_TEAMFIGHT_TACTICS = 1090,
    // Ranked Teamfight Tactics games on Convergence
    CONVERGENCE_RANKED_TEAMFIGHT_TACTICS = 1100,
    // Teamfight Tactics Tutorial games on Convergence
    CONVERGENCE_TEAMFIGHT_TACTICS_TUTORIAL = 1110,
    // Teamfight Tactics 1v0 testing games on Convergence
    CONVERGENCE_TEAMFIGHT_TACTICS_1V0_TESTING = 1111,
    // Nexus Blitz games on Nexus Blitz
    // Deprecated in patch 9.2 in favor of queueId 1300
    NEXUS_BLITZ_NEXUS_BLITZ_DEPRECATED_1200 = 1200,
    // Nexus Blitz games on Nexus Blitz
    NEXUS_BLITZ_NEXUS_BLITZ = 1300,
    // Tutorial 1 games on Summoner's Rift
    SUMMONERS_RIFT_TUTORIAL_1 = 2000,
    // Tutorial 2 games on Summoner's Rift
    SUMMONERS_RIFT_TUTORIAL_2 = 2010,
    // Tutorial 3 games on Summoner's Rift
    SUMMONERS_RIFT_TUTORIAL_3 = 2020,
}

export const friendlyQueueNames: Record<number, string> = {
    0: "Custom",
    2: "Normal Blind",
    4: "Ranked Solo",
    6: "Ranked Premade",
    7: "Co-op vs AI",
    8: "Normal Twisted Treeline",
    9: "Ranked flex Twisted Treeline",
    14: "Normal Draft",
    16: "Dominion Blind",
    17: "Dominion Draft",
    25: "Dominion Co-op vs AI",
    31: "Co-op vs AI intro",
    32: "Co-op vs AI beginner",
    33: "Co-op vs AI intermediate",
    41: "Ranked team Twisted Treeline",
    42: "Ranked team",
    52: "Co-op vs AI Twisted Treeline",
    61: "Team builder",
    65: "Aram",
    67: "Co-op vs AI Aram",
    70: "One for all",
    72: "1v1 Snowdown Showdown",
    73: "2v2 Snowdown Showdown",
    75: "6v6 Hexakill",
    76: "URF",
    78: "Aram One for all Mirror Mode",
    83: "URF Co-op vs AI",
    91: "Doom bots rank 1",
    92: "Doom bots rank 2",
    93: "Doom bots rank 5",
    96: "Ascension",
    98: "Hexakill Twisted Treeline",
    100: "Aram (Butcher's Bridge)",
    300: "Legend of the Poro King",
    310: "Nemesis",
    313: "Black Market Brawlers",
    315: "Nexus Siege",
    317: "Definitely not Dominion",
    318: "Arurf",
    325: "All random Summoners Rift",
    400: "Normal Draft",
    410: "Ranked Dynamic",
    420: "Ranked Solo",
    430: "Ranked Blind",
    440: "Ranked Flex",
    450: "Aram",
    460: "Twisted Treeline Blind",
    470: "Twisted Treeline Ranked Flex",
    600: "Blood Hunt Assassin",
    610: "Cosmic Ruins Dark Star Singularity",
    700: "Clash",
    800: "Twisted Treeline Co-op vs AI Intermediate",
    810: "Twisted Treeline Co-op vs AI Intro",
    820: "Twisted Treeline Co-op vs AI Beginner",
    830: "Co-op vs AI intro",
    840: "Co-op vs AI beginner",
    850: "Co-op vs AI intermediate",
    900: "URF",
    910: "Ascension",
    920: "Legend of the Poro King",
    940: "Nexus Siege",
    950: "Doom bots voting",
    960: "Doom bots standard",
    980: "Star Guardian Invasion Normal",
    990: "Star Guardian Invasion Onslaught",
    1000: "Project Hunters",
    1010: "Snow Arurf",
    1020: "One for All",
    1030: "Oddysey Extraction intro",
    1040: "Oddysey Extraction cadet",
    1050: "Oddysey Extraction crewmember",
    1060: "Oddysey Extraction captain",
    1070: "Oddysey Extraction onslaught",
    1090: "TFT",
    1100: "TFT Ranked",
    1110: "TFT tutorial",
    1111: "TFT 1v0 testing",
    1200: "Nexus Blitz",
    1300: "Nexus Blitz",
    2000: "Tutorial 1",
    2010: "Tutorial 2",
    2020: "Tutorial 3",
};

export function getQueueFromId(queueId: number): string {
    if (queueId in friendlyQueueNames) {
        return friendlyQueueNames[queueId];
    }
    return Queue[queueId];
}

export const queueSeriousness = {
    Normal: [
        2, 8, 14, 16, 17, 61, 65, 70, 72, 73, 75, 76, 78, 96, 98, 100, 300, 310, 313, 315, 317, 318, 325, 400, 430, 450,
        460, 600, 610, 900, 910, 920, 940, 1000, 1010, 1020, 1300,
    ],
    Ranked: [4, 6, 9, 41, 42, 410, 420, 440, 470, 700],
    Bots: [
        7, 25, 31, 32, 33, 52, 67, 83, 91, 92, 93, 800, 810, 820, 830, 840, 850, 950, 960, 980, 990, 1030, 1040, 1050,
        1060, 1070, 2000, 2010, 2020,
    ],
};

export const queuePickType = {
    Blind: [2, 7, 8, 16, 25, 31, 32, 33, 52, 75, 83, 91, 92, 93, 430, 460, 800, 810, 820, 830, 840, 850, 950, 960],
    Draft: [4, 6, 9, 14, 17, 41, 42, 61, 400, 410, 420, 440, 470, 700],
    "Blind Draft": [72, 73, 76, 96, 98, 300, 313, 315, 317, 900, 910, 920, 940, 1300],
    Random: [65, 67, 100, 318, 325, 450, 1010],
    Vote: [70, 1020],
    "Mirror Vote": [78],
    Nemesis: [310],
    "Limited Blind": [600, 980, 990, 1000, 1030, 1040, 1050, 1060, 1070],
    Fixed: [610, 2000, 2010, 2020],
};

export const queuePlayerSize = {
    "1": [72],
    "2": [73],
    "3": [8, 9, 41, 52, 460, 470, 610, 800, 810, 820],
    "5": [
        2, 4, 6, 7, 14, 16, 17, 25, 31, 32, 33, 42, 61, 65, 67, 70, 76, 78, 83, 91, 92, 93, 96, 100, 300, 310, 313, 315,
        317, 318, 325, 400, 410, 420, 430, 440, 450, 600, 700, 830, 840, 850, 900, 910, 920, 940, 950, 960, 980, 990,
        1010, 1020, 1030, 1040, 1050, 1060, 1070, 1300, 2000, 2010, 2020,
    ],
    "6": [75, 98, 1000],
};

export const queueMap = {
    "Summoner's Rift": [
        2, 4, 6, 7, 14, 31, 32, 33, 42, 61, 70, 75, 76, 83, 91, 92, 310, 313, 315, 318, 325, 400, 410, 420, 430, 440,
        600, 700, 830, 840, 850, 900, 940, 950, 960, 1010, 1020, 2000, 2010, 2020,
    ],
    "Twisted Treeline": [8, 9, 41, 52, 98, 460, 470, 800, 810, 820],
    "Crystal Scar": [16, 17, 25, 96, 317, 910],
    "Howling Abyss": [65, 67, 72, 73, 78, 300, 450, 920],
    "Butcher's Bridge": [100],
    "Cosmic Ruins": [610],
    "Valoran City Park": [980, 990],
    "Substructure 43": [1000],
    "Crash Site": [1030, 1040, 1050, 1060, 1070],
    "Temple of Lily and Lotus": [1300],
};

export const queueMode = {
    Normal: [2, 8, 14, 61, 310, 325, 400, 430, 460],
    "Ranked Solo": [4, 420],
    "Ranked Flex": [9, 410, 440, 470],
    "Ranked Team": [6, 41, 42],
    Clash: [700],
    "Co-op vs AI": [7, 25, 31, 32, 33, 52, 67, 83, 800, 810, 820, 830, 840, 850],
    Dominion: [16, 17, 25],
    "Definitely Not Dominion": [317],
    ARAM: [65, 67, 100, 450],
    "Snowdown Showdown": [72, 73],
    Hexakill: [75, 98],
    URF: [76, 83, 318, 900, 1010],
    OFA: [70, 78, 1020],
    "Doom bots": [91, 92, 93, 950, 960],
    Ascension: [96, 910],
    "Poro King": [300, 920],
    "Black Market Brawlers": [313],
    "Nexus Siege": [315, 940],
    "Blood Hunt Assassin": [600],
    "Dark Star Singularity": [610],
    Invasion: [980, 990],
    Overcharge: [1000],
    "Odyssey: Extraction": [1030, 1040, 1050, 1060, 1070],
    "Nexus Blitz": [1300],
    Tutorial: [2000, 2010, 2020],
};
