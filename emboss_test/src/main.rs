use std::collections::HashMap;

use emboss::{emboss, emboss_env, emboss_envs, emboss_many};
use object::{Object, ObjectSection};

emboss!(key = "regular-emboss", value = "1");

emboss_many!(
    items = [
        { key = "many-emboss-1", value = "2" },
        { key = "many-emboss-2", value = "3" }
    ],
    stored_in = "somewhere"
);

emboss_env!(
    env_var = "env-emboss-var",
    key = "env-emboss",
    stored_in = "elsewhere"
);

emboss_envs!(export_name = ManyEnvVars, env_vars = [
    { env_var = "many-env-emboss-var-1" },
    { env_var = "many-env-emboss-var-2", fallback = Empty },
    { env_var = "many-env-emboss-var-3", key = "many-env-emboss-3", fallback = Empty },
    { env_var = "many-env-emboss-var-4", key = "many-env-emboss-4", fallback = Value("7"), variant_name = "LastEmbossVar" },
], stored_in = "nowhere");

fn main() {
    let self_path = std::env::current_exe().expect("failed to get current path");
    let file = std::fs::read(self_path).expect("could not read file");
    let file = object::File::parse(&*file).expect("failed to parse object file");

    let metadata = get_section_data(&file, emboss::DEFAULT_SECTION_NAME);

    let value = *metadata
        .get("regular-emboss")
        .expect("regular-emboss should be present");
    assert_eq!(value, "1");

    let metadata = get_section_data(&file, "somewhere");

    let value = *metadata
        .get("many-emboss-1")
        .expect("many-emboss-1 should be present");
    assert_eq!(value, "2");

    let value = *metadata
        .get("many-emboss-2")
        .expect("many-emboss-2 should be present");
    assert_eq!(value, "3");

    let metadata = get_section_data(&file, "elsewhere");

    let value = *metadata
        .get("env-emboss")
        .expect("env-emboss should be present");
    assert_eq!(value, "4");

    let metadata = get_section_data(&file, "nowhere");

    let value = *metadata
        .get("many-env-emboss-var-1")
        .expect("many-env-emboss-var-1 should be present");
    assert_eq!(value, "5");

    let value = *metadata
        .get("many-env-emboss-var-2")
        .expect("many-env-emboss-var-2 should be present");
    assert_eq!(value, "");

    let value = *metadata
        .get("many-env-emboss-3")
        .expect("many-env-emboss-3 should be present");
    assert_eq!(value, "6");

    let value = *metadata
        .get("many-env-emboss-4")
        .expect("many-env-emboss-4 should be present");
    assert_eq!(value, "7");

    let field = ManyEnvVars::EMBOSSED.get_by_index(0);
    assert_eq!(field, Some(("many-env-emboss-var-1", "5")));

    let field = ManyEnvVars::EMBOSSED.get_by_kind(ManyEnvVars::EmbossedKeyKind::ManyEnvEmbossVar2);
    assert_eq!(field, ("many-env-emboss-var-2", ""));

    let field = ManyEnvVars::EMBOSSED.get_by_key("many-env-emboss-3");
    assert_eq!(field, Some(("many-env-emboss-3", "6")));

    let field = ManyEnvVars::EMBOSSED.get_by_kind(ManyEnvVars::EmbossedKeyKind::LastEmbossVar);
    assert_eq!(field, ("many-env-emboss-4", "7"));
}

fn get_section_data<'a>(file: &'a object::File, section_name: &str) -> HashMap<&'a str, &'a str> {
    let section = file
        .section_by_name(section_name)
        .expect("metadata should exist");
    let data = section.data().expect("data should be available");
    emboss::extract::extract_metadata_into_hashmap(data).expect("should be able to parse metadata")
}
