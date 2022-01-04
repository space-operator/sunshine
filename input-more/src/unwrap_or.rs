#[macro_export]
macro_rules! unwrap_or_else {
    ($value:expr, $err:ident => $or_else:expr) => {
        match $value {
            Ok(ok) => ok,
            Err($err) => $or_else,
        }
    };
    ($value:expr, $or_else:expr) => {
        match $value {
            Some(some) => some,
            None => $or_else,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_continue {
    ($($label:lifetime,)? $value:expr) => {
        match $value {
            Some(some) => some,
            None => continue $($label)?,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_break {
    ($($label:lifetime,)? $value:expr) => {
        match $value {
            Some(some) => some,
            None => break $($label)?,
        }
    };
    ($($label:lifetime,)? $value:expr, $err:ident => $or_else:expr) => {
        match $value {
            Ok(ok) => ok,
            Err($err) => break $($label)? $or_else,
        }
    };
    ($($label:lifetime,)? $value:expr, $or_else:expr) => {
        match $value {
            Some(some) => some,
            None => break $($label)? $or_else,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_return {
    ($value:expr) => {
        match $value {
            Some(some) => some,
            None => return,
        }
    };
    ($value:expr, $or_else:expr) => {
        match $value {
            Some(some) => some,
            None => return $or_else,
        }
    };
}
