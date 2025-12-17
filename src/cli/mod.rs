use clap::{Args, Parser, Subcommand, command};

#[derive(Debug, Parser)]
#[command(
    name = "blast",
    version,
    about = "CLI для создания и запуска Rust/React проектов на основе Blast фреймворка",
    long_about = "blast — это инструмент командной строки для создания приложений и проектов на Rust и React,\n\
                  а также для их запуска в режиме разработки или тестирования."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Создать новое приложение или проект")]
    Create(CreateCommand),

    #[command(about = "Запустить сервер разработки или тестовый запуск")]
    Start(StartArgs),
}

#[derive(Debug, Subcommand)]
pub enum CreateKind {
    #[command(about = "Создать новое приложение (одиночный сервис или фронтенд)")]
    App,

    #[command(about = "Создать новый проект (монорепозиторий или набор сервисов)")]
    Project,
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    #[command(subcommand)]
    pub kind: CreateKind,

    #[arg(
        long,
        help = "Добавить в создаваемый объект административный модуль (панель администратора, роли, права доступа)",
        long_help = "Если указан флаг --admin, при создании приложения или проекта будет сгенерирована базовая\n\
                     административная инфраструктура: панель администратора, роли пользователей и права доступа."
    )]
    pub admin: bool,
}

#[derive(Debug, Args)]
pub struct StartArgs {
    #[arg(
        long,
        short,
        default_value_t = 8000,
        help = "Порт для запуска сервера",
        long_help = "Порт, на котором будет запущен сервер разработки или тестовый сервер.\n\
                     По умолчанию используется порт 8000."
    )]
    pub port: u16,

    #[arg(
        long,
        short,
        help = "Включить отладочный режим (подробные логи)",
        long_help = "Если указан флаг --debug, сервер будет запускаться в отладочном режиме\n\
                     с более подробным логированием и дополнительной диагностикой."
    )]
    pub debug: bool,

    #[arg(
        long,
        short,
        help = "Запустить сервер в тестовом режиме",
        long_help = "Если указан флаг --test, сервер будет запущен в тестовом режиме.\n\
                     Это может означать использование тестовой базы данных, фикстур или особых настроек окружения."
    )]
    pub test: bool,
}
