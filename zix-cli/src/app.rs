use std::fs;

use zix_core::entry::create;
use zix_core::entry::{Entry, create::filter_entries};
use crate::parser::{parse, Opti};
use crate::ref_command::{help, version, NAME, VERSION};

#[cfg(windows)]
use glob::glob;

#[derive(Clone, Debug)]
pub struct App {
    pub entries: Vec<Vec<Entry>>,
    pub name: &'static str,
    pub version: &'static str,
    pub options: Vec<Opti>,
}

impl App {
    pub fn init() -> Option<App> {
        let mut app = App {
            entries: Vec::new(),
            name: &NAME,
            version: &VERSION,
            options: Vec::new()
        };

        let (options, values) = parse();
        for op in options {
            match op.as_str() {
                "--help" | "-?" => {
                    help();
                    return None;
                }
                "--headers" | "-h" => app.options.push(Opti::Headers),
                "--version" | "-v" => {
                    version();
                    return None;
                }
                "--all" | "-a" => app.options.push(Opti::All),
                "--list" | "-l" => app.options.push(Opti::List),
                "--tree" | "-t" => app.options.push(Opti::Tree),
                "--grid" | "-g" => app.options.push(Opti::Grid),
                _ => {
                    println!(
                        "'{}' is not a valid option\nType 'zx --help' for more information.",
                        op
                    );
                    return None;
                }
            }
        }

        if app.options.is_empty()   {
            app.options.push(Opti::Grid);
        }

        #[cfg(unix)]
        {
            for val in values.iter() {
                let mut entries: Vec<Entry> = Vec::new();

                match fs::metadata(val) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            let path = std::path::PathBuf::from(val);
                            if let Some(entry) = create::filter_dir(&path) {
                                entries.push(entry);
                            }
                        } else if metadata.is_dir() {
                            if let Ok(dir) = fs::read_dir(val) {
                                let unfiltered_entries: Vec<Entry> = dir
                                    .filter_map(Result::ok)
                                    .filter_map(|path| create::dir(&path, &app.options[0]))
                                    .collect();
                                entries.extend(filter_entries(&unfiltered_entries, &app.options[0]));
                            } else {
                                println!("Cannot read directory: {}", val);
                            }
                        }
                    }
                    Err(_) => println!("Cannot access path: {}", val),
                }

                app.entries.push(entries);
            }
        }

        #[cfg(windows)]
        {
            for val in values.iter() {
                let mut entries: Vec<Entry> = Vec::new();

                if val.contains('*') {
                    if let Ok(paths) = glob(&val) {
                        entries.extend(
                            paths
                                .filter_map(Result::ok)
                                .filter_map(|path| create::filter_dir(&path)),
                        );
                    } else {
                        println!("Error interpreting the pattern: {}", val);
                    }
                } else {
                    if let Ok(dir) = fs::read_dir::<&String>(&val) {
                        let unfiltered_entries: Vec<Entry> = dir
                            .filter_map(Result::ok)
                            .filter_map(|path| create::dir(&path, &app.options[0]))
                            .collect();
                        if app.options.is_empty() {
                            entries.extend(unfiltered_entries);
                        } else {
                            entries.extend(filter_entries(&unfiltered_entries, &app.options[0]));
                        }
                    } else {
                        continue;
                    };
                }
                app.entries.push(entries);
            }
        }

        Some(app)
    }
}

// use std::fs;

// use zix_core::entry::create;
// use zix_core::entry::{Entry, create::dir};
// use crate::parser::{parse, Opti};
// use crate::ref_command::*;

// #[cfg(windows)]
// use glob::glob;


// #[derive(Clone, Debug)]
// pub struct App {
//     pub entries: Vec<Vec<Entry>>,
//     pub name: &'static str,
//     pub version: &'static str,
//     pub options: Vec<Opti>

// }

// impl App    {
//     pub fn init() -> Option<App>    {
//         let mut app = App {
//             entries: Vec::new(),
//             name: &NAME,
//             version: &VERSION,
//             options: Vec::new()
//         };

//         let (options, values) = parse();
//         for op in options   {
//             match op.as_str()    {
//                 "--help" | "-?" => { help(); return None},
//                 "--headers" | "-h" => app.options.push(Opti::Headers),
//                 "--version" | "-v" => { version(); return None},
//                 "--all" | "-a" => app.options.push(Opti::All),
//                 "--list" | "-l" => app.options.push(Opti::List),
//                 "--tree" | "-t" => app.options.push(Opti::Tree),
//                 _ => {
//                     println!(
//                         "'{}' is not a valid option\nType 'zx --help' for more information.",
//                         op
//                     );
//                     return None
//                 }
//             }
//         }


//         #[cfg(unix)] {
//             for val in values.iter() {
//                 let mut entries: Vec<Entry> = Vec::new();

//                 match fs::metadata(val) {
//                     Ok(metadata) => {
//                         if metadata.is_file() {
//                             let path = std::path::PathBuf::from(val);
//                             if let Some(entry) = create::filter_dir(&path) {
//                                 entries.push(entry);
//                             }
//                         } else if metadata.is_dir() {
//                             if let Ok(dir) = fs::read_dir(val) {
//                                 entries.extend(
//                                     dir
//                                         .filter_map(Result::ok)
//                                         .filter_map(|path| create::dir(&path, &app.options))
//                                 );
//                             } else {
//                                 println!("Cannot read directory: {}", val);
//                             }
//                         }
//                     },
//                     Err(_) => println!("Cannot access path: {}", val)
//                 }

//                 app.entries.push(entries)
//             }
//         }

//         #[cfg(windows)] {
//         for val in values.iter()   {
//             let mut entries: Vec<Entry> = Vec::new();

//                 if val.contains('*')    {
//                     if let Ok(paths) = glob(&val) {
//                         entries.extend(
//                             paths
//                                 .filter_map(Result::ok)
//                                 .filter_map(|path| create::filter_dir(&path))
//                         );
//                     } else {
//                         println!("Error interpreting the pattern: {}", val);
//                     }
//                 } else {
//                     if let Ok(dir) = fs::read_dir::<&String>(&val)   {
//                         entries.extend(
//                             dir
//                                     .filter_map(Result::ok)
//                                     .filter_map(
//                                         |path|
//                                         create::dir(&path, &app.options)
//                                     )
//                         );
//                    } else {
//                        continue;
//                    };
//                 }
//                 app.entries.push(entries);
//             }
//         }

//         Some(app)
//     }
// }
