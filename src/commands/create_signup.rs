use std::{collections::HashMap, time::Duration};

use chrono::{NaiveDate, NaiveTime};
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateQuickModal,
};

pub const CREATE_SIGNUP_COMMAND_NAME: &'static str = "new-lol-signup";

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let modal = CreateQuickModal::new("When are we gaming")
        .timeout(Duration::from_secs(600))
        .short_field("Date (format `YYYY-MM-DD`)")
        .short_field("Opponents (comma separated)")
        .short_field("Game times (format `HH:MM` comma separated)");
    let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    println!("Received modal inputs: {inputs:?}");
    let form = match handle_create_inputs(inputs) {
        Ok(f) => f,
        Err(err) => {
            response
                .interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(err),
                    ),
                )
                .await?;
            return Ok(());
        }
    };

    Ok(())
}

struct CreateResponse {
    date: NaiveDate,
    times_to_opponents_map: HashMap<NaiveTime, String>,
}

fn handle_create_inputs(inputs: Vec<String>) -> Result<CreateResponse, String> {
    let (date, opponents, times) = (&inputs[0], &inputs[1], &inputs[2]);

    let parsed_date: NaiveDate = match date.parse() {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Failed to parse date {date} due to err: {err:?}");
            return Err(format!("Invalid date, expected `yyyy-MM-dd` got `{date}`"));
        }
    };

    let mut parsed_times = Vec::new();
    for time in times.split(",") {
        let time: NaiveTime = match time.parse() {
            Ok(t) => t,
            Err(err) => {
                eprintln!("Failed to parse time {time} due to err: {err:?}");
                return Err(format!("Invalid time, expected `HH:mm` got `{time}`"));
            }
        };

        parsed_times.push(time);
    }

    let opponents: Vec<String> = opponents.split(",").map(|s| s.to_string()).collect();

    if opponents.len() != parsed_times.len() {
        eprintln!(
            "Missmatched opponents and times, {opponents:?} ({}) vs {parsed_times:?} ({})",
            opponents.len(),
            parsed_times.len()
        );
        return Err(format!(
            "Missmatched opponents and game times, got {} opponents and {} game times",
            opponents.len(),
            parsed_times.len()
        ));
    }

    let mut map = HashMap::new();
    for (i, time) in parsed_times.into_iter().enumerate() {
        map.insert(time, opponents[i].clone());
    }

    Ok(CreateResponse {
        date: parsed_date,
        times_to_opponents_map: map,
    })
}

pub fn register() -> CreateCommand {
    CreateCommand::new(CREATE_SIGNUP_COMMAND_NAME)
        .description("Create a new league of legends signup")
}
