// Source: https://github.com/hashintel/hash/blob/24e82f839bdd2d70758b9676f76f1388e6973241/apps/hash-graph/lib/temporal-versioning/src/serde.rs#L6

use ::time::format_description::well_known;

const CONFIG: well_known::iso8601::EncodedConfig = well_known::iso8601::Config::DEFAULT
    .set_year_is_six_digits(false)
    .encode();
const FORMAT: well_known::Iso8601<CONFIG> = well_known::Iso8601::<CONFIG>;
::time::serde::format_description!(time_format, OffsetDateTime, FORMAT);

// The macro above creates a private macro so we re-export it here publicly.
pub mod iso8601 {
    pub use super::time_format::*;

    pub mod option {
        pub use super::super::time_format::option::*;
    }
}
