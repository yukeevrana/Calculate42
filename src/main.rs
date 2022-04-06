use teloxide::{prelude2::*, utils::command::BotCommand};
mod calculate42;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let reply;

        match message.text() {
            Some(t) => {
                if let Ok(command) = Command::parse(t, "bot") {
                    match command {
                        Command::Help => reply = Command::descriptions()
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
    Help
}