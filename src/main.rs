use clap::Parser;
use commands::create_signup;
use eyre::Context as EyreContext;
use serenity::{
    Client,
    all::{
        Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
        GatewayIntents, Interaction, Ready,
    },
    async_trait,
};

pub mod commands;
pub mod db;

/// Commandline arguments.
#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, env = "DISCORD_TOKEN")]
    discord_token: String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    color_eyre::install()?;

    db::queries::setup();

    let mut client = Client::builder(cli.discord_token, GatewayIntents::empty())
        .event_handler(DiscordHandler)
        .await
        .wrap_err("Failed to setup discord client")?;

    client
        .start()
        .await
        .wrap_err("Failed to start discord client")?;

    Ok(())
}

struct DiscordHandler;

#[async_trait]
impl EventHandler for DiscordHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        println!("Interaction create called");

        match interaction {
            Interaction::Command(command) => {
                println!("Received command interaction: {command:#?}");

                let content = match command.data.name.as_str() {
                    create_signup::CREATE_SIGNUP_COMMAND_NAME => {
                        create_signup::run(&ctx, &command)
                            .await
                            .expect("failed to create signup");
                        None
                    }
                    _ => Some("not implemented :(".to_string()),
                };

                if let Some(content) = content {
                    let data = CreateInteractionResponseMessage::new().content(content);
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(err) = command.create_response(&ctx.http, builder).await {
                        println!("Cannot respond to slash command: {err}");
                    }
                }
            }
            Interaction::Component(component) => todo!(), //component.data.custom_id,
            Interaction::Modal(_) => {} // This is handled by the creator of the modal.
            i => eprintln!("Unsupported command {i:?}"),
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected to {:?}!", ready.user.name, ready.guilds);

        for guild in ready.guilds.iter() {
            let commands = guild
                .id
                .set_commands(&ctx.http, vec![commands::create_signup::register()])
                .await
                .expect("Failed to set commands");

            println!("The following commands are available: {commands:?}")
        }

        //let guild_id = GuildId::new()
    }
}
