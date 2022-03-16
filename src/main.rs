// use std::io;

// mod calculate42;

// fn main() {
//     loop {
//         let mut buffer = String::new();
//         let stdin = io::stdin();

//         stdin.read_line(&mut buffer).expect("Input error"); 

//         let string_expr = String::from(buffer.trim_end());

//         match calculate42::try_calculate(&string_expr) {
//             Some(n) => println!("{}", n),
//             None => println!("Failed to calculate")
//         }
//     }
// }

use teloxide::prelude2::*;

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
                match calculate42::try_calculate(&String::from(t)) {
                    Some(n) => { reply = format!("{}", n) },
                    None => { reply = String::from("Failed to calculate") }
                }
            },
            None => reply = String::from("Eh?")
        }
        
        bot.send_message(message.chat.id, reply).await?;
        // bot.send_dice(message.chat.id).await?;
        respond(())
    })
    .await;
}