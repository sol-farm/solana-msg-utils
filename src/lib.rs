//! Provides macros used for logging messages on-chain, with
//! the ability to provide "trace level" logging messages. 
//! 
//! In addition to this, the trace level logging messages utilizes
//! stack based formatting if the message size is less than 512 bytes
//! for maximal compute units consumption efficiency

/// msg_panic! is a wrapper around the `msg!` and `panic!`
/// macros used to log an error message, and panic in bpf environments
/// which do not actually show a message emitted by a panic macro
#[macro_export]
macro_rules! msg_panic {
    ($($args:tt)+) => {{
        // the actual error message
        solana_program::msg!("RUNTIME ERROR: {}", format_args!($($args)*));
        // panic to indicate the line the error came from
        // but panicking doesn't log anything in bpf target for solana programs
        panic!("RUNTIME ERROR: {}", format_args!($($args)*));
    }};
}

#[macro_export]
macro_rules! sum {
    // this delcares an exrpession i think :shrug:
    // todo(): explain this more
    ($($args:expr),*) => {{
        let mut result = 0;
        $(
            // combine the size of each value
            result = result + $args.len();
        )*
        // return the size of all arguments
        result
    }}
}

/// msg_trace! is a wrapper around the `msg!` macro, that faciliates logging trace
/// level logs, which include the file and line number from where the message was emitted.
///
/// if the total msg size is less than or equal to 512 bytes, then `arrform!` is used for
/// the optimize (heap-less) message formatting. messages larger than 512 bytes use the traditional `format!`.
#[macro_export]
macro_rules! msg_trace {
    ($($args:tt)+) => {
        // get the filename that produce the log, it's less info than the fille path
        // but it saves pace, an when paired with the line number is more than enough debug
        let file_name = std::path::Path::new(file!()).file_name().unwrap().to_string_lossy();
        let input_sizes = sum!($($args)*);
        solana_program::msg!("input sizes {}", input_sizes);
        if input_sizes > 512 {
            // slow path
            solana_program::msg!("{}", format!("'{}', '{}:{}", format!($($args)*), file_name, line!()).as_str());
        } else {
            use tulip_arrform::{arrform, ArrForm};
            let file_info = arrform!(256, "{}:{}", file_name, line!());
            let msg_part = arrform!(512, $($args)*);
            solana_program::msg!("'{}', {}", msg_part.as_str(), file_info.as_str());
        }
    };
}


#[cfg(test)]
mod test {
    use super::*;
    use solana_program::msg;
    #[test]
    fn test_trace() {
        {
            msg_trace!("hello world this is {}", "very big message");
        }
        {
            let mut msg_str = String::new();
            for _ in 0..550 {
                msg_str.push('a');
            }
            msg_trace!("{}", msg_str);
        }
    }
    #[test]
    #[should_panic(expected = "RUNTIME ERROR: too many keks")]
    fn test_msg_panic() {
        msg_panic!("too many keks");
    }
}
