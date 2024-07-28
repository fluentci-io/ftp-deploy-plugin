# FTP Deploy Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/ftp-deploy)](https://pkg.fluentci.io/ftp-deploy)
[![ci](https://github.com/fluentci-io/ftp-deploy-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/ftp-deploy-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [@samkirkland/ftp-deploy](https://github.com/SamKirkland/ftp-deploy).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm ftp-deploy deploy --server ftp.samkirkland.com --username test@samkirkland.com --password \"CrazyUniquePassword&%123\"
```

## Functions

| Name   | Description                                |
| ------ | ------------------------------------------ |
| setup  | Installs a specific version of ftp-deploy. |
| deploy | Deploy files to a remote FTP server.       |

## Environment Variables

| Name               | Description                               | Default                            |
| ------------------ | ----------------------------------------- | ---------------------------------- |
| FTP_DEPLOY_VERSION | The version of ftp-deploy to install.     | `latest`                           |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call(
  "https://pkg.fluentci.io/ftp-deploy@v0.1.0?wasm=1", 
  "deploy", 
  vec![
    "--server", "ftp.samkirkland.com", 
    "--username", "test@samkirkland.com", 
    "--password", "\"CrazyUniquePassword&%123\""
])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: ftp-deploy
    args: |
      deploy --server ftp.samkirkland.com --username test@samkirkland.com --password \"CrazyUniquePassword&%123\"
```
