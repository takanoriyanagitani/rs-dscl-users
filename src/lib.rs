use std::io;

use std::process::Command;
use std::process::Stdio;

use io::BufRead;
use io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct User {
    pub name: String,
    pub id: i32,
}

pub const DSCL_NAME_DEFAULT: &str = "dscl";
pub const DSCL_DATASOURCE_DEFAULT: &str = ".";
pub const DSCL_SUBCMD_DEFAULT: &str = "-list";
pub const DSCL_PATH2USER_DEFAULT: &str = "/Users";
pub const DSCL_KEY4ID_DEFAULT: &str = "UniqueID";

pub struct BasicCmd {
    pub dsclname: String,
    pub datasource: String,
    pub subcmd: String,
    pub path2user: String,
    pub key4id: String,
}

impl BasicCmd {
    pub fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.dsclname);
        cmd.arg(&self.datasource)
            .arg(&self.subcmd)
            .arg(&self.path2user)
            .arg(&self.key4id);
        cmd
    }

    pub fn to_stdout_lines(
        &self,
    ) -> Result<impl Iterator<Item = Result<String, io::Error>>, io::Error> {
        let mut cmd = self.to_command();
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        let mut child = cmd.spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or("no stdout exist")
            .map_err(io::Error::other)?;

        let reader = BufReader::new(stdout);
        Ok(reader.lines())
    }

    pub fn to_users(&self) -> Result<impl Iterator<Item = Result<User, io::Error>>, io::Error> {
        let lines = self.to_stdout_lines()?;
        Ok(lines2users(lines))
    }
}

impl Default for BasicCmd {
    fn default() -> Self {
        Self {
            dsclname: DSCL_NAME_DEFAULT.to_string(),
            datasource: DSCL_DATASOURCE_DEFAULT.to_string(),
            subcmd: DSCL_SUBCMD_DEFAULT.to_string(),
            path2user: DSCL_PATH2USER_DEFAULT.to_string(),
            key4id: DSCL_KEY4ID_DEFAULT.to_string(),
        }
    }
}

pub fn parse_line(line: &str) -> Result<User, &'static str> {
    let mut parts = line.split_whitespace();

    let name = parts.next().ok_or("missing user name")?;

    let id_str = parts.next().ok_or("missing user id")?;
    let id: i32 = id_str.parse().map_err(|_| "invalid user id")?;

    Ok(User {
        name: name.to_string(),
        id,
    })
}

pub fn lines2users<I>(lines: I) -> impl Iterator<Item = Result<User, io::Error>>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    lines.map(|res_line| {
        res_line.and_then(|line| {
            parse_line(&line).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
    })
}
