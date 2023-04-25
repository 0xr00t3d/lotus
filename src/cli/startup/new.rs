use crate::{
    cli::{
        args::new::ScriptType,
        default_scripts::{
            write_file, CVE_EXAMPLE, FUZZ_EXAMPLE, SERVICE_EXAMPLE,
        },
        errors::CliErrors,
    },
    show_msg, MessageLevel,
};
use std::path::PathBuf;

pub fn new_args(scan_type: ScriptType, file_name: PathBuf) {
    let script_code = match scan_type {
        ScriptType::Fuzz => FUZZ_EXAMPLE,
        ScriptType::CVE => CVE_EXAMPLE,
        ScriptType::SERVICE => SERVICE_EXAMPLE,
        ScriptType::NotSupported => "",
    };
    let write_script_file = write_file(file_name, script_code);
    if let Err(CliErrors::FileExists) = write_script_file {
        show_msg(
            "File Exists, cannot overwrite it, please rename/remove it or try another name",
            MessageLevel::Error,
        );
    } else if let Err(CliErrors::WritingError) = write_script_file {
        show_msg(
            CliErrors::WritingError.to_string().as_str(),
            MessageLevel::Error,
        );
    } else {
        show_msg(
            "A copy of the Example file has been created",
            MessageLevel::Info,
        );
    }
    show_msg("Exit ..", MessageLevel::Info);
}
