use std::io::{BufRead, Cursor, Read};

#[derive(Debug)]
pub struct Metadata {
    pub file_format: String,
    pub creation_data: ReleaseData,
    pub modification_data: ReleaseData,
}

#[derive(Debug)]
pub struct ReleaseData {
    pub datetime: String,
    pub idp: String, // TODO: What does IDP stand for?
    pub program: String,
    pub version: String,
    pub license: String,
    pub scouter_name: String,
}

#[derive(Debug)]
pub struct Game {
    pub date: String,
    pub time: String,
    pub season: String,
    pub game_type: String,
}

#[derive(Debug)]
pub struct Team {
    pub team_id: String,
    pub team_name: String,
    pub sets_won: u8,
    pub head_coach: String,
    pub assistant_coaches: String,
}

#[derive(Debug)]
pub struct SetPoints {
    pub home: u8,
    pub visiting: u8,
}

#[derive(Debug)]
pub struct Set {
    pub set_number: u8,
    pub first_quarter: SetPoints,
    pub second_quarter: SetPoints,
    pub third_quarter: SetPoints,
    pub fourth_quarter: SetPoints,
    pub duration: String,
}

#[derive(Debug)]
pub struct Player {
    pub team_id: String,
    pub player_number: u8,
    pub player_id: String,
    pub last_name: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Action {
    pub code: String,
    pub code_explanation: CodeExplanation,
    pub point_phase: String,
    pub attack_phase: String,
    pub start_coordinate: String,
    pub mid_coordinate: String,
    pub end_coordinate: String,
    pub time: String,
    pub set: u8,
    pub home_rotation: u8,
    pub visiting_rotation: u8,
    pub video_file_number: u8,
    pub video_time: String,
}

#[derive(Debug)]
pub enum TeamSide {
    Home,
    Visiting,
}

#[derive(Debug)]
pub enum Skill {
    Serve,
    Reception,
    Attack,
    Block,
    Dig,
    Set,
    FreeBall,
}

#[derive(Debug)]
pub enum ActionType {
    High,
    Medium,
    Quick,
    Tense,
    Super,
    Fast,
    Other,
}

// See page 30 of the Data Volley Scout manual for more information on evaluations per action type
#[derive(Debug)]
pub enum Evaluation {
    Equal,
    Slash,
    Minus,
    Exclamation,
    Plus,
    Hashtag,
}

// From page 27 of the Data Volley Scout manual (https://dataprojectwebsoftware.blob.core.windows.net/software/dvw4media/DataVolleyMedia_handbook.pdf)
// Only parses the main code, not the advanced and extended codes
#[derive(Debug)]
pub struct CodeExplanation {
    pub team: TeamSide,
    pub player_number: u8,
    pub skill: Skill,
    pub action_type: ActionType,
    pub evaluation: Evaluation,
}

#[derive(Debug)]
pub struct ScoutFile {
    pub metadata: Metadata,
    pub game: Game,
    pub home_team: Team,
    pub visiting_team: Team,
    pub sets: Vec<Set>,
    pub home_players: Vec<Player>,
    pub visiting_players: Vec<Player>,
    pub actions: Vec<Action>,
}

impl Metadata {
    pub fn new(
        file_format: String,
        creation_data: ReleaseData,
        modification_data: ReleaseData,
    ) -> Metadata {
        Metadata {
            file_format,
            creation_data,
            modification_data,
        }
    }
}

impl ReleaseData {
    pub fn new(
        datetime: String,
        idp: String,
        program: String,
        version: String,
        license: String,
        scouter_name: String,
    ) -> ReleaseData {
        ReleaseData {
            datetime,
            idp,
            program,
            version,
            license,
            scouter_name,
        }
    }
}

impl Game {
    pub fn new(date: String, time: String, season: String, game_type: String) -> Game {
        Game {
            date,
            time,
            season,
            game_type,
        }
    }
}

impl Team {
    pub fn new(
        team_id: String,
        team_name: String,
        sets_won: u8,
        head_coach: String,
        assistant_coaches: String,
    ) -> Team {
        Team {
            team_id,
            team_name,
            sets_won,
            head_coach,
            assistant_coaches,
        }
    }
}

impl SetPoints {
    pub fn new(home: u8, visiting: u8) -> SetPoints {
        SetPoints { home, visiting }
    }
}

impl Set {
    pub fn new(
        set_number: u8,
        first_quarter: SetPoints,
        second_quarter: SetPoints,
        third_quarter: SetPoints,
        fourth_quarter: SetPoints,
        duration: String,
    ) -> Set {
        Set {
            set_number,
            first_quarter,
            second_quarter,
            third_quarter,
            fourth_quarter,
            duration,
        }
    }
}

impl Player {
    pub fn new(
        team_id: String,
        player_number: u8,
        player_id: String,
        last_name: String,
        name: String,
    ) -> Player {
        Player {
            team_id,
            player_number,
            player_id,
            last_name,
            name,
        }
    }
}

impl CodeExplanation {
    pub fn new(code: String) -> CodeExplanation {
        let code = code.trim().chars().collect::<Vec<char>>();

        if code.len() < 6 {
            return CodeExplanation {
                team: TeamSide::Home,
                player_number: 0,
                skill: Skill::FreeBall,
                action_type: ActionType::Other,
                evaluation: Evaluation::Equal,
            }; // TODO: Return an error instead of a default value
        }

        let team = match code[0] {
            '*' => TeamSide::Home,
            'a' => TeamSide::Visiting,
            _ => TeamSide::Home, // TODO: Return an error instead of a default value
        };

        let player_number = code[1..3].iter().collect::<String>().parse().unwrap_or(0);

        let skill = match code[3] {
            'S' => Skill::Serve,
            'R' => Skill::Reception,
            'A' => Skill::Attack,
            'B' => Skill::Block,
            'D' => Skill::Dig,
            'E' => Skill::Set,
            'F' => Skill::FreeBall,
            _ => Skill::FreeBall, // TODO: Return an error instead of a default value
        };

        let action_type = match code[4] {
            'H' => ActionType::High,
            'M' => ActionType::Medium,
            'Q' => ActionType::Quick,
            'T' => ActionType::Tense,
            'S' => ActionType::Super,
            'N' => ActionType::Fast,
            'O' => ActionType::Other,
            _ => ActionType::Other, // TODO: Return an error instead of a default value
        };

        let evaluation = match code[5] {
            '=' => Evaluation::Equal,
            '/' => Evaluation::Slash,
            '-' => Evaluation::Minus,
            '!' => Evaluation::Exclamation,
            '+' => Evaluation::Plus,
            '#' => Evaluation::Hashtag,
            _ => Evaluation::Equal, // TODO: Return an error instead of a default value
        };

        CodeExplanation {
            team,
            player_number,
            skill,
            action_type,
            evaluation,
        }
    }
}

impl Action {
    pub fn new(
        code: String,
        point_phase: String,
        attack_phase: String,
        start_coordinate: String,
        mid_coordinate: String,
        end_coordinate: String,
        time: String,
        set: u8,
        home_rotation: u8,
        visiting_rotation: u8,
        video_file_number: u8,
        video_time: String,
    ) -> Action {
        Action {
            code: code.clone(),
            code_explanation: CodeExplanation::new(code),
            point_phase,
            attack_phase,
            start_coordinate,
            mid_coordinate,
            end_coordinate,
            time,
            set,
            home_rotation,
            visiting_rotation,
            video_file_number,
            video_time,
        }
    }
}

impl ScoutFile {
    pub fn new(
        metadata: Metadata,
        game: Game,
        home_team: Team,
        visiting_team: Team,
        sets: Vec<Set>,
        home_players: Vec<Player>,
        visiting_players: Vec<Player>,
        actions: Vec<Action>,
    ) -> ScoutFile {
        ScoutFile {
            metadata,
            game,
            home_team,
            visiting_team,
            sets,
            home_players,
            visiting_players,
            actions,
        }
    }
}

pub fn read_scout_file(mut input: impl Read) -> Result<ScoutFile, std::io::Error> {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let mut reader = Cursor::new(buffer);

    let metadata = read_metadata(&mut reader)?;
    let game = read_game(&mut reader)?;
    let home_team = read_team(&mut reader, false)?;
    let visiting_team = read_team(&mut reader, true)?;
    // skip_category(&mut reader, "[3MORE]")?;
    // skip_category(&mut reader, "[3COMMENTS]")?;
    skip_until(&mut reader, "[3SET]")?;
    let sets = read_sets(&mut reader)?;
    let home_players = read_players(&mut reader)?;
    let visiting_players = read_players(&mut reader)?;
    // skip_category(&mut reader, "[3ATTACKCOMBINATION]")?;
    // skip_category(&mut reader, "[3SETTERCALL]")?;
    // skip_category(&mut reader, "[3WINNINGSYMBOLS]")?;
    // skip_category(&mut reader, "[3RESERVE]")?;
    skip_until(&mut reader, "[3SCOUT]")?;
    let actions = read_actions(&mut reader)?;

    Ok(ScoutFile {
        metadata,
        game,
        home_team,
        visiting_team,
        sets,
        home_players,
        visiting_players,
        actions,
    })
}

macro_rules! metadata_field {
    ($field:expr) => {
        $field
            .split(":")
            .last()
            .unwrap_or("ERROR PARSING")
            .to_string()
            .trim()
            .to_string()
    };
}

pub fn read_metadata(reader: &mut Cursor<String>) -> Result<Metadata, std::io::Error> {
    let mut header = String::new();
    reader.read_line(&mut header)?;

    if header.trim() != "[3DATAVOLLEYSCOUT]" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid metadata header",
        ));
    }

    let mut file_format = String::new();
    reader.read_line(&mut file_format)?;
    file_format = metadata_field!(file_format);

    let creation_data = read_release_data(reader)?;

    let modification_data = read_release_data(reader)?;

    Ok(Metadata::new(file_format, creation_data, modification_data))
}

pub fn read_release_data(reader: &mut Cursor<String>) -> Result<ReleaseData, std::io::Error> {
    let mut datetime = String::new();
    reader.read_line(&mut datetime)?;
    datetime = metadata_field!(datetime);

    let mut idp = String::new();
    reader.read_line(&mut idp)?;
    idp = metadata_field!(idp);

    let mut program = String::new();
    reader.read_line(&mut program)?;
    program = metadata_field!(program);

    let mut version = String::new();
    reader.read_line(&mut version)?;
    version = metadata_field!(version);

    let mut license = String::new();
    reader.read_line(&mut license)?;
    license = metadata_field!(license);

    let mut scouter_name = String::new();
    reader.read_line(&mut scouter_name)?;
    scouter_name = metadata_field!(scouter_name);

    Ok(ReleaseData::new(
        datetime,
        idp,
        program,
        version,
        license,
        scouter_name,
    ))
}

pub fn read_game(reader: &mut Cursor<String>) -> Result<Game, std::io::Error> {
    let mut header = String::new();
    reader.read_line(&mut header)?;

    if header.trim() != "[3MATCH]" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid game header, got: {}", header),
        ));
    }

    let mut data = String::new();
    reader.read_line(&mut data)?;

    let mut splitted = data.split(";").map(|s| s.trim().to_string());

    let (date, time, season, game_type) = (
        splitted
            .nth(0)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid date"))?
            .trim()
            .to_string(),
        splitted
            .nth(2)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid time"))?
            .trim()
            .to_string(),
        splitted
            .nth(3)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid season"))?
            .trim()
            .to_string(),
        splitted
            .nth(4)
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid game type")
            })?
            .trim()
            .to_string(),
    );

    // skip a line
    reader.read_line(&mut String::new())?;

    Ok(Game::new(date, time, season, game_type))
}

pub fn read_team(reader: &mut Cursor<String>, skip_header: bool) -> Result<Team, std::io::Error> {
    if !skip_header {
        let mut header = String::new();
        reader.read_line(&mut header)?;

        if header.trim() != "[3TEAMS]" {
            panic!("Invalid team header, got: {}", header);
        }
    }

    let mut data = String::new();
    reader.read_line(&mut data)?;

    let splitted = data
        .split(";")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    let (team_id, team_name, sets_won, head_coach, assistant_coaches) = (
        splitted
            .get(0)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid team id"))?
            .trim()
            .to_string(),
        splitted
            .get(1)
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid team name")
            })?
            .trim()
            .to_string(),
        splitted
            .get(2)
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid sets won")
            })?
            .trim()
            .parse()
            .unwrap_or_else(|_| 0),
        splitted
            .get(3)
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid head coach")
            })?
            .trim()
            .to_string(),
        splitted
            .get(4)
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid assistant coaches")
            })?
            .trim()
            .to_string(),
    );

    Ok(Team::new(
        team_id,
        team_name,
        sets_won,
        head_coach,
        assistant_coaches,
    ))
}

macro_rules! set_quarter {
    ($data:expr, $quarter_n: expr, $quarter:expr) => {{
        let raw_value = $data.get($quarter_n).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid {} quarter", $quarter),
            )
        })?;

        let trimmed_value = raw_value.trim();

        if trimmed_value.is_empty() {
            SetPoints::new(0, 0)
        } else {
            let (home, visiting) = trimmed_value.split_once("-").ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid {} quarter", $quarter),
                )
            })?;

            let (home, visiting) = (
                home.trim().parse().unwrap(),
                visiting.trim().parse().unwrap(),
            );

            SetPoints::new(home, visiting)
        }
    }};
}

pub fn read_sets(reader: &mut Cursor<String>) -> Result<Vec<Set>, std::io::Error> {
    let mut sets = Vec::new();

    let mut header = String::new();
    reader.read_line(&mut header).unwrap();

    if header.trim() != "[3SET]" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid sets header, got: {}", header),
        ));
    }

    for i in 0..5 {
        let mut data = String::new();
        reader.read_line(&mut data).unwrap();

        let splitted = data
            .split(";")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        sets.push(Set {
            set_number: i + 1,
            first_quarter: set_quarter!(splitted, 1, "first"),
            second_quarter: set_quarter!(splitted, 2, "second"),
            third_quarter: set_quarter!(splitted, 3, "third"),
            fourth_quarter: set_quarter!(splitted, 4, "fourth"),
            duration: splitted.get(5).unwrap().to_string(),
        });
    }

    Ok(sets)
}

pub fn read_players(reader: &mut Cursor<String>) -> Result<Vec<Player>, std::io::Error> {
    let mut players = Vec::new();

    let mut header = String::new();
    reader.read_line(&mut header)?;

    if !header.trim().starts_with("[3PLAYERS") {
        panic!("Invalid players header, got: {}", header);
    }

    loop {
        let mut data = String::new();
        reader.read_line(&mut data)?;

        if data.trim().starts_with("[") {
            reader.set_position(reader.position() - data.len() as u64);
            break;
        }

        let splitted = data
            .split(";")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        players.push(Player::new(
            splitted.get(0).unwrap().into(),
            splitted.get(1).unwrap().parse().unwrap(),
            splitted.get(8).unwrap().into(),
            splitted.get(9).unwrap().into(),
            splitted.get(10).unwrap().into(),
        ));
    }

    Ok(players)
}

pub fn read_actions(reader: &mut Cursor<String>) -> Result<Vec<Action>, std::io::Error> {
    let mut actions = Vec::new();

    let mut header = String::new();
    reader.read_line(&mut header)?;

    if !header.trim().starts_with("[3SCOUT]") {
        panic!("Invalid actions header, got: {}", header);
    }

    loop {
        let mut data = String::new();
        reader.read_line(&mut data)?;

        if data.trim().is_empty() {
            break;
        }

        let splitted = data
            .split(";")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        // TODO: Figure out what indexes all these fields are
        actions.push(Action::new(
            splitted.get(0).unwrap().into(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            0,
            0,
            0,
            0,
            String::new(),
        ));
    }

    Ok(actions)
}

fn skip_category(reader: &mut Cursor<String>, category: &str) -> Result<(), std::io::Error> {
    let mut header = String::new();
    reader.read_line(&mut header)?;

    if !header.trim().starts_with(category) {
        panic!("Invalid {} header, got: {}", category, header);
    }

    loop {
        let mut data = String::new();
        reader.read_line(&mut data)?;

        if data.trim().starts_with("[") {
            reader.set_position(reader.position() - data.len() as u64);
            break;
        }
    }

    Ok(())
}

fn skip_until(reader: &mut Cursor<String>, category: &str) -> Result<(), std::io::Error> {
    loop {
        let mut data = String::new();
        reader.read_line(&mut data)?;

        if data.trim().starts_with(category) {
            reader.set_position(reader.position() - data.len() as u64);
            break;
        }
    }

    Ok(())
}
