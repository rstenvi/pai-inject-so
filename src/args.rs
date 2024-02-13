use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Default)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum Flags {
	RTLD_LAZY = libc::RTLD_LAZY,
	#[default]
	RTLD_NOW = libc::RTLD_NOW,
}
impl std::fmt::Display for Flags {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{:?}", self))
	}
}
impl std::str::FromStr for Flags {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"RTLD_LAZY" => Ok(Self::RTLD_LAZY),
			"RTLD_NOW" => Ok(Self::RTLD_NOW),
			_ => Err(anyhow::Error::msg("flag not found")),
		}
	}
}

#[derive(clap::Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[command(flatten)]
	pub verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::WarnLevel>,

	/// Attach to the argument, as opposed to spawning it
	#[arg(short, long)]
	pub attach: bool,

	/// Check if a new version is available when starting
	#[arg(long)]
	pub check_update: bool,

	/// The SO-file to inject
	#[arg(long, short)]
	pub inject: PathBuf,

	#[arg(long, short, default_value_t = Flags::default())]
	pub flags: Flags,

	#[arg(long, short)]
	pub r#override: Vec<String>,

	#[arg(long)]
	pub listen: Option<std::net::SocketAddr>,

	/// Program to attach to, program to start, etc
	#[arg(trailing_var_arg = true, allow_hyphen_values = true)]
	pub args: Vec<String>,
}
