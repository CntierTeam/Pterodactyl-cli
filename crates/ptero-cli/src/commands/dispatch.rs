//! Command dispatch — all business via ptero-core services.

use super::*;
use crate::output::{parse_json_body, parse_permissions, print_ok, print_result};
use crate::{ctx_from_cli, load_config};
use anyhow::{bail, Context, Result};
use ptero_core::*;
use ptero_protocol::account::*;
use ptero_protocol::application::*;
use ptero_protocol::backup::*;
use ptero_protocol::files::*;
use ptero_protocol::network::*;
use ptero_protocol::remote::*;
use ptero_protocol::schedule::*;
use ptero_protocol::server::*;
use std::io::{self, BufRead, Write};

pub async fn dispatch(cli: Cli) -> Result<()> {
    let Some(command) = cli.command.clone() else {
        bail!("no subcommand");
    };
    match command {
        Command::Config { action } => run_config(&cli, action).await,
        Command::Account { action } => run_account(&cli, action).await,
        Command::Server { action } => run_server(&cli, action).await,
        Command::File { action } => run_file(&cli, action).await,
        Command::Db { action } => run_db(&cli, action).await,
        Command::Schedule { action } => run_schedule(&cli, action).await,
        Command::Network { action } => run_network(&cli, action).await,
        Command::Subuser { action } => run_subuser(&cli, action).await,
        Command::Backup { action } => run_backup(&cli, action).await,
        Command::Terminal { action } => run_terminal(&cli, action).await,
        Command::App { action } => run_app(&cli, action).await,
        Command::Remote { action } => run_remote(&cli, action).await,
    }
}

async fn run_config(cli: &Cli, action: ConfigCmd) -> Result<()> {
    match action {
        ConfigCmd::Show => {
            let cfg = load_config(cli)?;
            println!("panel_url = {:?}", cfg.panel_url);
            println!("client_api_key = {}", redact(&cfg.client_api_key));
            println!("application_api_key = {}", redact(&cfg.application_api_key));
            println!("daemon_token = {}", redact(&cfg.daemon_token));
            println!(
                "ready: client={} application={} remote={}",
                cfg.is_ready_client(),
                cfg.is_ready_application(),
                cfg.is_ready_remote()
            );
        }
        ConfigCmd::Path => {
            println!("{}", ptero_config::config_path()?.display());
        }
        ConfigCmd::SetUrl { url } => {
            let mut cfg = ptero_config::load().unwrap_or_default();
            cfg.panel_url = url;
            ptero_config::save(&cfg)?;
            print_ok(cli.json, "panel_url saved");
        }
        ConfigCmd::SetClientKey { key } => {
            let mut cfg = ptero_config::load().unwrap_or_default();
            cfg.client_api_key = key;
            ptero_config::save(&cfg)?;
            print_ok(cli.json, "client_api_key saved");
        }
        ConfigCmd::SetAppKey { key } => {
            let mut cfg = ptero_config::load().unwrap_or_default();
            cfg.application_api_key = key;
            ptero_config::save(&cfg)?;
            print_ok(cli.json, "application_api_key saved");
        }
        ConfigCmd::SetDaemonToken { token } => {
            let mut cfg = ptero_config::load().unwrap_or_default();
            cfg.daemon_token = token;
            ptero_config::save(&cfg)?;
            print_ok(cli.json, "daemon_token saved");
        }
    }
    Ok(())
}

fn redact(secret: &str) -> String {
    if secret.is_empty() {
        "(unset)".into()
    } else if secret.len() <= 8 {
        "********".into()
    } else {
        format!("{}…", &secret[..4])
    }
}

async fn run_account(cli: &Cli, action: AccountCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = AccountService(&ctx);
    match action {
        AccountCmd::Get => print_result(cli.json, &svc.get().await?),
        AccountCmd::Email { email, password } => {
            svc.update_email(&UpdateEmailRequest { email, password })
                .await?;
            print_ok(cli.json, "email updated");
        }
        AccountCmd::Password {
            current,
            new_password,
        } => {
            svc.update_password(&UpdatePasswordRequest {
                current_password: current,
                password: new_password.clone(),
                password_confirmation: new_password,
            })
            .await?;
            print_ok(cli.json, "password updated");
        }
        AccountCmd::TwoFactor => print_result(cli.json, &svc.two_factor_details().await?),
        AccountCmd::TwoFactorEnable { code, password } => {
            print_result(
                cli.json,
                &svc.enable_two_factor(&EnableTwoFactorRequest { code, password })
                    .await?,
            );
        }
        AccountCmd::TwoFactorDisable { password } => {
            svc.disable_two_factor(&DisableTwoFactorRequest { password })
                .await?;
            print_ok(cli.json, "2fa disabled");
        }
        AccountCmd::Activity => print_result(cli.json, &svc.activity(&[]).await?),
        AccountCmd::ApiKeys => print_result(cli.json, &svc.list_api_keys().await?),
        AccountCmd::ApiKeyCreate {
            description,
            allowed_ips,
        } => {
            let ips = if allowed_ips.is_empty() {
                vec![]
            } else {
                allowed_ips
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            };
            print_result(
                cli.json,
                &svc.create_api_key(&CreateApiKeyRequest {
                    description,
                    allowed_ips: ips,
                })
                .await?,
            );
        }
        AccountCmd::ApiKeyDelete { identifier } => {
            svc.delete_api_key(&identifier).await?;
            print_ok(cli.json, "api key deleted");
        }
        AccountCmd::SshKeys => print_result(cli.json, &svc.list_ssh_keys().await?),
        AccountCmd::SshKeyCreate { name, public_key } => {
            print_result(
                cli.json,
                &svc.create_ssh_key(&CreateSshKeyRequest { name, public_key })
                    .await?,
            );
        }
        AccountCmd::SshKeyDelete { fingerprint } => {
            svc.delete_ssh_key(&DeleteSshKeyRequest { fingerprint })
                .await?;
            print_ok(cli.json, "ssh key deleted");
        }
    }
    Ok(())
}

async fn run_server(cli: &Cli, action: ServerCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = ServerService(&ctx);
    match action {
        ServerCmd::List => print_result(cli.json, &svc.list(&[]).await?),
        ServerCmd::Permissions => print_result(cli.json, &svc.permissions().await?),
        ServerCmd::Get { server } => print_result(cli.json, &svc.get(&server).await?),
        ServerCmd::Resources { server } => print_result(cli.json, &svc.resources(&server).await?),
        ServerCmd::Activity { server } => {
            print_result(cli.json, &svc.activity(&server, &[]).await?)
        }
        ServerCmd::Power { server, signal } => {
            let sig = PowerSignal::parse(&signal)
                .with_context(|| format!("invalid power signal: {signal}"))?;
            svc.power(&server, sig).await?;
            print_ok(cli.json, &format!("power {signal} sent"));
        }
        ServerCmd::Command { server, command } => {
            svc.command(&server, &command).await?;
            print_ok(cli.json, "command sent");
        }
        ServerCmd::Websocket { server } => print_result(cli.json, &svc.websocket(&server).await?),
        ServerCmd::Startup { server } => print_result(cli.json, &svc.startup(&server).await?),
        ServerCmd::StartupVar { server, key, value } => {
            print_result(
                cli.json,
                &svc.update_startup_variable(&server, &UpdateStartupVariableRequest { key, value })
                    .await?,
            );
        }
        ServerCmd::StartupEgg { server, json_body } => {
            let body = parse_json_body(&json_body)?;
            print_result(cli.json, &svc.update_startup_egg(&server, &body).await?);
        }
        ServerCmd::Rename {
            server,
            name,
            description,
        } => {
            svc.rename(&server, &RenameServerRequest { name, description })
                .await?;
            print_ok(cli.json, "renamed");
        }
        ServerCmd::Reinstall { server } => {
            svc.reinstall(&server).await?;
            print_ok(cli.json, "reinstall queued");
        }
        ServerCmd::DockerImage {
            server,
            docker_image,
        } => {
            svc.set_docker_image(&server, &DockerImageRequest { docker_image })
                .await?;
            print_ok(cli.json, "docker image updated");
        }
    }
    Ok(())
}

async fn run_file(cli: &Cli, action: FileCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = FileService(&ctx);
    match action {
        FileCmd::List { server, directory } => {
            print_result(cli.json, &svc.list(&server, &directory).await?)
        }
        FileCmd::Contents { server, file } => {
            let text = svc.contents(&server, &file).await?;
            if cli.json {
                print_result(cli.json, &text);
            } else {
                print!("{text}");
            }
        }
        FileCmd::Download { server, file } => {
            print_result(cli.json, &svc.download(&server, &file).await?)
        }
        FileCmd::Write {
            server,
            file,
            contents,
            from_file,
        } => {
            let body = if let Some(p) = from_file {
                std::fs::read_to_string(p)?
            } else {
                contents.unwrap_or_default()
            };
            svc.write(&server, &file, &body).await?;
            print_ok(cli.json, "written");
        }
        FileCmd::Rename {
            server,
            root,
            from,
            to,
        } => {
            svc.rename(
                &server,
                &RenameFileRequest {
                    root,
                    files: vec![RenameFilePair { from, to }],
                },
            )
            .await?;
            print_ok(cli.json, "renamed");
        }
        FileCmd::Copy { server, location } => {
            svc.copy(&server, &CopyFileRequest { location }).await?;
            print_ok(cli.json, "copied");
        }
        FileCmd::Compress {
            server,
            root,
            files,
        } => {
            let files: Vec<_> = files.split(',').map(|s| s.trim().to_string()).collect();
            print_result(
                cli.json,
                &svc.compress(&server, &CompressFilesRequest { root, files })
                    .await?,
            );
        }
        FileCmd::Decompress { server, root, file } => {
            svc.decompress(&server, &DecompressFileRequest { root, file })
                .await?;
            print_ok(cli.json, "decompressed");
        }
        FileCmd::Delete {
            server,
            root,
            files,
        } => {
            let files: Vec<_> = files.split(',').map(|s| s.trim().to_string()).collect();
            svc.delete(&server, &DeleteFilesRequest { root, files })
                .await?;
            print_ok(cli.json, "deleted");
        }
        FileCmd::Mkdir { server, root, name } => {
            svc.create_folder(&server, &CreateFolderRequest { root, name })
                .await?;
            print_ok(cli.json, "folder created");
        }
        FileCmd::Chmod {
            server,
            root,
            file,
            mode,
        } => {
            svc.chmod(
                &server,
                &ChmodFilesRequest {
                    root,
                    files: vec![ChmodFileEntry { file, mode }],
                },
            )
            .await?;
            print_ok(cli.json, "chmod ok");
        }
        FileCmd::Pull {
            server,
            url,
            directory,
            filename,
        } => {
            svc.pull(
                &server,
                &PullFileRequest {
                    url,
                    directory,
                    filename,
                    use_header: None,
                    foreground: None,
                },
            )
            .await?;
            print_ok(cli.json, "pull started");
        }
        FileCmd::UploadUrl { server } => print_result(cli.json, &svc.upload_url(&server).await?),
    }
    Ok(())
}

async fn run_db(cli: &Cli, action: DbCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = DatabaseService(&ctx);
    match action {
        DbCmd::List { server } => print_result(cli.json, &svc.list(&server).await?),
        DbCmd::Create {
            server,
            database,
            remote,
        } => {
            print_result(
                cli.json,
                &svc.create(&server, &CreateDatabaseRequest { database, remote })
                    .await?,
            );
        }
        DbCmd::Rotate { server, database } => {
            print_result(cli.json, &svc.rotate(&server, &database).await?)
        }
        DbCmd::Delete { server, database } => {
            svc.delete(&server, &database).await?;
            print_ok(cli.json, "database deleted");
        }
    }
    Ok(())
}

async fn run_schedule(cli: &Cli, action: ScheduleCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = ScheduleService(&ctx);
    match action {
        ScheduleCmd::List { server } => print_result(cli.json, &svc.list(&server).await?),
        ScheduleCmd::Get { server, schedule } => {
            print_result(cli.json, &svc.get(&server, schedule).await?)
        }
        ScheduleCmd::Create {
            server,
            name,
            minute,
            hour,
            day_of_month,
            month,
            day_of_week,
            active,
        } => {
            print_result(
                cli.json,
                &svc.create(
                    &server,
                    &CreateScheduleRequest {
                        name,
                        minute,
                        hour,
                        day_of_month,
                        month,
                        day_of_week,
                        is_active: active,
                        only_when_online: false,
                    },
                )
                .await?,
            );
        }
        ScheduleCmd::Update {
            server,
            schedule,
            json_body,
        } => {
            let body = parse_json_body(&json_body)?;
            print_result(cli.json, &svc.update(&server, schedule, &body).await?);
        }
        ScheduleCmd::Execute { server, schedule } => {
            svc.execute(&server, schedule).await?;
            print_ok(cli.json, "executed");
        }
        ScheduleCmd::Delete { server, schedule } => {
            svc.delete(&server, schedule).await?;
            print_ok(cli.json, "deleted");
        }
        ScheduleCmd::TaskCreate {
            server,
            schedule,
            action,
            payload,
            time_offset,
        } => {
            print_result(
                cli.json,
                &svc.create_task(
                    &server,
                    schedule,
                    &CreateTaskRequest {
                        action,
                        payload,
                        time_offset,
                        continue_on_failure: false,
                    },
                )
                .await?,
            );
        }
        ScheduleCmd::TaskUpdate {
            server,
            schedule,
            task,
            json_body,
        } => {
            let body = parse_json_body(&json_body)?;
            print_result(
                cli.json,
                &svc.update_task(&server, schedule, task, &body).await?,
            );
        }
        ScheduleCmd::TaskDelete {
            server,
            schedule,
            task,
        } => {
            svc.delete_task(&server, schedule, task).await?;
            print_ok(cli.json, "task deleted");
        }
    }
    Ok(())
}

async fn run_network(cli: &Cli, action: NetworkCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = NetworkService(&ctx);
    match action {
        NetworkCmd::List { server } => print_result(cli.json, &svc.list(&server).await?),
        NetworkCmd::Create { server } => print_result(cli.json, &svc.create(&server).await?),
        NetworkCmd::Notes {
            server,
            allocation,
            notes,
        } => {
            print_result(
                cli.json,
                &svc.update(
                    &server,
                    allocation,
                    &UpdateAllocationRequest { notes: Some(notes) },
                )
                .await?,
            );
        }
        NetworkCmd::Primary { server, allocation } => {
            svc.set_primary(&server, allocation).await?;
            print_ok(cli.json, "primary set");
        }
        NetworkCmd::Delete { server, allocation } => {
            svc.delete(&server, allocation).await?;
            print_ok(cli.json, "allocation deleted");
        }
    }
    Ok(())
}

async fn run_subuser(cli: &Cli, action: SubuserCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = SubuserService(&ctx);
    match action {
        SubuserCmd::List { server } => print_result(cli.json, &svc.list(&server).await?),
        SubuserCmd::Get { server, user } => print_result(cli.json, &svc.get(&server, &user).await?),
        SubuserCmd::Create {
            server,
            email,
            permissions,
        } => {
            print_result(
                cli.json,
                &svc.create(
                    &server,
                    &SubuserRequest {
                        email,
                        permissions: parse_permissions(&permissions),
                    },
                )
                .await?,
            );
        }
        SubuserCmd::Update {
            server,
            user,
            email,
            permissions,
        } => {
            print_result(
                cli.json,
                &svc.update(
                    &server,
                    &user,
                    &SubuserRequest {
                        email,
                        permissions: parse_permissions(&permissions),
                    },
                )
                .await?,
            );
        }
        SubuserCmd::Delete { server, user } => {
            svc.delete(&server, &user).await?;
            print_ok(cli.json, "subuser deleted");
        }
    }
    Ok(())
}

async fn run_backup(cli: &Cli, action: BackupCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = BackupService(&ctx);
    match action {
        BackupCmd::List { server } => print_result(cli.json, &svc.list(&server).await?),
        BackupCmd::Get { server, backup } => {
            print_result(cli.json, &svc.get(&server, &backup).await?)
        }
        BackupCmd::Create {
            server,
            name,
            ignored,
            locked,
        } => {
            print_result(
                cli.json,
                &svc.create(
                    &server,
                    &CreateBackupRequest {
                        name,
                        ignored,
                        is_locked: if locked { Some(true) } else { None },
                    },
                )
                .await?,
            );
        }
        BackupCmd::Download { server, backup } => {
            print_result(cli.json, &svc.download(&server, &backup).await?)
        }
        BackupCmd::Lock { server, backup } => {
            print_result(cli.json, &svc.toggle_lock(&server, &backup).await?)
        }
        BackupCmd::Restore {
            server,
            backup,
            truncate,
        } => {
            svc.restore(&server, &backup, &RestoreBackupRequest { truncate })
                .await?;
            print_ok(cli.json, "restore started");
        }
        BackupCmd::Delete { server, backup } => {
            svc.delete(&server, &backup).await?;
            print_ok(cli.json, "backup deleted");
        }
    }
    Ok(())
}

async fn run_terminal(cli: &Cli, action: TerminalCmd) -> Result<()> {
    let TerminalCmd::Attach { server } = action;
    let ctx = ctx_from_cli(cli)?;
    let mut console = TerminalService(&ctx).attach(&server).await?;
    println!("attached to {server}; type commands, or 'exit' / Ctrl-D to quit");
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    std::thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    if tx.send(l).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
    loop {
        tokio::select! {
            ev = console.next() => {
                match ev {
                    Some(ConsoleEvent::ConsoleOutput(line)) => {
                        println!("{line}");
                        let _ = io::stdout().flush();
                    }
                    Some(ConsoleEvent::Status(s)) => eprintln!("[status] {s}"),
                    Some(ConsoleEvent::DaemonMessage(s)) => eprintln!("[daemon] {s}"),
                    Some(ConsoleEvent::DaemonError(s)) => eprintln!("[error] {s}"),
                    Some(ConsoleEvent::AuthSuccess) => eprintln!("[auth] ok"),
                    Some(ConsoleEvent::Closed) | None => break,
                    _ => {}
                }
            }
            cmd = rx.recv() => {
                match cmd {
                    Some(c) => {
                        if c == "exit" || c == "quit" { break; }
                        console.send_command(&c).await?;
                    }
                    None => break,
                }
            }
        }
    }
    console.close().await;
    Ok(())
}

async fn run_app(cli: &Cli, action: AppCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = ApplicationService(&ctx);
    match action {
        AppCmd::User { action } => match action {
            AppUserCmd::List => print_result(cli.json, &svc.list_users(&[]).await?),
            AppUserCmd::Get { id } => print_result(cli.json, &svc.get_user(id).await?),
            AppUserCmd::External { external_id } => {
                print_result(cli.json, &svc.get_user_external(&external_id).await?)
            }
            AppUserCmd::Create {
                email,
                username,
                first_name,
                last_name,
                password,
                root_admin,
            } => {
                print_result(
                    cli.json,
                    &svc.create_user(&CreateAppUserRequest {
                        email,
                        username,
                        first_name,
                        last_name,
                        external_id: None,
                        password,
                        root_admin: if root_admin { Some(true) } else { None },
                        language: None,
                    })
                    .await?,
                );
            }
            AppUserCmd::Update { id, json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.update_user(id, &body).await?);
            }
            AppUserCmd::Delete { id } => {
                svc.delete_user(id).await?;
                print_ok(cli.json, "user deleted");
            }
        },
        AppCmd::Node { action } => match action {
            AppNodeCmd::List => print_result(cli.json, &svc.list_nodes(&[]).await?),
            AppNodeCmd::Get { id } => print_result(cli.json, &svc.get_node(id).await?),
            AppNodeCmd::Deployable => print_result(cli.json, &svc.deployable_nodes(&[]).await?),
            AppNodeCmd::Configuration { id } => {
                print_result(cli.json, &svc.node_configuration(id).await?)
            }
            AppNodeCmd::Create { json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.create_node(&body).await?);
            }
            AppNodeCmd::Update { id, json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.update_node(id, &body).await?);
            }
            AppNodeCmd::Delete { id } => {
                svc.delete_node(id).await?;
                print_ok(cli.json, "node deleted");
            }
            AppNodeCmd::Allocations { node } => {
                print_result(cli.json, &svc.list_node_allocations(node, &[]).await?)
            }
            AppNodeCmd::AllocationCreate { node, json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.create_node_allocations(node, &body).await?);
            }
            AppNodeCmd::AllocationDelete { node, allocation } => {
                svc.delete_node_allocation(node, allocation).await?;
                print_ok(cli.json, "allocation deleted");
            }
        },
        AppCmd::Location { action } => match action {
            AppLocationCmd::List => print_result(cli.json, &svc.list_locations(&[]).await?),
            AppLocationCmd::Get { id } => print_result(cli.json, &svc.get_location(id).await?),
            AppLocationCmd::Create { short, long } => {
                print_result(
                    cli.json,
                    &svc.create_location(&CreateLocationRequest { short, long })
                        .await?,
                );
            }
            AppLocationCmd::Update { id, json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.update_location(id, &body).await?);
            }
            AppLocationCmd::Delete { id } => {
                svc.delete_location(id).await?;
                print_ok(cli.json, "location deleted");
            }
        },
        AppCmd::Server { action } => match action {
            AppServerCmd::List => print_result(cli.json, &svc.list_servers(&[]).await?),
            AppServerCmd::Get { id } => print_result(cli.json, &svc.get_server(id).await?),
            AppServerCmd::External { external_id } => {
                print_result(cli.json, &svc.get_server_external(&external_id).await?)
            }
            AppServerCmd::Create { json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.create_server(&body).await?);
            }
            AppServerCmd::Details { id, json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.update_server_details(id, &body).await?);
            }
            AppServerCmd::Build { id, json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.update_server_build(id, &body).await?);
            }
            AppServerCmd::Startup { id, json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.update_server_startup(id, &body).await?);
            }
            AppServerCmd::Suspend { id } => {
                svc.suspend_server(id).await?;
                print_ok(cli.json, "suspended");
            }
            AppServerCmd::Unsuspend { id } => {
                svc.unsuspend_server(id).await?;
                print_ok(cli.json, "unsuspended");
            }
            AppServerCmd::Reinstall { id } => {
                svc.reinstall_server(id).await?;
                print_ok(cli.json, "reinstall queued");
            }
            AppServerCmd::Delete { id, force } => {
                svc.delete_server(id, force).await?;
                print_ok(cli.json, "server deleted");
            }
            AppServerCmd::DbList { server } => {
                print_result(cli.json, &svc.list_databases(server).await?)
            }
            AppServerCmd::DbGet { server, database } => {
                print_result(cli.json, &svc.get_database(server, database).await?)
            }
            AppServerCmd::DbCreate { server, json_body } => {
                let body = parse_json_body(&json_body)?;
                print_result(cli.json, &svc.create_database(server, &body).await?);
            }
            AppServerCmd::DbReset { server, database } => {
                svc.reset_database_password(server, database).await?;
                print_ok(cli.json, "password reset");
            }
            AppServerCmd::DbDelete { server, database } => {
                svc.delete_database(server, database).await?;
                print_ok(cli.json, "database deleted");
            }
        },
        AppCmd::Nest { action } => match action {
            AppNestCmd::List => print_result(cli.json, &svc.list_nests(&[]).await?),
            AppNestCmd::Get { id } => print_result(cli.json, &svc.get_nest(id).await?),
        },
        AppCmd::Egg { action } => match action {
            AppEggCmd::List { nest } => print_result(cli.json, &svc.list_eggs(nest, &[]).await?),
            AppEggCmd::Get { nest, egg } => print_result(cli.json, &svc.get_egg(nest, egg).await?),
        },
    }
    Ok(())
}

async fn run_remote(cli: &Cli, action: RemoteCmd) -> Result<()> {
    let ctx = ctx_from_cli(cli)?;
    let svc = RemoteService(&ctx);
    match action {
        RemoteCmd::SftpAuth { username, password } => {
            print_result(
                cli.json,
                &svc.sftp_auth(&SftpAuthRequest { username, password })
                    .await?,
            );
        }
        RemoteCmd::Servers => print_result(cli.json, &svc.list_servers(&[]).await?),
        RemoteCmd::ServersReset => {
            svc.reset_servers().await?;
            print_ok(cli.json, "servers reset");
        }
        RemoteCmd::Activity { json_body } => {
            let body = parse_json_body(&json_body)?;
            svc.ingest_activity(&body).await?;
            print_ok(cli.json, "activity ingested");
        }
        RemoteCmd::Server { uuid } => print_result(cli.json, &svc.server_details(&uuid).await?),
        RemoteCmd::Install { uuid } => print_result(cli.json, &svc.install_details(&uuid).await?),
        RemoteCmd::InstallComplete { uuid, successful } => {
            svc.install_complete(
                &uuid,
                &RemoteInstallCompleteRequest {
                    successful,
                    sequence: None,
                },
            )
            .await?;
            print_ok(cli.json, "install complete reported");
        }
        RemoteCmd::TransferFailure { uuid } => {
            svc.transfer_failure(&uuid).await?;
            print_ok(cli.json, "transfer failure reported");
        }
        RemoteCmd::TransferSuccess { uuid } => {
            svc.transfer_success(&uuid).await?;
            print_ok(cli.json, "transfer success reported");
        }
        RemoteCmd::BackupUpload { backup } => {
            print_result(cli.json, &svc.backup_upload(&backup).await?)
        }
        RemoteCmd::BackupStatus { backup, json_body } => {
            let req: RemoteBackupStatusRequest = serde_json::from_str(&json_body)?;
            svc.backup_status(&backup, &req).await?;
            print_ok(cli.json, "backup status reported");
        }
        RemoteCmd::BackupRestore { backup, successful } => {
            svc.backup_restore(&backup, &RemoteBackupRestoreRequest { successful })
                .await?;
            print_ok(cli.json, "backup restore reported");
        }
    }
    Ok(())
}
