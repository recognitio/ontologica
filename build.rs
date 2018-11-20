fn main() {}

/*
Workaround to recompile when non-.rs files are changed - /u/mbrubeck:

	Try adding an empty build script, just fn main() {}.

	When a build script is present, Cargo is conservative and treats every file within the project directory as a potential source file by default, unless the build script prints explicit rerun-if-changed directives.
*/