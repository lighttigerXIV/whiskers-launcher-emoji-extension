//REQUIRED FOR WINDOWS EXTENSIONS
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use convert_case::{Case, Casing};
use sniffer_rs::sniffer::Sniffer;
use whiskers_launcher_core::{
    features::{core::extensions::get_extension_request, extensions::send_search_results},
    results::{CopyTextAction, ResultAction, SearchResult, SearchResults},
};

fn main() {
    let request = get_extension_request();

    let search = request.search_text.unwrap();

    if search.is_empty() {
        send_search_results(SearchResults::new_list_results(vec![]));
    }

    let mut results = Vec::<SearchResult>::new();
    let sniffer = Sniffer::new();

    for emoji in emojis::iter() {
        if sniffer.matches(emoji.name(), &search) {
            let result = SearchResult::new(
                format!(
                    "{} - {}",
                    emoji.to_string(),
                    emoji.name().to_case(Case::Title)
                ),
                ResultAction::new_copy_text_action(CopyTextAction::new(emoji.as_str())),
            );

            results.push(result);
        }
    }

    send_search_results(SearchResults::new_list_results(results));
}
