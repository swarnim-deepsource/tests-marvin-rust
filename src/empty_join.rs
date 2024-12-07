#[rustfmt::skip]
fn f() {
          let m = ["test", "ing"].join("");
     let array = ["tt", "tt"];
          array.join("");
}

fn t(sl: &[&str]) -> String {
    sl.join("")
}

fn no_match() {
    let _ = ["test", "ing"].join(" ");
    let _ = ["test", "ing"].join("-");
    let _ = ["test", "ing"].concat();
    let a = ["test", "ing"];
    a.join("*");
}
