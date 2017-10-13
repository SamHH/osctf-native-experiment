use components::team::Team;

pub fn get_std() -> (Team, Team) {
    let std_team_1: Team = Team {
        id: "std_team_1",
        name: "Orange Team",
        color_rgba: [1.0, 0.405, 0.24, 1.0],
    };

    let std_team_2: Team = Team {
        id: "std_team_2",
        name: "Purple Team",
        color_rgba: [0.332, 0.22, 1.0, 1.0],
    };

    return (std_team_1, std_team_2);
}
