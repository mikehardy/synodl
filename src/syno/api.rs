/*

SynoDL - CLI for Synology's DownloadStation
Copyright (C) 2015 - 2023  Stefan Ott

This program is free software: you can redistribute it and/or
modify it under the terms of the GNU General Public License as
published by the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.

*/

use std::{error, io::{self, ErrorKind}, sync::Arc};

use url::Url;
use serde::{Deserialize, Serialize};
use crate::syno::tls::get_tls_config;

use crate::{
    Task, Config
};

#[derive(Deserialize, Serialize)]
pub struct Session {
    sid: String,
}

#[derive(Deserialize, Serialize)]
struct SynoResponse {
    success: bool
}

#[derive(Deserialize, Serialize)]
struct SessionResponse {
    #[serde(flatten)]
    response: SynoResponse,
    data: Session
}

#[derive(Deserialize, Serialize)]
struct TaskResponseTransfer {
    size_downloaded: u64,
    size_uploaded: u64,
    speed_download: u64,
    speed_upload: u64
}

#[derive(Deserialize, Serialize)]
struct TaskResponseAdditional {
    transfer: TaskResponseTransfer
}

#[derive(Deserialize, Serialize)]
struct TaskResponse {
    id: String,
    title: String,
    status: String,
    size: u64,
    additional: TaskResponseAdditional
}

#[derive(Deserialize, Serialize)]
struct TaskListResponseData {
    offset: u32,
    tasks: Vec<TaskResponse>
}

#[derive(Deserialize, Serialize)]
struct TaskListResponse {
    #[serde(flatten)]
    response: SynoResponse,
    data: TaskListResponseData
}


fn syno_do(url: &Url, cfg: &Config) -> Result<String, Box<dyn error::Error>>
{
    let tls_config = get_tls_config(cfg)?;

    let agent = ureq::AgentBuilder::new()
        .tls_config(Arc::new(tls_config))
        .build();

    let res = agent
        .request_url("GET", &url)
        .call()?
        .into_string()?;

    let syno = serde_json::from_str::<SynoResponse>(&res);
    let success = match syno {
        Ok(res) => res.success,
        Err(e) => {
            println!("Failed to load JSON response: {}", res);
            return Err(Box::new(e))
        }
    };

    match success {
        true => Ok(res),
        false => {
            eprintln!("API request failed: {}", url);
            eprintln!("Response was: {}", res);
            Err(Box::new(io::Error::new(ErrorKind::Other, "API request failed")))
        }
    }
}

pub fn syno_login(cfg: &Config)
                  -> Result<Session, Box<dyn error::Error>> {
    let mut url = Url::parse(&cfg.url)?
        .join("/webapi/auth.cgi")?;
    url.query_pairs_mut()
        .clear()
        .append_pair("api", "SYNO.API.Auth")
        .append_pair("version", "2")
        .append_pair("method", "login")
        .append_pair("account", &cfg.user)
        .append_pair("passwd", cfg.password.as_ref().unwrap())
        .append_pair("session", "DownloadStation")
        .append_pair("format", "sid");

    let res = syno_do(&url, cfg)?;

    match serde_json::from_str::<SessionResponse>(&res) {
        Ok(s) => Ok(s.data),
        Err(e) => {
            println!("Failed to parse server response: {}", res);
            Err(Box::new(e))
        }
    }
}

pub fn syno_list(cfg: &Config, s: &Session)
                 -> Result<Vec<Task>, Box<dyn error::Error>> {
    let mut url = Url::parse(&cfg.url)?
        .join("/webapi/DownloadStation/task.cgi")?;
    url.query_pairs_mut()
        .clear()
        .append_pair("api", "SYNO.DownloadStation.Task")
        .append_pair("version", "2")
        .append_pair("method", "list")
        .append_pair("additional", "transfer")
        .append_pair("_sid", &s.sid);

    let res = syno_do(&url, cfg)?;

    let json = serde_json::from_str::<TaskListResponse>(&res)?;

    let iter = json.data.tasks.iter().map(|t|
        Task {
            id: String::from(&t.id),
            title: String::from(&t.title),
            status: String::from(&t.status),
            size: t.size,
            size_downloaded: t.additional.transfer.size_downloaded,
            size_uploaded: t.additional.transfer.size_uploaded,
            speed_download: t.additional.transfer.speed_download,
            speed_upload: t.additional.transfer.speed_upload,
            percent_dn: match t.size {
                0 => 0 as f64,
                _ => t.additional.transfer.size_downloaded as f64 / t.size as f64
            },
            percent_up: match t.size {
                0 => 0 as f64,
                _ => t.additional.transfer.size_uploaded as f64 / t.size as f64
            }
        });

    Ok(iter.rev().collect())
}

pub fn syno_download(cfg: &Config, s: &Session, remote: &String)
                     -> Result<(), Box<dyn error::Error>> {
    let mut url = Url::parse(&cfg.url)?
        .join("/webapi/DownloadStation/task.cgi")?;
    url.query_pairs_mut()
        .clear()
        .append_pair("api", "SYNO.DownloadStation.Task")
        .append_pair("version", "2")
        .append_pair("method", "create")
        .append_pair("uri", remote)
        .append_pair("_sid", &s.sid);

    syno_do(&url, cfg)?;
    Ok(())
}

pub fn syno_delete(cfg: &Config, s: &Session, t: &Task)
                   -> Result<(), Box<dyn error::Error>> {
    let mut url = Url::parse(&cfg.url)?
        .join("/webapi/DownloadStation/task.cgi")?;
    url.query_pairs_mut()
        .clear()
        .append_pair("api", "SYNO.DownloadStation.Task")
        .append_pair("version", "1")
        .append_pair("method", "delete")
        .append_pair("id", &t.id)
        .append_pair("_sid", &s.sid);

    syno_do(&url, cfg)?;
    Ok(())
}

pub fn syno_logout(cfg: &Config, s: &Session)
                   -> Result<(), Box<dyn error::Error>> {
    let mut url = Url::parse(&cfg.url)?
        .join("/webapi/auth.cgi")?;
    url.query_pairs_mut()
        .clear()
        .append_pair("api", "SYNO.API.Auth")
        .append_pair("version", "1")
        .append_pair("method", "logout")
        .append_pair("session", "DownloadStation")
        .append_pair("_sid", &s.sid);

    syno_do(&url, cfg)?;
    Ok(())
}
