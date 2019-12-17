// This mod cribbed from ructe.

use lazy_static::lazy_static;
use nom::ErrorKind;
use std::sync::Mutex;

macro_rules! err_str(
    ($msg:expr) => {{
        use crate::parse::errors::def_error;
        use nom::ErrorKind;
        lazy_static! {
            static ref ERR: ErrorKind = def_error($msg);
        }
        ERR.clone()
    }}
);

pub fn def_error(msg: &'static str) -> ErrorKind {
    let mut errors = ERRORS.lock().unwrap();
    let n = errors.len();
    errors.push(msg);
    ErrorKind::Custom(n as u32)
}

pub fn get_error(n: u32) -> Option<String> {
    match ERRORS.lock() {
        Ok(e) => e.get(n as usize).map(|s| s.to_string()),
        Err(_) => None,
    }
}

lazy_static! {
    static ref ERRORS: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
}

use std::fmt::Debug;
use std::io::Write;

use nom::types::CompleteStr;
use nom::{Context, Err};

pub fn show_errors<E, W>(out: &mut W, buf: &str, result: nom::IResult<CompleteStr, E>, prefix: &str)
where
    E: Debug,
    W: Write,
{
    match result {
        Ok(_) => (),
        Err(Err::Error(Context::Code(_before, e))) => {
            show_error(out, buf, 0, &format!("error {:?}", e), prefix);
        }
        Err(Err::Error(Context::List(mut v))) => {
            v.reverse();
            for (i, e) in v {
                let pos = buf.len() - i.len();
                show_error(out, buf, pos, &get_message(&e), prefix);
            }
        }
        Err(Err::Failure(Context::List(mut v))) => {
            v.reverse();
            for (i, e) in v {
                let pos = buf.len() - i.len();
                show_error(out, buf, pos, &get_message(&e), prefix);
            }
        }
        Err(Err::Failure(e)) => {
            show_error(out, buf, 0, &format!("failure {:?}", e), prefix);
        }
        Err(_) => show_error(out, buf, 0, "xyzzy", prefix),
    }
}

fn get_message(err: &ErrorKind) -> String {
    match err {
        &ErrorKind::Custom(n) => match get_error(n) {
            Some(msg) => msg,
            None => format!("Unknown error #{}", n),
        },
        err => format!("{:?}", err),
    }
}

fn show_error<W>(out: &mut W, buf: &str, pos: usize, msg: &str, prefix: &str)
where
    W: Write,
{
    let mut line_start = buf[0..pos].rsplitn(2, '\n');
    let _ = line_start.next();
    let line_start = line_start.next().map(|bytes| bytes.len() + 1).unwrap_or(0);
    let line = buf[line_start..].splitn(2, '\n').next().unwrap();
    let line_no = what_line(buf, line_start);
    let pos_in_line = buf[line_start..pos].chars().count() + 1;
    writeln!(
        out,
        "{prefix}{:>4}:{}\n\
         {prefix}     {:>pos$} {}",
        line_no,
        line,
        "^",
        msg,
        pos = pos_in_line,
        prefix = prefix,
    )
    .unwrap();
}

fn what_line(buf: &str, pos: usize) -> usize {
    1 + buf[0..pos].chars().filter(|c| *c == '\n').count()
}
