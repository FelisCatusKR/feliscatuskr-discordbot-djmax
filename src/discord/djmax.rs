use serenity::{
    builder::CreateMessage,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

use crate::db::{establish_connection, models::Song};

fn send_simple_message(ctx: &mut Context, msg: &Message, text: &str) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, &text) {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
#[aliases("search", "st", "s", "곡검색")]
#[bucket = "djmax"]
pub fn search_by_title(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() < 1 {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n사용법: `search_by_title (곡 제목의 일부)`",
        );
    }

    let conn = establish_connection();
    let original_query = str::replace(args.message(), "%", "");
    let query = str::replace(original_query.as_str(), " ", "%");
    let contents = Song::by_title(query.as_str(), &conn);

    if let Some(content) = contents {
        // For debug purpose
        #[cfg(debug_assertions)]
        println!("{:?}", content);

        if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
            m.content(format!("\"{}\" 검색 결과:", original_query.as_str()));
            m.embed(|e| {
                e.title(content.title.as_str());
                e.description(content.artist.as_str());
                let mut bpm_string = String::new();
                if let Some(bpm) = content.min_bpm {
                    bpm_string.push_str(format!("{}~", bpm).as_str());
                }
                bpm_string.push_str(format!("{}", content.max_bpm).as_str());
                e.field("BPM", bpm_string, true);
                e.field("Category", content.category, true);
                if let Some(dlc) = content.dlc {
                    e.field("DLC", dlc, true);
                }
                e
            });
            m
        }) {
            println!("Error sending message: {:?}", why);
        }

        Ok(())
    } else {
        send_simple_message(ctx, msg, "검색 결과가 없습니다.")
    }
}

#[command]
#[aliases(
    "4blv",
    "4lv",
    "4l",
    "4",
    "4버튼레벨",
    "4키레벨",
    "4버튼렙",
    "4키렙",
    "4버튼",
    "4키"
)]
#[bucket = "djmax"]
pub fn search_by_4b_level(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() < 1 || args.len() > 2 {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n사용법: `search_by_4b_level 레벨 (페이지)`",
        );
    }
    let arg = args.single::<i32>();
    if let Err(e) = arg {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n레벨은 1 이상 15 이하의 정수여야 합니다.`",
        );
    }
    let level = arg.unwrap();
    if level > 15 || level < 1 {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n레벨은 1 이상 15 이하의 정수여야 합니다.",
        );
    }

    let mut page: i64 = 1;
    if !args.is_empty() {
        let arg = args.single::<i64>();
        if let Err(e) = arg {
            return send_simple_message(
                ctx,
                msg,
                "잘못된 사용법입니다.\n페이지는 1 이상의 정수여야 합니다.`",
            );
        }
        page = arg.unwrap();
        if page <= 0 {
            return send_simple_message(
                ctx,
                msg,
                "잘못된 사용법입니다.\n페이지는 1 이상의 정수여야 합니다.",
            );
        }
    }

    let conn = establish_connection();
    let (count, contents) = Song::by_level(4, level, 25 * (page - 1), &conn);
    if count == 0 {
        return send_simple_message(ctx, msg, "검색 결과가 없습니다.");
    }

    // For debug purpose
    #[cfg(debug_assertions)]
    for song in &contents {
        println!("{:?}", song);
    }

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(&format!("{}버튼 {}레벨 검색 결과:", 4, level));
            e.description(&format!(
                "총 {}곡이 발견되었습니다. ({} / {} 페이지)",
                contents.len(),
                page,
                count / 25 + 1
            ));
            let mut fields = Vec::new();
            for content in contents {
                let mut pattern = String::new();
                if level == content.four_button_0 {
                    pattern.push_str("NM ");
                }
                if Some(level) == content.four_button_1 {
                    pattern.push_str("HD ");
                }
                if Some(level) == content.four_button_2 {
                    pattern.push_str("MX ");
                }
                if Some(level) == content.four_button_3 {
                    pattern.push_str("SC ");
                }
                fields.push((content.title, pattern, true));
            }
            e.fields(fields);
            e
        });
        m
    }) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[aliases(
    "5blv",
    "5lv",
    "5l",
    "5",
    "5버튼레벨",
    "5키레벨",
    "5버튼렙",
    "5키렙",
    "5버튼",
    "5키"
)]
#[bucket = "djmax"]
pub fn search_by_5b_level(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() < 1 || args.len() > 2 {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n사용법: `search_by_5b_level 레벨 (페이지)`",
        );
    }
    let arg = args.single::<i32>();
    if let Err(e) = arg {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n레벨은 1 이상 15 이하의 정수여야 합니다.`",
        );
    }
    let level = arg.unwrap();
    if level > 15 || level < 1 {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n레벨은 1 이상 15 이하의 정수여야 합니다.",
        );
    }

    let mut page: i64 = 1;
    if !args.is_empty() {
        let arg = args.single::<i64>();
        if let Err(e) = arg {
            return send_simple_message(
                ctx,
                msg,
                "잘못된 사용법입니다.\n페이지는 1 이상의 정수여야 합니다.`",
            );
        }
        page = arg.unwrap();
        if page <= 0 {
            return send_simple_message(
                ctx,
                msg,
                "잘못된 사용법입니다.\n페이지는 1 이상의 정수여야 합니다.",
            );
        }
    }

    let conn = establish_connection();
    let (count, contents) = Song::by_level(5, level, 25 * (page - 1), &conn);
    if count == 0 {
        return send_simple_message(ctx, msg, "검색 결과가 없습니다.");
    }

    // For debug purpose
    #[cfg(debug_assertions)]
    for song in &contents {
        println!("{:?}", song);
    }

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(&format!("{}버튼 {}레벨 검색 결과:", 5, level));
            e.description(&format!(
                "총 {}곡이 발견되었습니다. ({} / {} 페이지)",
                count,
                page,
                count / 25 + 1
            ));
            let mut fields = Vec::new();
            for content in contents {
                let mut pattern = String::new();
                if level == content.five_button_0 {
                    pattern.push_str("NM ");
                }
                if Some(level) == content.five_button_1 {
                    pattern.push_str("HD ");
                }
                if Some(level) == content.five_button_2 {
                    pattern.push_str("MX ");
                }
                if Some(level) == content.five_button_3 {
                    pattern.push_str("SC ");
                }
                fields.push((content.title, pattern, true));
            }
            e.fields(fields);
            e
        });
        m
    }) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[aliases(
    "6blv",
    "6lv",
    "6l",
    "6",
    "6버튼레벨",
    "6키레벨",
    "6버튼렙",
    "6키렙",
    "6버튼",
    "6키"
)]
#[bucket = "djmax"]
pub fn search_by_6b_level(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() < 1 || args.len() > 2 {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n사용법: `search_by_6b_level 레벨 (페이지)`",
        );
    }
    let arg = args.single::<i32>();
    if let Err(e) = arg {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n레벨은 1 이상 15 이하의 정수여야 합니다.`",
        );
    }
    let level = arg.unwrap();
    if level > 15 || level < 1 {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n레벨은 1 이상 15 이하의 정수여야 합니다.",
        );
    }

    let mut page: i64 = 1;
    if !args.is_empty() {
        let arg = args.single::<i64>();
        if let Err(e) = arg {
            return send_simple_message(
                ctx,
                msg,
                "잘못된 사용법입니다.\n페이지는 1 이상의 정수여야 합니다.`",
            );
        }
        page = arg.unwrap();
        if page <= 0 {
            return send_simple_message(
                ctx,
                msg,
                "잘못된 사용법입니다.\n페이지는 1 이상의 정수여야 합니다.",
            );
        }
    }

    let conn = establish_connection();
    let (count, contents) = Song::by_level(6, level, 25 * (page - 1), &conn);
    if count == 0 {
        return send_simple_message(ctx, msg, "검색 결과가 없습니다.");
    }

    // For debug purpose
    #[cfg(debug_assertions)]
    for song in &contents {
        println!("{:?}", song);
    }

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(&format!("{}버튼 {}레벨 검색 결과:", 6, level));
            e.description(&format!(
                "총 {}곡이 발견되었습니다. ({} / {} 페이지)",
                count,
                page,
                count / 25 + 1
            ));
            let mut fields = Vec::new();
            for content in contents {
                let mut pattern = String::new();
                if level == content.six_button_0 {
                    pattern.push_str("NM ");
                }
                if Some(level) == content.six_button_1 {
                    pattern.push_str("HD ");
                }
                if Some(level) == content.six_button_2 {
                    pattern.push_str("MX ");
                }
                if Some(level) == content.six_button_3 {
                    pattern.push_str("SC ");
                }
                fields.push((content.title, pattern, true));
            }
            e.fields(fields);
            e
        });
        m
    }) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[aliases(
    "8blv",
    "8lv",
    "8l",
    "8",
    "8버튼레벨",
    "8키레벨",
    "8버튼렙",
    "8키렙",
    "8버튼",
    "8키"
)]
#[bucket = "djmax"]
pub fn search_by_8b_level(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() < 1 || args.len() > 2 {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n사용법: `search_by_8b_level 레벨 (페이지)`",
        );
    }
    let arg = args.single::<i32>();
    if let Err(e) = arg {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n레벨은 1 이상 15 이하의 정수여야 합니다.`",
        );
    }
    let level = arg.unwrap();
    if level > 15 || level < 1 {
        return send_simple_message(
            ctx,
            msg,
            "잘못된 사용법입니다.\n레벨은 1 이상 15 이하의 정수여야 합니다.",
        );
    }

    let mut page: i64 = 1;
    if !args.is_empty() {
        let arg = args.single::<i64>();
        if let Err(e) = arg {
            return send_simple_message(
                ctx,
                msg,
                "잘못된 사용법입니다.\n페이지는 1 이상의 정수여야 합니다.`",
            );
        }
        page = arg.unwrap();
        if page <= 0 {
            return send_simple_message(
                ctx,
                msg,
                "잘못된 사용법입니다.\n페이지는 1 이상의 정수여야 합니다.",
            );
        }
    }

    let conn = establish_connection();
    let (count, contents) = Song::by_level(8, level, 25 * (page - 1), &conn);
    if count == 0 {
        return send_simple_message(ctx, msg, "검색 결과가 없습니다.");
    }

    // For debug purpose
    #[cfg(debug_assertions)]
    for song in &contents {
        println!("{:?}", song);
    }

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(&format!("{}버튼 {}레벨 검색 결과:", 8, level));
            e.description(&format!(
                "총 {}곡이 발견되었습니다. ({} / {} 페이지)",
                count,
                page,
                count / 25 + 1
            ));
            let mut fields = Vec::new();
            for content in contents {
                let mut pattern = String::new();
                if level == content.eight_button_0 {
                    pattern.push_str("NM ");
                }
                if Some(level) == content.eight_button_1 {
                    pattern.push_str("HD ");
                }
                if Some(level) == content.eight_button_2 {
                    pattern.push_str("MX ");
                }
                if Some(level) == content.eight_button_3 {
                    pattern.push_str("SC ");
                }
                fields.push((content.title, pattern, true));
            }
            e.fields(fields);
            e
        });
        m
    }) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
