use argh::FromArgs;
use clipboard::{ClipboardContext, ClipboardProvider};
use device_query::{DeviceQuery, DeviceState, Keycode};
use regex::Regex;
use std::{thread, time};
use winapi::um::winuser::MessageBeep;

/// Convert clipboard dates on Ctrl+Shift+C
#[derive(FromArgs)]
struct Args {
    /// enable debug output
    #[argh(switch, short = 'd')]
    debug: bool,
}

fn main() {
    let args: Args = argh::from_env();
    let debug = args.debug;

    let device_state = DeviceState::new();
    let mut prev_pressed = vec![];

    println!("Date clipboard converter running. Press Ctrl+Shift+Y to trigger conversion.");

    loop {
        let keys = device_state.get_keys();
        if keys.contains(&Keycode::LControl)
            && keys.contains(&Keycode::LShift)
            && keys.contains(&Keycode::Y)
            && prev_pressed != keys
        {
            match process_clipboard(debug) {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        prev_pressed = keys;
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn process_clipboard(debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let content = ctx.get_contents()?;

    let date_re = Regex::new(r"\b(\d{1,2})/(\d{1,2})/(\d{2,4})\b")?;
    let mut has_match = false;

    let converted = date_re.replace_all(&content, |caps: &regex::Captures| {
        has_match = true;
        format!("{}/{}/{}", &caps[2], &caps[1], &caps[3])
    });

    if has_match {
        if debug {
            println!(
                "Clipboard contains:\n{}\nConverted to:\n{}",
                content, converted
            );
        }
        // 🔔 Play notification sound
        unsafe {
            MessageBeep(winapi::um::winuser::MB_ICONINFORMATION);
        }
        ctx.set_contents(converted.to_string())?;
    } else if debug {
        println!("No valid DD/MM/YYYY dates found in clipboard.");
    }

    Ok(())
}
