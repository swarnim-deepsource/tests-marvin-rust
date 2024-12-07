use std::borrow::Cow;

use actix_web_lab::web as web_lab;

fn redir_fn(dest: String, other_dest: String) -> web_lab::Redirect {
    web_lab::redirect("Hello", dest)
}

fn redir_struct_method(dest: &str) -> web_lab::Redirect {
    // TODO(prajwal): fix this [RS-S1009]
    web_lab::Redirect::new("Hello", dest.to_string())
}

fn redir_with_cast(dest: &str) -> web_lab::Redirect {
    // TODO(prajwal): fix this [RS-S1009]
    web_lab::redirect(Cow::from("Hello"), dest.to_string())
}

fn no_match_on_field(dest_tuple: (String, usize)) -> web_lab::Redirect {
    // Should not raise on this call
    web_lab::redirect(Cow::from("Hello"), dest_tuple.0)
}

fn valid(src: impl AsRef<str>) -> bool {
    false
}

fn no_match_used_var(dest: String) -> Option<web_lab::Redirect> {
    if !valid(&dest) {
        return None;
    };
    // Should not raise on this call
    Some(web_lab::redirect("Hello", dest))
}
