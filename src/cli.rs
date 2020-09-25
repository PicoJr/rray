use crate::ray::RT;
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
            Arg::with_name("vertical_fov")
                .long("vfov")
                .value_name("VERTICAL_FOV")
                .required(false)
                .help("vertical fov (degrees)")
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
        .arg(
            Arg::with_name("parallel")
                .long("parallel")
                .required(false)
                .help("enable multi-threading"),
        )
}

pub(crate) struct RConfig {
    pub sample_per_pixel: usize,
    pub max_depth: usize,
    pub image_width: usize,
    pub aspect_ratio: RT,
    pub output_file_path: String,
    pub vfov: RT,
    pub parallel: bool,
}

impl Default for RConfig {
    fn default() -> Self {
        RConfig {
            sample_per_pixel: 1,
            max_depth: 10,
            image_width: 128,
            aspect_ratio: 16.0 / 9.0,
            output_file_path: String::from("out.png"),
            vfov: 90.,
            parallel: false,
        }
    }
}

impl RConfig {
    pub(crate) fn get_image_height(&self) -> u32 {
        (self.image_width as RT / self.aspect_ratio) as u32
    }

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

    pub(crate) fn with_vertical_fov(self, vertical_fov: RT) -> anyhow::Result<Self> {
        if vertical_fov > 0. {
            Ok(RConfig {
                vfov: vertical_fov,
                ..self
            })
        } else {
            Err(anyhow::anyhow!("vertical fov should be > 0"))
        }
    }

    pub(crate) fn with_parallel(self, parallel: bool) -> anyhow::Result<Self> {
        Ok(RConfig { parallel, ..self })
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
        let config = if let Some(vertical_fov) = matches.value_of("vertical_fov") {
            let vertical_fov = vertical_fov.parse::<RT>()?;
            config.with_vertical_fov(vertical_fov)?
        } else {
            config
        };
        let config = if matches.is_present("parallel") {
            config.with_parallel(true)?
        } else {
            config
        };
        Ok(config)
    }
}
