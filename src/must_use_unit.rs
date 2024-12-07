#[rustfmt::skip]
mod must_use_unit {
    type T = ();

      #[must_use] fn a() {}

      #[must_use] pub fn b() {}

      #[must_use] pub fn c() -> T {}

      #[must_use] #[cfg(feature = "publish")] fn d() -> () {}

      #[must_use = "With note"] pub fn must_use_with_note() {}

    #[must_use] pub async fn yield_now() {}
}
