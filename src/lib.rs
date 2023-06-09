// Copyright (C) 2023 Fred Clausen

// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA

extern crate chrono;
extern crate env_logger;
extern crate log;

use chrono::Local;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Trait to setup logging
/// To initialize logging, call `enable_logging` on a u8
pub trait SetupLogging {
    fn set_logging_level(self) -> LevelFilter;
    fn enable_logging(&self);
}

/// Setup logging. 0 = Info, 1 = Debug, 2 = Trace

impl SetupLogging for u8 {
    fn set_logging_level(self) -> LevelFilter {
        match self {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            2..=u8::MAX => LevelFilter::Trace,
        }
    }

    /// Enable logging<br><br>
    /// The output is colored and looks like this:<br>
    /// \[INFO \]\[2021-08-22T15:49:01\]This is an info message<br>
    /// \[DEBUG\]\[2021-08-22T15:49:01\]This is a debug message<br>
    /// \[TRACE\]\[2021-08-22T15:49:01\]This is a trace message<br>
    /// \[ERROR\]\[2021-08-22T15:49:01\]This is an error message<br>
    /// \[WARN \]\[2021-08-22T15:49:01\]This is a warning message<br>
    /// \[OTHER\]\[2021-08-22T15:49:01\]This is a message with a different log level<br><br>
    /// The level field is colored and bold if the terminal supports it.<br>

    fn enable_logging(&self) {
        Builder::new()
            .format(|buf, record| {
                let mut level_style = buf.style();

                if record.level() == log::Level::Info {
                    level_style.set_color(Color::Green).set_bold(true);
                } else if record.level() == log::Level::Debug {
                    level_style.set_color(Color::Cyan).set_bold(true);
                } else if record.level() == log::Level::Trace {
                    level_style.set_color(Color::Magenta).set_bold(true);
                } else if record.level() == log::Level::Error {
                    level_style.set_color(Color::Red).set_bold(true);
                } else if record.level() == log::Level::Warn {
                    level_style.set_color(Color::Yellow).set_bold(true);
                } else {
                    level_style.set_color(Color::White).set_bold(true);
                }

                writeln!(
                    buf,
                    "[{}][{}]{}",
                    level_style.value(format!("{: <5}", record.level())),
                    Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    record.args()
                )
            })
            .filter(None, self.set_logging_level())
            .init();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_logging_level() {
        let info_level: u8 = 0;
        let debug_level: u8 = 1;
        let trace_level: u8 = 2;
        let stupid_levels: u8 = 255;
        let info_level_logging: LevelFilter = info_level.set_logging_level();
        let debug_level_logging: LevelFilter = debug_level.set_logging_level();
        let trace_level_logging: LevelFilter = trace_level.set_logging_level();
        let stupid_levels_logging: LevelFilter = stupid_levels.set_logging_level();
        assert_eq!(info_level_logging, LevelFilter::Info);
        assert_eq!(debug_level_logging, LevelFilter::Debug);
        assert_eq!(trace_level_logging, LevelFilter::Trace);
        assert_eq!(stupid_levels_logging, LevelFilter::Trace);
    }
}
