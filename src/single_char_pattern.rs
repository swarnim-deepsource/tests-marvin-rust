mod tests {
    use std::collections::HashSet;
    fn trivial() {
        let x = "foo";
        x.split("x");
        x.split("xx");
        x.split('x');
    }

    fn all_methods() {
        let x = "foo";
        x.split_inclusive("x");
        x.contains("x");
        x.starts_with("x");
        x.ends_with("x");
        x.find("x");
        x.rfind("x");
        x.rsplit("x");
        x.split_terminator("x");
        x.rsplit_terminator("x");
        x.splitn(3, "x");
        x.rsplitn(3, "x");
        x.split_once("x");
        x.rsplit_once("x");
        x.matches("x");
        x.rmatches("x");
        x.match_indices("x");
        x.rmatch_indices("x");
        x.trim_start_matches("x");
        x.trim_end_matches("x");
        x.strip_prefix("x");
        x.strip_suffix("x");
        x.replace("x", "y");
        x.replacen("x", "y", 3);
        x.split("\t");
        x.split("'");
        x.split("\'");
    }

    fn no_match() {
        let h = HashSet::<String>::new();
        h.contains("X");
    }

    fn raw_strings() {
        let x = "foo";
        x.split(r"a");
        x.split(r#"a"#);
        x.split(r###"a"###);
        x.split(r###"'"###);
        x.split(r###"#"###);
    }
}
