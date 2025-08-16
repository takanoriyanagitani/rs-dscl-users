use std::io;

use std::process::ExitCode;

use io::BufWriter;
use io::Write;

use rs_dscl_users::BasicCmd;
use rs_dscl_users::User;

fn bcmd_new() -> BasicCmd {
    BasicCmd::default()
}

fn bcmd2users(bcmd: &BasicCmd) -> Result<impl Iterator<Item = Result<User, io::Error>>, io::Error> {
    bcmd.to_users()
}

fn user2json2wtr<W>(wtr: &mut W, u: &User) -> Result<(), io::Error>
where
    W: Write,
{
    serde_json::to_writer(wtr.by_ref(), u).map_err(io::Error::other)
}

fn sub() -> Result<(), io::Error> {
    let bcmd: BasicCmd = bcmd_new();
    let users = bcmd2users(&bcmd)?;

    let stdout = io::stdout();
    let mut locked = stdout.lock();
    let mut handle = BufWriter::new(&mut locked);

    for user_res in users {
        let user = user_res?;
        user2json2wtr(&mut handle, &user)?;
        writeln!(handle)?;
    }

    handle.flush()?;
    drop(handle);
    locked.flush()?;

    Ok(())
}

fn main() -> ExitCode {
    match sub() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
