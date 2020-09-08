use clap::{App, Arg};

pub(crate) fn get_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author("PicoJr")
        .about("Ray Tracer")
        .arg(
            Arg::with_name("sample_per_pixel")
                .short("spp")
                .value_name("SPP")
                .required(false)
                .help("sample per pixel (>=1)")
                .takes_value(true),
        )
}

pub(crate) struct RConfig {
    pub sample_per_pixel: usize,
}

impl Default for RConfig {
    fn default() -> Self {
        RConfig {
            sample_per_pixel: 1,
        }
    }
}

impl RConfig {
    pub(crate) fn with_sample_per_pixel(self, sample_per_pixel: usize) -> anyhow::Result<Self> {
        if sample_per_pixel != 0 {
            Ok(RConfig {
                sample_per_pixel,
                ..self
            })
        } else {
            Err(anyhow::anyhow!("sample per pixel should be >= 1"))
        }
    }

    pub(crate) fn from_matches(matches: clap::ArgMatches) -> anyhow::Result<Self> {
        let config = RConfig::default();
        let config = if let Some(spp) = matches.value_of("spp") {
            let spp = spp.parse::<usize>()?;
            config.with_sample_per_pixel(spp)
        } else {
            Ok(config)
        };
        config
    }
}
