use results::handle_results;
use whiskers_launcher_rs::api::extensions::get_extension_request;

pub mod results;


fn main() {
    let request = get_extension_request();
    
    handle_results(request);
}
