use metis_sys::rstatus_et;
use num_traits::FromPrimitive;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("METIS routine ({api_name}) cannot allocate required memory")]
    MetisMemoryError { api_name: String },
}

pub(crate) trait MetisErrorCodeCheck {
    fn check(self, api_name: &str) -> Result<()>;
}

impl MetisErrorCodeCheck for i32 {
    fn check(self, api_name: &str) -> Result<()> {
        let st = rstatus_et::from_i32(self).expect("Failed to convert to METIS status enum");
        match st {
            rstatus_et::METIS_OK => Ok(()),
            rstatus_et::METIS_ERROR_INPUT => panic!(
                "METIS routine ({}) raises METIS_ERROR_INPUT. This must be a bug of metis crate.",
                api_name
            ),
            rstatus_et::METIS_ERROR_MEMORY => Err(Error::MetisMemoryError {
                api_name: api_name.into(),
            }),
            rstatus_et::METIS_ERROR => panic!(
                "METIS routine ({}) raises METIS_ERROR. Something wrong...",
                api_name
            ),
        }
    }
}
