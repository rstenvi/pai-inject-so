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

	/// Check for update when starting and exit if this is not the newest
	/// version.
	#[arg(long, verbatim_doc_comment)]
	pub check_update: bool,

	/// The SO-file to inject
	#[arg(long, short)]
	pub inject: Option<PathBuf>,

	/// Flags to pass to `dlopen`
	#[arg(long, short, default_value_t = Flags::default())]
	pub flags: Flags,

	/// For each function provided, we will override the GOT entry so that the
	/// function with the same in the SO-file will be executed in place of the
	/// original function.
	#[arg(long, short, verbatim_doc_comment)]
	pub r#override: Vec<String>,

	/// Program will try and resolve dlopen in expected modules, but it may not
	/// work correctly, use this option to override that behaviour.
	#[arg(long, verbatim_doc_comment)]
	pub dlpath: Option<PathBuf>,

	// Unstable
	// #[arg(long)]
	// pub listen: Option<std::net::SocketAddr>,
	/// Program to attach to or program to start. When spawning, extra arguments
	/// will be forwarded to spawned program.
	#[arg(
		trailing_var_arg = true,
		allow_hyphen_values = true,
		verbatim_doc_comment
	)]
	pub args: Vec<String>,
}

impl Args {
	pub fn sanity_check(&self) -> anyhow::Result<()> {
		if self.check_update {
			if let Ok(Some(version)) = check_latest::check_max!() {
				let msg = format!("version {version} is now available!");
				log::warn!("{msg}");
				log::warn!("update with 'cargo install --force pai-inject-so'");
				return Err(anyhow::Error::msg(msg));
			} else {
				log::debug!("already running newest version");
			}
		}
		if self.args.is_empty() {
			let msg = "need to pass a program to attach to or spawn";
			log::warn!("{msg}");
			return Err(anyhow::Error::msg(msg));
		}
		if self.inject.is_none() {
			let msg = "no file passed in --inject";
			log::warn!("{msg}");
			return Err(anyhow::Error::msg(msg));
		}
		Ok(())
	}
}
