use whiskers_launcher_rs::{
    action::{Action, CopyAction},
    api::extensions::{send_response, ExtensionRequest},
    result::{TextResult, WLResult},
    utils::get_search,
};

struct EmojiResult {
    pub glyph: String,
    pub name: String,
}

pub fn handle_results(request: ExtensionRequest) {
    let search = get_search(request.search_text.unwrap());

    if search.search_text.is_empty() {
        send_response(Vec::new());
    }

    let mut results = Vec::<WLResult>::new();

    let emojis = emoji::search::search_annotation(&search.search_text, "en")
        .into_iter()
        .map(|e| EmojiResult {
            glyph: e.glyph.to_owned(),
            name: e.name.to_owned(),
        });

    for emoji in emojis {
        let action = Action::new_copy(CopyAction::new(&emoji.glyph));
        let result = WLResult::new_text(TextResult::new(
            format!("{} - {}", &emoji.glyph, &emoji.name),
            action,
        ));

        results.push(result);
    }

    send_response(results);
}
