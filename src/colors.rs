//! Подсветка текста в сообщение с разделением
//! # Функции
//! - green_text_and_msg()
//! - red_text_and_msg()
//! - yellow_text_and_msg()
//! - blue_text_and_msg()

///  Возвращает TEXT | message
///  > Цвет текста - зеленый
///  # Пример
/// ```
/// let green_text_and_msg("DONE", "Good work!")
/// ```
pub fn green_text_and_msg(text: &str, msg: &str) -> String {
    return String::from("[ \x1b[32m{text}\x1b[0m ] {msg}");
}

///  Возвращает Text | message
///
///  # Пример
/// ```
/// let red_text_and_msg("DONE", "Good work!")
/// ```
pub fn red_text_and_msg(text: &str, msg: &str) -> String {
    return String::from(format!("[ \x1b[31m{}\x1b[0m ] {}", text, msg));
}
pub fn yellow_text_and_msg(text: &str, msg: &str) -> String {
    return String::from(format!("[ \x1b[33m{}\x1b[0m ] {}", text, msg));
}
pub fn blue_text_and_msg(text: &str, msg: &str) -> String {
    return String::from(format!("[ \x1b[34m{}\x1b[0m ] {}", text, msg));
}
