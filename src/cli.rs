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
        .arg(
            Arg::with_name("max_depth")
                .short("md")
                .value_name("MAX_DEPTH")
                .required(false)
                .help("max ray recursion depth (>=1)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("image_width")
                .short("w")
                .value_name("IMAGE_WIDTH")
                .required(false)
                .help("image width (pixels)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .value_name("OUTPUT")
                .required(false)
                .help("output file path")
                .takes_value(true),
        )
}

pub(crate) struct RConfig {
    pub sample_per_pixel: usize,
    pub max_depth: usize,
    pub image_width: usize,
    pub output_file_path: String,
}

impl Default for RConfig {
    fn default() -> Self {
        RConfig {
            sample_per_pixel: 1,
            max_depth: 10,
            image_width: 128,
            output_file_path: String::from("out.png"),
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

    pub(crate) fn with_max_depth(self, max_depth: usize) -> anyhow::Result<Self> {
        if max_depth != 0 {
            Ok(RConfig { max_depth, ..self })
        } else {
            Err(anyhow::anyhow!("max depth should be >= 1"))
        }
    }

    pub(crate) fn with_image_width(self, image_width: usize) -> anyhow::Result<Self> {
        if image_width != 0 {
            Ok(RConfig {
                image_width,
                ..self
            })
        } else {
            Err(anyhow::anyhow!("image width should be >= 1"))
        }
    }

    pub(crate) fn with_output_file_path(self, output_file_path: String) -> anyhow::Result<Self> {
        Ok(RConfig {
            output_file_path,
            ..self
        })
    }

    pub(crate) fn from_matches(matches: clap::ArgMatches) -> anyhow::Result<Self> {
        let config = RConfig::default();
        let config = if let Some(spp) = matches.value_of("sample_per_pixel") {
            let spp = spp.parse::<usize>()?;
            config.with_sample_per_pixel(spp)?
        } else {
            config
        };
        let config = if let Some(max_depth) = matches.value_of("max_depth") {
            let max_depth = max_depth.parse::<usize>()?;
            config.with_max_depth(max_depth)?
        } else {
            config
        };
        let config = if let Some(image_width) = matches.value_of("image_width") {
            let image_width = image_width.parse::<usize>()?;
            config.with_image_width(image_width)?
        } else {
            config
        };
        let config = if let Some(output_file_path) = matches.value_of("output") {
            config.with_output_file_path(String::from(output_file_path))?
        } else {
            config
        };
        Ok(config)
    }
}
