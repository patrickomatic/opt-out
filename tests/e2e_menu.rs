use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::Stdio;
use std::time::Duration;

use thirtyfour::prelude::*;
use tokio::process::{Child, Command};

type TestResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

const BROKER_ID: &str = "fastbackgroundcheck";
const BROKER_NAME: &str = "FastBackgroundCheck";
const PUBLIC_URL: &str = "/opt-out/";

struct ChildGuard {
    child: Child,
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

#[tokio::test]
async fn broker_menu_navigates_and_closes_on_outside_click() -> TestResult<()> {
    tokio::time::timeout(Duration::from_secs(90), run_broker_menu_test()).await??;
    Ok(())
}

async fn run_broker_menu_test() -> TestResult<()> {
    let port = open_local_port()?;
    let mut trunk = start_trunk(port)?;
    wait_for_port(&mut trunk, port).await?;

    let webdriver_port = open_local_port()?;
    let mut chromedriver = start_chromedriver(webdriver_port)?;
    wait_for_port(&mut chromedriver, webdriver_port).await?;

    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    caps.set_no_sandbox()?;
    caps.set_disable_gpu()?;
    caps.set_disable_dev_shm_usage()?;
    if let Ok(chrome_path) = std::env::var("CHROME_PATH")
        && !chrome_path.is_empty()
    {
        caps.set_binary(&chrome_path)?;
    }

    let driver = WebDriver::new(format!("http://127.0.0.1:{webdriver_port}"), caps).await?;
    let result = exercise_broker_menu(&driver, port).await;
    let quit_result = driver.quit().await;

    result?;
    quit_result?;

    Ok(())
}

async fn exercise_broker_menu(driver: &WebDriver, port: u16) -> TestResult<()> {
    let base_url = format!("http://127.0.0.1:{port}{PUBLIC_URL}");
    driver.goto(&base_url).await?;
    driver.find(By::Css(".app-header")).await?;

    open_broker_menu(driver).await?;
    driver
        .find(By::XPath(format!(
            "//details[contains(@class,'broker-menu')]//a[.//span[normalize-space()='{BROKER_NAME}']]"
        )))
        .await?
        .click()
        .await?;

    wait_for_path(driver, &format!("/opt-out/workflow/{BROKER_ID}")).await?;
    wait_for_selector(
        driver,
        &format!("#{BROKER_ID_PREFIX}{BROKER_ID}.selected-broker"),
    )
    .await?;

    open_broker_menu(driver).await?;
    driver.find(By::Css("main")).await?.click().await?;
    wait_for_broker_menu_closed(driver).await?;

    Ok(())
}

const BROKER_ID_PREFIX: &str = "broker-";

async fn open_broker_menu(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find(By::Css("details.broker-menu > summary"))
        .await?
        .click()
        .await?;
    let menu = driver.find(By::Css("details.broker-menu")).await?;
    assert_eq!(menu.attr("open").await?.as_deref(), Some(""));
    Ok(())
}

async fn wait_for_path(driver: &WebDriver, expected_path: &str) -> TestResult<()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    loop {
        let current_url = driver.current_url().await?;
        if current_url.path() == expected_path {
            return Ok(());
        }
        if tokio::time::Instant::now() >= deadline {
            return Err(format!("expected browser path {expected_path}, got {current_url}").into());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn wait_for_selector(driver: &WebDriver, selector: &str) -> TestResult<()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    loop {
        if driver.find(By::Css(selector)).await.is_ok() {
            return Ok(());
        }
        if tokio::time::Instant::now() >= deadline {
            return Err(format!("selector did not appear: {selector}").into());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn wait_for_broker_menu_closed(driver: &WebDriver) -> TestResult<()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(4);
    loop {
        let menu = driver.find(By::Css("details.broker-menu")).await?;
        if menu.attr("open").await?.is_none() {
            return Ok(());
        }
        if tokio::time::Instant::now() >= deadline {
            return Err("broker menu stayed open after outside click".into());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn open_local_port() -> TestResult<u16> {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;
    Ok(listener.local_addr()?.port())
}

fn start_trunk(port: u16) -> TestResult<ChildGuard> {
    let mut command = Command::new("trunk");
    command
        .arg("serve")
        .arg("--port")
        .arg(port.to_string())
        .arg("--public-url")
        .arg(PUBLIC_URL)
        .env_remove("NO_COLOR")
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    if let Some(rustc) = rustup_rustc() {
        command.env("RUSTC", rustc);
    }

    Ok(ChildGuard {
        child: command.spawn()?,
    })
}

fn start_chromedriver(port: u16) -> TestResult<ChildGuard> {
    let child = Command::new("chromedriver")
        .arg(format!("--port={port}"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("failed to start chromedriver from PATH: {error}"))?;

    Ok(ChildGuard { child })
}

fn rustup_rustc() -> Option<String> {
    let output = std::process::Command::new("rustup")
        .arg("which")
        .arg("rustc")
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

async fn wait_for_port(process: &mut ChildGuard, port: u16) -> TestResult<()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(45);
    loop {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return Ok(());
        }
        if let Some(status) = process.child.try_wait()? {
            return Err(format!("process exited before listening on port {port}: {status}").into());
        }
        if tokio::time::Instant::now() >= deadline {
            return Err(format!("process did not listen on port {port}").into());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}
