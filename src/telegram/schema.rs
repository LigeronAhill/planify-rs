use crate::Storage;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dptree::case;
use teloxide::macros::BotCommands;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use crate::models::User;

/// Поддерживаются следующие команды
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Открыть меню
    Start,
    /// Отмена
    Cancel,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    MainMenu,
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn router() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(send_start_menu))
        .branch(case![Command::Cancel].endpoint(cancel));
    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(invalid_state));
    let callback_query_handler =
        Update::filter_callback_query().branch(case![State::MainMenu].endpoint(main_menu));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
async fn send_start_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    storage: Arc<Storage>,
) -> HandlerResult {
    if let Some(user) = msg.from {
        let id = user.id.0 as i64;
        let first_name = user.first_name;
        let last_name = user.last_name;
        let username = user.username;
        let is_bot = user.is_bot;
        let u = crate::models::User {
            id,
            first_name,
            last_name,
            username,
            is_bot,
        };
        if let Err(e) = storage.insert_user(&u).await {
            tracing::warn!("{e:?}");
        }
    }
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let add_task = InlineKeyboardButton::callback("Создать задачу", "add_task");
    keyboard.push(vec![add_task]);
    let my_tasks = InlineKeyboardButton::callback("Мои задачи", "my_tasks");
    let to_me_tasks = InlineKeyboardButton::callback("Задачи мне", "to_me_tasks");
    let tasks = vec![my_tasks, to_me_tasks];
    keyboard.push(tasks);
    let mu = InlineKeyboardMarkup::new(keyboard);
    let input_file = InputFile::file("./images/menu.png");
    bot.send_photo(msg.chat.id, input_file)
        .reply_markup(mu)
        .await?;
    dialogue.update(State::MainMenu).await?;
    Ok(())
}
async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}
async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}
async fn main_menu(bot: Bot, dialogue: MyDialogue, q: CallbackQuery, _storage: Arc<Storage>) -> HandlerResult {
    if let Some(menu_item) = &q.data {
        match menu_item.as_str() {
            "add_task" => {
                bot.send_message(dialogue.chat_id(), "Вы хотите создать задачу")
                    .await?;
            }
            "my_tasks" => {
                let text = format!("{}, Вот задачи, созданные вами:", q.from.first_name);
                bot.send_message(dialogue.chat_id(), text)
                    .await?;
            }
            "to_me_tasks" => {
                bot.send_message(
                    dialogue.chat_id(),
                    "Вы хотите посмотреть задачи, порученные вам",
                )
                .await?;
            }
            _ => {
                dialogue.exit().await?;
            }
        }
    }

    Ok(())
}
