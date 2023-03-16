use crate::{
    lua::{
        output::{
            cve::CveReport,
            general::GeneralReport,
            vuln::{AllReports, OutReport},
        },
        parsing::url::HttpMessage,
    },
    CliErrors, LuaRunTime,
};
use log::{debug, error, info, warn};
use mlua::ExternalError;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

pub trait HTTPEXT {
    fn add_httpfuncs(&self, target_url: Option<&str>);
}

impl HTTPEXT for LuaRunTime<'_> {
    fn add_httpfuncs(&self, target_url: Option<&str>) {
        self.lua
            .globals()
            .set(
                "JOIN_SCRIPT_DIR",
                self.lua
                    .create_function(|c_lua, new_path: String| {
                        let script_path = c_lua.globals().get::<_, String>("SCRIPT_PATH").unwrap();
                        let the_path = Path::new(&script_path);
                        Ok(the_path
                            .parent()
                            .unwrap()
                            .join(new_path)
                            .to_str()
                            .unwrap()
                            .to_string())
                    })
                    .unwrap(),
            )
            .unwrap();

        let log_info = self
            .lua
            .create_function(|_, log_msg: String| {
                info!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        let log_warn = self
            .lua
            .create_function(|_, log_msg: String| {
                warn!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        let log_debug = self
            .lua
            .create_function(|_, log_msg: String| {
                debug!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        let log_error = self
            .lua
            .create_function(|_, log_msg: String| {
                error!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        self.lua
            .globals()
            .set(
                "sleep",
                self.lua
                    .create_async_function(|_, time: u64| async move {
                        sleep(Duration::from_secs(time)).await;
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "pathjoin",
                self.lua
                    .create_function(|_, (current_path, new_path): (String, String)| {
                        let the_path = std::path::Path::new(&current_path).join(new_path);
                        Ok(the_path.to_str().unwrap().to_string())
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "readfile",
                self.lua
                    .create_function(|_ctx, file_path: String| {
                        if Path::new(&file_path).exists() {
                            let mut file = File::open(&file_path)?;
                            let mut file_content = String::new();
                            file.read_to_string(&mut file_content)?;
                            Ok(file_content)
                        } else {
                            Err(CliErrors::ReadingError.to_lua_err())
                        }
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua.globals().set("log_info", log_info).unwrap();
        self.lua.globals().set("log_error", log_error).unwrap();
        self.lua.globals().set("log_debug", log_debug).unwrap();
        self.lua.globals().set("log_warn", log_warn).unwrap();

        if !target_url.is_none() {
            self.lua
                .globals()
                .set(
                    "HttpMessage",
                    HttpMessage {
                        url: Url::parse(target_url.unwrap()).unwrap(),
                    },
                )
                .unwrap();
        }
        self.lua
            .globals()
            .set(
                "Reports",
                AllReports {
                    reports: Vec::new(),
                },
            )
            .unwrap();
        self.lua
            .globals()
            .set("VulnReport", OutReport::init())
            .unwrap();
        self.lua
            .globals()
            .set("GeneralReport", GeneralReport::init())
            .unwrap();
        self.lua
            .globals()
            .set("CveReport", CveReport::init())
            .unwrap();
    }
}
