use convert_case::{Case, Casing};
use sniffer_rs::sniffer::Sniffer;
use whiskers_launcher_rs::{
    action::{Action, CopyAction},
    api::extensions::{send_response, ExtensionRequest},
    result::{TextResult, WLResult},
};

pub fn handle_results(request: ExtensionRequest) {
    let search = request.search_text.unwrap();

    if search.trim().is_empty() {
        send_response(Vec::new());
    }

    let mut results = Vec::<WLResult>::new();
    let sniffer = Sniffer::new();

    for emoji in emojis::iter() {
        if sniffer.clone().matches(emoji.name(), &search) {
            let action = Action::new_copy(CopyAction::new(emoji.as_str()));
            let result = WLResult::new_text(TextResult::new(
                format!(
                    "{} - {}",
                    emoji.to_string(),
                    emoji.name().to_case(Case::Title)
                ),
                action,
            ));

            results.push(result);
        }
    }

    send_response(results);
}
