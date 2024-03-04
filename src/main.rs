use crate::args::Args;
use anyhow::Result;
use clap::Parser;
use pai::ctx;

pub mod args;

fn main() -> Result<()> {
	let mut args = Args::parse();
	pretty_env_logger::formatted_builder()
		.filter_level(args.verbose.log_level_filter())
		.init();
	log::info!("starting");
	args.sanity_check()?;

	// Set up our instance
	let mut cargs = std::mem::take(&mut args.args);
	let prog = cargs.remove(0);
	let mut ctx: ctx::Main<(), anyhow::Error> = ctx::Main::new_main(args.attach, prog, cargs, ())?;
	let sec = ctx.secondary_mut();

	// If we spawned the program, let it run until entry so that libraries
	// (including libdl) are loaded.
	if !args.attach {
		sec.run_until_entry()?;
	}

	// Get the canonical path
	let inject = args.inject.expect("No SO-file passed in --inject");
	let injectp = inject.canonicalize()?;
	let inject = injectp
		.clone()
		.into_os_string()
		.into_string()
		.expect("unable to convert inject parameter to string");

	// Get a tid we can interact with and resolve dlopen
	let tid = sec.get_first_stopped()?;
	let dlopen = sec.lookup_symbol_in_any("dlopen")?.expect("unable to find dlopen");
	log::info!("found dlopen @ {:x}", dlopen.value);

	// Need to write our string to memory
	let addr = sec.client_mut().write_scratch_string(tid, &inject)?;

	// Construct our function call
	let dlargs = vec![addr, (args.flags as i32).into()];
	log::info!("calling dlopen({:x}, {})", dlargs[0], dlargs[1]);
	let v = sec.call_func(tid, dlopen.value, &dlargs)?;

	// Verify that handle is valid
	log::info!("returned handle: {v:x}");
	if usize::from(v) == 0 {
		let msg = format!("dlopen returned 0, '{}' not loaded", inject);
		log::error!("{msg}");
		return Err(anyhow::Error::msg(msg));
	} else {
		log::info!("library succecssfully loaded");
	}

	// We also verify by finding the location of our loaded module
	let loc = sec
		.proc
		.exact_match_path(&injectp)?
		.expect("unable to verify that module was loaded");
	log::info!("module loaded @ {loc:?}");

	// Override all GOT entries the user specified
	let exe = sec.proc.exe_path()?;
	for hook in args.r#override.into_iter() {
		log::info!("hooking fucntion {hook}");
		let fake = sec
			.resolve_symbol_in_mod(&injectp, &hook)?
			.expect("unable to find matching function in SO-file");
		log::info!("hooking '{hook}' with addr: {:x}", fake.value);
		sec.overwrite_got_symbol(tid, &exe, &hook, fake.value)?;
	}

	// Free up the memory we allocated and detach
	sec.client_mut().free_scratch_addr(tid, addr)?;
	ctx.detach()?;
	Ok(())
}
