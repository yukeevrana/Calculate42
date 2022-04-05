use teloxide::{prelude2::*, utils::command::BotCommand};
mod calculate42;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    // teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let reply;

        match message.text() {
            Some(t) => {
                if let Ok(command) = Command::parse(t, "cognito_bot") {
                    match command {
                        Command::Help => reply = Command::descriptions(),
                        // Command::Username(username) => {
                        //     reply = format!("Your username is @{}.", username)
                        // }
                        // Command::UsernameAndAge { username, age } => {
                        //     reply = format!("Your username is @{} and age is {}.", username, age)
                        // }
                    }
                } 
                else {
                    match calculate42::try_calculate(&String::from(t)) {
                        Ok(n) => { reply = format!("{}", n) },
                        Err(e) => { reply = String::from(format!("{}", e)) }
                    }
                }
            },
            None => reply = String::from("Eh?")
        }
        
        bot.send_message(message.chat.id, reply).await?;
        respond(())
    })
    .await;
}

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "Bot can calculate any (almost) mathematical expression. These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    // #[command(description = "handle a username.")]
    // Username(String),
    // #[command(description = "handle a username and an age.", parse_with = "split")]
    // UsernameAndAge { username: String, age: u8 },
}