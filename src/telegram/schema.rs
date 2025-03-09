use crate::models::{Task, TaskStatus, User};
use crate::Storage;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dptree::case;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::command::BotCommands;

/// Поддерживаются следующие команды
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Показать справку
    Help,
    /// Открыть меню
    Menu,
    /// Отмена
    Cancel,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    MainMenu,
    AddingTask,
    TasksUpdating,
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn router() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Menu].endpoint(start))
        .branch(case![Command::Cancel].endpoint(cancel));
    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::AddingTask].endpoint(receive_task_name))
        .branch(dptree::endpoint(invalid_state));
    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::MainMenu].endpoint(main_menu))
        .branch(case![State::TasksUpdating].endpoint(update_task));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}
async fn send_menu(bot: Bot, dialogue: MyDialogue, chat_id: ChatId) -> HandlerResult {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let add_task = InlineKeyboardButton::callback("✒️ Создать задачу", "add_task");
    let my_tasks = InlineKeyboardButton::callback("📋 Мои задачи", "my_tasks");
    keyboard.push(vec![add_task, my_tasks]);
    let completed = InlineKeyboardButton::callback("👌 Выполненные задачи", "done_tasks");
    keyboard.push(vec![completed]);
    let mu = InlineKeyboardMarkup::new(keyboard);
    // let input_file = InputFile::file("./images/menu.png");
    bot.send_message(chat_id, "📖 Выберите пункт меню:")
        .reply_markup(mu)
        .await?;
    dialogue.update(State::MainMenu).await?;
    Ok(())
}
async fn start(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    storage: Arc<Storage>,
) -> HandlerResult {
    if let Some(ref user) = msg.from {
        let id = user.id.0 as i64;
        let first_name = user.first_name.clone();
        let last_name = user.last_name.clone();
        let username = user.username.clone();
        let is_bot = user.is_bot;
        let u = User {
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
    send_menu(bot, dialogue, msg.chat.id).await?;
    Ok(())
}
async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Отменяю диалог.").await?;
    dialogue.exit().await?;
    Ok(())
}
async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Не могу обработать сообщение.")
        .await?;
    Ok(())
}
async fn main_menu(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    storage: Arc<Storage>,
) -> HandlerResult {
    if let Some(menu_item) = &q.data {
        let user_id = q.from.id.0 as i64;
        match menu_item.as_str() {
            "add_task" => {
                bot.send_message(dialogue.chat_id(), "Введите название задачи")
                    .await?;
                dialogue.update(State::AddingTask).await?;
            }
            "my_tasks" => {
                let tasks = storage.get_all_tasks(user_id).await?;
                if !tasks.is_empty() {
                    let text = format!("{}, вот задачи, созданные вами:", q.from.first_name);
                    bot.send_message(dialogue.chat_id(), text).await?;
                    for task in tasks {
                        let cb_in_progress = format!("in_progress {id}", id = task.id);
                        let cb_done = format!("done {id}", id = task.id);
                        let buttons = if task.status == TaskStatus::New {
                            vec![
                                InlineKeyboardButton::callback("💼 В работу", cb_in_progress),
                                InlineKeyboardButton::callback("🏆 Выполнена", cb_done),
                            ]
                        } else {
                            vec![
                                InlineKeyboardButton::callback("🏆 Выполнена", cb_done),
                            ]
                            
                        };
                        let kb = InlineKeyboardMarkup::new(vec![buttons]);
                        bot.send_message(dialogue.chat_id(), task.print())
                            .reply_markup(kb)
                            .await?;
                    }
                    dialogue.update(State::TasksUpdating).await?;
                } else {
                    bot.send_message(dialogue.chat_id(), "🍹 Все задачи выполнены")
                        .await?;
                    send_menu(bot, dialogue.clone(), dialogue.chat_id()).await?;
                }
            }
            "done_tasks" => {
                let tasks = storage.get_completed_tasks(user_id).await?;
                let text = format!("{}, вот ваши завершенные задачи:", q.from.first_name);
                bot.send_message(dialogue.chat_id(), text).await?;
                for task in tasks {
                    let cb_in_progress = format!("in_progress {id}", id = task.id);
                    let buttons = vec![InlineKeyboardButton::callback(
                        "💼 Вернуть в работу",
                        cb_in_progress,
                    )];
                    let kb = InlineKeyboardMarkup::new(vec![buttons]);
                    bot.send_message(dialogue.chat_id(), task.print())
                        .reply_markup(kb)
                        .await?;
                }
                dialogue.update(State::TasksUpdating).await?;
            }
            _ => {
                dialogue.exit().await?;
            }
        }
    }

    Ok(())
}
async fn receive_task_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    storage: Arc<Storage>,
) -> HandlerResult {
    match msg.clone().text() {
        None => {
            bot.send_message(msg.chat.id, "Пришлите мне, пожалуйста, название задачи")
                .await?;
        }
        Some(task_name) => {
            let user_id = msg.from.clone().map(|u| u.id.0 as i64).unwrap_or_default();
            let task = Task::new(user_id, task_name.to_string());
            let created = storage.create_task(&task).await?;
            let answer = format!("🎯 Создана\n{}", created.print());
            bot.send_message(msg.chat.id, answer).await?;
            send_menu(bot, dialogue, msg.chat.id).await?;
        }
    }
    Ok(())
}
async fn update_task(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    storage: Arc<Storage>,
) -> HandlerResult {
    if let Some(menu_item) = &q.data {
        let task_id = menu_item
            .split_whitespace()
            .nth(1)
            .and_then(|id| id.parse::<i64>().ok())
            .unwrap_or_default();
        let status = if menu_item.starts_with("done") {
            TaskStatus::Done
        } else {
            TaskStatus::InProgress
        };
        let user_id = q.from.id.0 as i64;
        let updated = storage.update_task_status(task_id, status.clone(), user_id).await?;
        let text = format!(
            "Статус задачи с id: '{id}' изменен на '{status}'",
            id = updated.id,
        );
        bot.send_message(dialogue.chat_id(), text).await?;
        send_menu(bot, dialogue.clone(), dialogue.chat_id()).await?;
    }
    Ok(())
}
