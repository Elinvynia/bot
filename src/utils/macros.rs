#[macro_export]
macro_rules! error_return {
    ($e:expr) => {
        match $e {
            Ok(o) => o,
            Err(_) => return,
        }
    };
}

#[macro_export]
macro_rules! none_return {
    ($e:expr) => {
        match $e {
            Some(s) => s,
            None => return,
        }
    };
}

#[macro_export]
macro_rules! error_return_ok {
    ($e:expr) => {
        match $e {
            Ok(o) => o,
            Err(_) => return Ok(()),
        }
    };
}

#[macro_export]
macro_rules! none_return_ok {
    ($e:expr) => {
        match $e {
            Some(s) => s,
            None => return Ok(()),
        }
    };
}

#[macro_export]
macro_rules! sql_block {
    ($e:expr) => {
        tokio::task::block_in_place(|| -> anyhow::Result<()> {
            $e;
            Ok(())
        });
    };
}
