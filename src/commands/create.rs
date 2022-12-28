use crate::{config::Config, utils::str_to_case};
use convert_case::{Case, Casing};

pub fn replace_contents(content: String, to_replace: String, value: String, case: Case) -> String {
    let content = content.clone();
    let content = content.replace(&to_replace, &value.to_case(case));

    content
}

pub async fn create(
    template_name: String,
    name: String,
    directory: String,
    variables: std::collections::HashMap<String, String>,
    config: Config,
) -> anyhow::Result<()> {
    let template_dir = config.config_dir.join(template_name);

    if !template_dir.exists() {
        anyhow::bail!("Template directory does not exist");
    }

    let directory_path = config.current_dir.join(directory);

    let casings_supported: Vec<&str> = vec![
        "snake",
        "kebab",
        "camel",
        "pascal",
        "upper",
        "lower",
        "title",
        "toggle",
        "upper_camel",
        "upper_snake",
        "cobol",
        "upper_kebab",
        "train",
        "flat",
        "upper_flat",
        "alternating",
    ];

    if !directory_path.parent().unwrap().exists() {
        std::fs::create_dir_all(directory_path.parent().unwrap())?;
    }

    // copy the contents of the template directory to the directory path
    copy_dir::copy_dir(&template_dir, &directory_path)?;

    // walk through the directory_path and replace all instances of {{name:case}} with the name (case is either snake, kebab, camel, pascal, upper, lower, title, toggle, upper_camel, upper_snake, cobol, upper_kebab, train, flat, upper_flat, alternating)
    for entry in walkdir::WalkDir::new(&directory_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let mut contents = std::fs::read_to_string(path).unwrap();

            for casing in &casings_supported {
                let to_replace = format!("{{{{name:{}}}}}", casing);

                contents = replace_contents(
                    contents.clone(),
                    to_replace,
                    name.clone(),
                    str_to_case(casing),
                );
            }

            for variable in variables.iter() {
                for casing in &casings_supported {
                    let to_replace = format!("{{{{{}:{}}}}}", variable.0, casing);

                    contents = replace_contents(
                        contents.clone(),
                        to_replace,
                        variable.1.clone(),
                        str_to_case(casing),
                    );
                }
            }

            std::fs::write(path, contents).unwrap();
        }

        for casing in &casings_supported {
            let to_replace = format!("{{{{name:{}}}}}", casing);
            let new_name = name.to_case(Case::Snake);
            rename_file(path, &new_name, &to_replace, str_to_case(casing));
        }

        for variable in variables.iter() {
            for casing in &casings_supported {
                let to_replace = format!("{{{{{}:{}}}}}", variable.0, casing);
                let new_name = variable.1.to_case(Case::Snake);
                rename_file(path, &new_name, &to_replace, str_to_case(casing));
            }
        }
    }

    Ok(())
}

fn rename_file(path: &std::path::Path, name: &String, file_name_check: &str, case: Case) {
    if path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains(file_name_check)
    {
        let new_path = path.with_file_name(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(file_name_check, &name.to_case(case)),
        );

        std::fs::rename(path, new_path).unwrap();
    }
}
