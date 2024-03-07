pub mod ytdl;

use dotenv::dotenv;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]

enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "kill server")]
    Kill,
    #[command(description = "Download a youtube video")]
    Ytdl(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::Kill => {
            bot.send_message(msg.chat.id, "Killing server...").await?;
            std::process::exit(0);
        }
        Command::Ytdl(url) => {
            bot.send_message(msg.chat.id, "Downloading...").await?;
            let fname = ytdl::ytdl(&url).await;
            let file = InputFile::file(fname);
            bot.send_audio(msg.chat.id, file).await?
        }
    };

    Ok(())
}
