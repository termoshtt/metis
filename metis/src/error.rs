//! Errors in metis crate

use metis_sys::rstatus_et;
use num_traits::FromPrimitive;

use crate::io::InvalidGraphFileError;

const ISSUE_URL: &'static str = "https://github.com/termoshtt/metis/issues";

pub type Result<T> = ::std::result::Result<T, Error>;

/// Aggregated error type of metis crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("METIS routine ({api_name}) cannot allocate required memory")]
    MemoryCannotAllocate { api_name: String },

    #[error(transparent)]
    InvalidGraphFile(#[from] InvalidGraphFileError),
}

pub(crate) trait MetisErrorCodeCheck {
    fn check(self, api_name: &str) -> Result<()>;
}

impl MetisErrorCodeCheck for i32 {
    fn check(self, api_name: &str) -> Result<()> {
        let st = rstatus_et::from_i32(self).unwrap_or_else(|| {
            panic!(
                "Invalid return value ({}) of METIS routine ({}). This should be a bug of METIS. Please send a bug report to {}",
                self, api_name, ISSUE_URL
            )
        });
        match st {
            rstatus_et::METIS_OK => Ok(()),
            rstatus_et::METIS_ERROR_MEMORY => Err(Error::MemoryCannotAllocate {
                api_name: api_name.into(),
            }),

            // Following two cases must be bug of this crate, and cannot be recoverted by user.
            rstatus_et::METIS_ERROR_INPUT => panic!(
                "METIS routine ({}) raises METIS_ERROR_INPUT, which must be handled by metis crate. Please send a bug report to {}",
                api_name,
                ISSUE_URL,
            ),
            rstatus_et::METIS_ERROR => panic!(
                "METIS routine ({}) raises unknown error. Please send a bug report to {}",
                api_name,
                ISSUE_URL,
            ),
        }
    }
}
