enum SubmarineCommand {
    Forward(i32),
    Down(i32),
    Up(i32),
}

struct SubmarinePosition {
    depth: i32,
    x: i32,
}

struct SubmarineAimPosition {
    depth: i32,
    x: i32,
    aim: i32,
}

fn parse_command_string(command_strings: Vec<&str>) -> Result<Vec<SubmarineCommand>, &str> {
    let mut parsed_commands: Vec<SubmarineCommand> = Vec::new();

    for command_string in command_strings {
        let command_parts: Vec<&str> = command_string.split(' ').collect();
        if command_parts.len() != 2 {
            return Err("Command string invalid!");
        }
        let command = command_parts[0];
        let parameter = match command_parts[1].parse::<i32>() {
            Ok(value) => value,
            Err(_) => return Err("Invalid parameter found!"),
        };
        let parsed_command = match command {
            "forward" => SubmarineCommand::Forward(parameter),
            "up" => SubmarineCommand::Up(parameter),
            "down" => SubmarineCommand::Down(parameter),
            _ => return Err("Invalid command found"),
        };
        parsed_commands.push(parsed_command);
    }
    return Ok(parsed_commands);
}

pub fn calculate_aim_path(command_strings: Vec<&str>) -> Result<i32, &str> {
    let commands = match parse_command_string(command_strings) {
        Ok(commands) => commands,
        Err(err) => return Err(err),
    };
    let mut position = SubmarineAimPosition {
        aim: 0,
        depth: 0,
        x: 0,
    };
    for command in commands {
        match command {
            SubmarineCommand::Forward(parameter) => {
                position.x += parameter;
                position.depth += position.aim * parameter;
            }
            SubmarineCommand::Up(parameter) => position.aim -= parameter,
            SubmarineCommand::Down(parameter) => position.aim += parameter,
        }
    }
    return Ok(position.x * position.depth);
}

pub fn calculate_path(command_strings: Vec<&str>) -> Result<i32, &str> {
    let commands = match parse_command_string(command_strings) {
        Ok(commands) => commands,
        Err(err) => return Err(err),
    };
    let mut position = SubmarinePosition { depth: 0, x: 0 };
    for command in commands {
        match command {
            SubmarineCommand::Forward(parameter) => position.x += parameter,
            SubmarineCommand::Up(parameter) => position.depth -= parameter,
            SubmarineCommand::Down(parameter) => position.depth += parameter,
        }
    }
    return Ok(position.x * position.depth);
}

#[cfg(test)]
mod tests {
    use crate::solutions::day_02::{calculate_path, calculate_aim_path};

    #[test]
    fn test_known_answer() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(calculate_path(input).unwrap(), 150);
    }

    #[test]
    fn test_known_aim_answer() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(calculate_aim_path(input).unwrap(), 900);
    }
}
