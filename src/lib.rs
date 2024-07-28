use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "pkgx",
            "+nodejs.org",
            "+bun.sh",
            "bun",
            "install",
            "-g",
            &format!("@samkirkland/ftp-deploy@{}", version),
        ])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn deploy(args: String) -> FnResult<String> {
    let version = dag().get_env("FTP_DEPLOY_VERSION")?;

    if version.is_empty() {
        dag().set_envs(vec![("FTP_DEPLOY_VERSION".into(), "latest".into())])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "pkgx",
            "+nodejs.org",
            "+bun.sh",
            "bunx",
            "@samkirkland/ftp-deploy@$FTP_DEPLOY_VERSION",
            &args,
        ])?
        .stdout()?;
    Ok(stdout)
}
