use crates_io_api::{AsyncClient, Crate, CratesQuery};
use std::error::Error;
use teloxide::{prelude::*, types::*};

use crate::utils::inlines::*;

pub async fn inline(
    bot: Bot,
    crates_client: AsyncClient,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if q.query.is_empty() {
        return {
            bot.answer_inline_query(
                q.id,
                vec![InlineQueryResultArticle::new(
                    "101",
                    "Start searching!",
                    InputMessageContent::Text(
                        InputMessageContentText::new(NO_INPUT)
                            .parse_mode(ParseMode::Html)
                            .disable_web_page_preview(true),
                    ),
                )
                .reply_markup(err_keyboard())
                .into()],
            )
            .await?;
            Ok(())
        };
    }

    let request: CratesQuery = CratesQuery::builder()
        .search(q.query.clone())
        .page(1)
        .page_size(50)
        .build();

    let request: Vec<Crate> = crates_client.crates(request).await.unwrap().crates;

    if request.is_empty() {
        return {
            bot.answer_inline_query(
                q.id,
                vec![InlineQueryResultArticle::new(
                    "404",
                    "Couldn't find!",
                    InputMessageContent::Text(
                        InputMessageContentText::new(
                            format!("<b>There are no results related to {}!</b>\nPlease, Try to search with other names or parameters!", 
                            q.query.clone())
                        )
                            .parse_mode(ParseMode::Html)
                            .disable_web_page_preview(true),
                    ),
                )
                    .reply_markup(err_keyboard())
                .into()],
            )
            .await?;
            Ok(())
        };
    }

    let request: Vec<InlineQueryResult> = request
        .iter()
        .map(|c: &Crate| {
            InlineQueryResult::Article(
                InlineQueryResultArticle::new(
                    uuid::Uuid::new_v4(),
                    c.name.clone(),
                    InputMessageContent::Text(
                        InputMessageContentText::new(view_generate(c))
                            .parse_mode(ParseMode::Html)
                            .disable_web_page_preview(true),
                    ),
                )
                .description(c.description.clone().unwrap())
                .url(url::Url::parse(&format!("https://crates.io/crates/{}", c.id)).unwrap())
                .reply_markup(kb_generate(c)),
            )
        })
        .collect();

    bot.answer_inline_query(q.id, request).send().await?;
    Ok(())
}
