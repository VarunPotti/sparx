use clap::{command, Arg, ArgGroup, ArgMatches, Command};

pub fn build_cli() -> anyhow::Result<ArgMatches> {
    let matches= command!().author("Varun Potti")
        .subcommand(Command::new("init").about(
            "Create a new set of files/folders based on rules specified in the .sparx directory",
        ).long_about(
            "Create a new set of files/folders based on rules specified in the .sparx directory"
        ).arg(
            Arg::new("template-name")
                .help("The name of the template to use. The template must be present in the <current-dir>/.sparx directory")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("name").help("The name to be used for the creation of files using the specified template.").required(true).index(2)
        )
        .arg(
            Arg::new("directory")
                .help("The directory to create the files/folders in, relative to the current directory")
                .required(true)
                .index(3)
        ).arg(
            Arg::new("flags").value_delimiter(' ').required(false).index(4)
        )
    )
        .get_matches();
    Ok(matches)
}
