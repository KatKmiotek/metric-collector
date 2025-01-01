# Metric Collector
App fetches GitHub metrics for given project, exposes them over https to be consumed by Grafana dashboard
- [Metric Collector](#metric-collector)
  - [1. Tutorials](#1-tutorials)
    - [Tutorial 1: Setup](#tutorial-1-setup)
      - [Objective](#objective)
    - [Tutorial 2: Running docker compose locally](#tutorial-2-running-docker-compose-locally)
      - [Objective](#objective-1)
    - [Tutorial 3: Running project locally](#tutorial-3-running-project-locally)
      - [Objective](#objective-2)
    - [Tutorial 4: Installing pre-commit hooks](#tutorial-4-installing-pre-commit-hooks)
      - [Objective](#objective-3)
  - [3. Reference](#3-reference)
    - [About idea](#about-idea)
  - [4. Explanation](#4-explanation)
    - [Project](#project)
    - [Deployment pipeline](#deployment-pipeline)
    - [Static code analysis](#static-code-analysis)
    - [Dependency updates](#dependency-updates)
    - [Project secrets](#project-secrets)


## 1. Tutorials

Tutorials:
1. **[Setup](#tutorial-1-setup)**
2. **[Running docker compose locally](#tutorial-2-running-docker-compose-locally)**
3. **[Running project locally](#tutorial-3-running-project-locally)**
4. **[Installing pre-commit hooks](#tutorial-4-installing-pre-commit-hooks)**

---
### Tutorial 1: Setup

#### Objective
Setting up development env

Steps:
1. **Step 1**: In the root of the project create .env file
2. **Step 2**: Copy content of .env.example and paste it to .env file
3. **Step 3**: Update values GITHUB_TOKEN, OWNER and REPO
4. **Step 4**: Note: GitHub token is fine-grained token

### Tutorial 2: Running docker compose locally

#### Objective
By following those steps you will setup your machine to run project in docker with Grafana dashboard

Steps:
1. **Step 1**: start project by running: `docker compose up`
2. **Step 2**: App will start on `http://localhost:8080/metrics`
3. **Step 3**: Grafana dashboard runs on `http://localhost:3000/`
4. **Step 4**: Run `docker compose down` to stop application and remove containers


### Tutorial 3: Running project locally

#### Objective
By following those steps you will setup your machine to run project

Steps:
1. **Step 1**: Mac - Install rustup via [instruction](https://doc.rust-lang.org/cargo/getting-started/) from The Rust book
2. **Step 2**: Windows - Install rustup via [instruction](https://doc.rust-lang.org/cargo/getting-started/installation.html) from The Rust book
3. **Step 3**: Confirm `cargo -V` outputs version 1.82.0 or higher
4. **Step 4**: Run `cargo run` - this will build project

### Tutorial 4: Installing pre-commit hooks

#### Objective
Execute static code analysis locally

Steps:
1. **Step 1**: Install [pre-commit]()
2. **Step 2**: Run `pre-commit install` to add hook to .git/hooks/pre-commit - from now on git commit event staged files will be checked
3. **Step 3**: To run pre-commit on all files `pre-commit run --all-files`

---

## 3. Reference
### About idea
The idea behind the project was to build application that can be easy containerized that will run on schedule to collect and serve metrics collected from GitHub, allowing building insightful dashboard into DevEx.

---
## 4. Explanation

### Project
The project is Metric Collector written in Rust.

### Deployment pipeline
1. **Pull Request**: on pull request event there will be number of checks performed that include pre-commit hooks and tests
2. **Merge to main**: will trigger building project into Windows and Mac binaries that are uploaded as part of tagged release
3. **Merge to main**: will also tag repository and update Cargo.toml version
4. **workflow files**: project is using reusable workflows defined in [repository](https://github.com/KatKmiotek/reusable-workflows)

### Static code analysis
Static Code Analysis runs `cargo fmt` and `clippy` commands.
Pre commit hooks can be executed locally after installation - see [guide](#tutorial-4-installing-pre-commit-hooks)

### Dependency updates
For dependency update this project uses [Renovate](https://docs.renovatebot.com/)
Pull Requests with updates are scheduled weekly. All awaiting upgrades are listed on GitHub [issue](https://github.com/KatKmiotek/metric-collector/issues/16)

### Project secrets
For decryption and encryption project is using [SOPS](https://github.com/getsops/sops)
To encrypt secrets from `./secrets/` AGE Private Key is required.
Example usage:
```sh
export SOPS_AGE_KEY=$(cat key.txt)
# encryption
sops -e --input-type dotenv --output-type dotenv .env > secrets/.env.staging

# decryption
sops -d --input-type dotenv --output-type dotenv secrets/.env.staging > .env.decrypted
```
