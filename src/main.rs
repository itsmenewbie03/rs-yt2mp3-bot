pub mod mp3tagger;
pub mod ytdl;

use dotenv::dotenv;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
use url::Url;

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
            let user_id = msg.from().unwrap().id;
            let owner_id = std::env::var("OWNER_ID").unwrap_or("NO_OWNER_ID".to_owned());
            if owner_id == "NO_OWNER_ID" {
                log::warn!("No owner id found in .env file");
            }
            if owner_id != user_id.to_string() && owner_id != "NO_OWNER_ID" {
                bot.send_message(msg.chat.id, "You are not the owner!")
                    .await?;
                return Ok(());
            }
            bot.send_message(msg.chat.id, "Killing server...").await?;
            log::info!("Killing server...");
            std::process::exit(0);
        }
        Command::Ytdl(url) => {
            let ret = bot.send_message(msg.chat.id, "Downloading...").await?;
            let ytdl_res = ytdl::ytdl(&url).await;
            let tagged_file = mp3tagger::add_tags(ytdl_res).await;
            let file = InputFile::file(&tagged_file);

            let future_send_audio = bot.send_audio(msg.chat.id, file);
            match future_send_audio.await {
                Ok(_) => {
                    log::info!("Audio Sent... Cleaning file: {:?}", &tagged_file);
                    std::fs::remove_file(&tagged_file).unwrap();
                }
                Err(e) => {
                    log::error!("Error sending audio: {:?} will still delete the file", e);
                    std::fs::remove_file(&tagged_file).unwrap();
                }
            };
            ret
        }
    };

    Ok(())
}
