#[macro_export]
macro_rules! emboss {
    (groups=$($group: ident),+) => {
        $(
            emboss!(group=$group);
        )+
    };
    (group=build) => {
        emboss!(VERGEN_BUILD_DATE);
        emboss!(VERGEN_BUILD_TIME);
        emboss!(VERGEN_BUILD_TIMESTAMP);
        emboss!(VERGEN_BUILD_SEMVER);
    };
    (group=git) => {
        emboss!(VERGEN_GIT_BRANCH);
        emboss!(VERGEN_GIT_COMMIT_DATE);
        emboss!(VERGEN_GIT_COMMIT_TIME);
        emboss!(VERGEN_GIT_COMMIT_TIMESTAMP);
        emboss!(VERGEN_GIT_SEMVER);
        emboss!(VERGEN_GIT_SEMVER_LIGHTWEIGHT);
        emboss!(VERGEN_GIT_SHA);
        emboss!(VERGEN_GIT_SHA_SHORT);
    };
    (group=rustc) => {
        emboss!(VERGEN_RUSTC_CHANNEL);
        emboss!(VERGEN_RUSTC_COMMIT_DATE);
        emboss!(VERGEN_RUSTC_COMMIT_HASH);
        emboss!(VERGEN_RUSTC_HOST_TRIPLE);
        emboss!(VERGEN_RUSTC_LLVM_VERSION);
        emboss!(VERGEN_RUSTC_SEMVER);
    };
    (group=cargo) => {
        emboss!(VERGEN_CARGO_FEATURES);
        emboss!(VERGEN_CARGO_PROFILE);
        emboss!(VERGEN_CARGO_TARGET_TRIPLE);
    };
    (group=sysinfo)=> {
        emboss!(VERGEN_SYSINFO_NAME);
        emboss!(VERGEN_SYSINFO_OS_VERSION);
        emboss!(VERGEN_SYSINFO_USER);
        emboss!(VERGEN_SYSINFO_TOTAL_MEMORY);
        emboss!(VERGEN_SYSINFO_CPU_VENDOR);
        emboss!(VERGEN_SYSINFO_CPU_CORE_COUNT);
        emboss!(VERGEN_SYSINFO_CPU_NAME);
        emboss!(VERGEN_SYSINFO_CPU_BRAND);
        emboss!(VERGEN_SYSINFO_CPU_FREQUENCY);
    };
    (group=all) => {
        emboss!(groups=build,git,rustc,cargo,sysinfo);
    };
    (group=rsps) => {
        emboss!(VERGEN_BUILD_TIMESTAMP);
        emboss!(VERGEN_BUILD_SEMVER);

        emboss!(VERGEN_RUSTC_SEMVER);
        emboss!(VERGEN_CARGO_PROFILE);
        emboss!(VERGEN_CARGO_FEATURES);
    };
    ($var_name: ident) => {
        #[used]
        static $var_name: &str = concat!(
            stringify!($var_name),
            "=",
            env!(stringify!($var_name)),
            ";"
        );
    };
    () => {
        emboss!(group=rsps);
    };
}
