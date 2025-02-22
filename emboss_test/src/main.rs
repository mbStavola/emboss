use emboss::{EmbossingOptions, emboss, emboss_env, emboss_envs, emboss_many};
use object::{Object, ObjectSection};

emboss!(key = "regular-emboss", value = "1");

emboss_many!(pairs = [("many-emboss-1", "2"), ("many-emboss-2", "3")]);

emboss_env!(env_var = "env-emboss-var", key = "env-emboss");

emboss_envs!(env_vars = [
    { env_var = "many-env-emboss-var-1" },
    { env_var = "many-env-emboss-var-2", key = "many-env-emboss-2", fallback = Empty },
    { env_var = "many-env-emboss-var-3", key = "many-env-emboss-3", fallback = Value("7") },
]);

fn main() {
    let self_path = std::env::current_exe().expect("failed to get current path");
    let file = std::fs::read(self_path).expect("could not read file");
    let file = object::File::parse(&*file).expect("failed to parse object file");

    let section = file
        .section_by_name(emboss::DEFAULT_SECTION_NAME)
        .expect("metadata should exist");
    let data = section.data().expect("data should be available");
    let text = String::from_utf8_lossy(data).to_string();
    println!("{}", text);

    let metadata = emboss::extract::extract_metadata(&text, EmbossingOptions::default())
        .expect("should be able to parse metadata");

    let value = metadata
        .get("regular-emboss")
        .expect("regular-emboss should be present");
    assert_eq!(value, "1");

    let value = metadata
        .get("many-emboss-1")
        .expect("many-emboss-1 should be present");
    assert_eq!(value, "2");

    let value = metadata
        .get("many-emboss-2")
        .expect("many-emboss-2 should be present");
    assert_eq!(value, "3");

    let value = metadata
        .get("env-emboss")
        .expect("env-emboss should be present");
    assert_eq!(value, "4");

    let value = metadata
        .get("many-env-emboss-var-1")
        .expect("many-env-emboss-var-1 should be present");
    assert_eq!(value, "5");

    let value = metadata
        .get("many-env-emboss-2")
        .expect("many-env-emboss-2 should be present");
    assert_eq!(value, "6");

    let value = metadata
        .get("many-env-emboss-3")
        .expect("many-env-emboss-3 should be present");
    assert_eq!(value, "7");
}
