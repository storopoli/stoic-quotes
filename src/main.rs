use prest::*;

use stoic_quotes::{app, into_page};

embed_build_output_as!(BuiltAssets);

fn main() {
    app().wrap_non_htmx(into_page).embed(BuiltAssets).run();
}
