use whiskers_launcher_rs::{
    actions::{Action, CopyToClipboard},
    api::extensions::{send_extension_results, Context},
    results::{Text, WhiskersResult},
};

struct EmojiResult {
    pub glyph: String,
    pub name: String,
}

pub fn handle_results(context: Context) {
    let search_text = context.search_text.unwrap();
    let mut results = Vec::<WhiskersResult>::new();

    let emojis = emoji::search::search_annotation(&search_text, "en")
        .into_iter()
        .map(|e| EmojiResult {
            glyph: e.glyph.to_owned(),
            name: e.name.to_owned(),
        });

    for emoji in emojis {
        results.push(WhiskersResult::Text(Text::new(
            format!("{} - {}", emoji.glyph, emoji.name),
            Action::CopyToClipboard(CopyToClipboard::new(emoji.glyph)),
        )));
    }

    send_extension_results(results);
}
