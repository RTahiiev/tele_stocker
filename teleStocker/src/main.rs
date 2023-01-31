
mod stocks;

use teloxide::{
    prelude::*,
    types::{Update, UserId},
    utils::command::BotCommands,
};
use log::{error};

use stocks::get_stock;
use stocker_traits::Stock;


pub fn fibonacci(n: u64) -> u128 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1)
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dispatching features bot...");

    let bot = Bot::from_env();

    let parameters = ConfigParameters {
        bot_maintainer: UserId(240509207), // Paste your ID to run this bot.
        maintainer_username: None,
    };

    let handler = Update::filter_message()
        // You can use branching to define multiple ways in which an update will be handled. If the
        // first branch fails, an update will be passed to the second branch, and so on.
        .branch(
            dptree::entry()
                // Filter commands: the next handlers will receive a parsed `SimpleCommand`.
                .filter_command::<SimpleCommand>()
                // If a command parsing fails, this handler will not be executed.
                .endpoint(simple_commands_handler),
        )
        .branch(
            // Filter a maintainer by a user ID.
            dptree::filter(|cfg: ConfigParameters, msg: Message| {
                msg.from().map(|user| user.id == cfg.bot_maintainer).unwrap_or_default()
            })
            .filter_command::<MaintainerCommands>()
            .endpoint(|msg: Message, bot: Bot, cmd: MaintainerCommands| async move {
                match cmd {
                    MaintainerCommands::Fib { num } => {
                        let value: u128 = fibonacci(num);
                        bot.send_message(msg.chat.id, value.to_string()).await?;
                        Ok(())
                    }
                }
            }),
        );

    Dispatcher::builder(bot, handler)
        // Here you specify initial dependencies that all handlers will receive; they can be
        // database connections, configurations, and other auxiliary arguments. It is similar to
        // `actix_web::Extensions`.
        .dependencies(dptree::deps![parameters])
        // If no handler succeeded to handle an update, this closure will be called.
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If the dispatcher fails for some reason, execute this handler.
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(Clone)]
struct ConfigParameters {
    bot_maintainer: UserId,
    maintainer_username: Option<String>,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Simple commands")]
enum SimpleCommand {
    #[command(description = "shows this message.")]
    Help,
    #[command(description = "get stocks")]
    Stocks,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Maintainer commands")]
enum MaintainerCommands {
    #[command(description = "generate a n-th Fibonacci sequence")]
    Fib { num: u64 },
}

async fn simple_commands_handler(
    cfg: ConfigParameters,
    bot: Bot,
    me: teloxide::types::Me,
    msg: Message,
    cmd: SimpleCommand,
) -> Result<(), teloxide::RequestError> {
    let text = match cmd {
        SimpleCommand::Help => {
            if msg.from().unwrap().id == cfg.bot_maintainer {
                format!(
                    "{}\n\n{}",
                    SimpleCommand::descriptions(),
                    MaintainerCommands::descriptions()
                )
            } else if msg.chat.is_group() || msg.chat.is_supergroup() {
                SimpleCommand::descriptions().username_from_me(&me).to_string()
            } else {
                SimpleCommand::descriptions().to_string()
            }
        }
        SimpleCommand::Stocks => {
            let stock = get_stock().await.unwrap();
            stock.data()
        }
    };

    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}
