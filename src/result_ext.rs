use crate::err;

/// a result type equivalent to std::io::Result, but implements `Serialize` and `Deserialize`
pub type Result<T, E = super::Error> = std::result::Result<T, E>;

pub trait ResultExt<T> {
    fn map_not_found(self) -> Result<T>;
    fn map_permission_denied(self) -> Result<T>;
    fn map_conn_refused(self) -> Result<T>;
    fn map_conn_reset(self) -> Result<T>;
    fn map_conn_aborted(self) -> Result<T>;
    fn map_not_connected(self) -> Result<T>;
    fn map_in_use(self) -> Result<T>;
    fn map_addr_not_available(self) -> Result<T>;
    fn map_broken_pipe(self) -> Result<T>;
    fn map_already_exists(self) -> Result<T>;
    fn map_would_block(self) -> Result<T>;
    fn map_invalid_input(self) -> Result<T>;
    fn map_invalid_data(self) -> Result<T>;
    fn map_timeout(self) -> Result<T>;
    fn map_write_zero(self) -> Result<T>;
    fn map_interrupted(self) -> Result<T>;
    fn map_unsupported(self) -> Result<T>;
    fn map_unexpected_eof(self) -> Result<T>;
    fn map_out_of_memory(self) -> Result<T>;
    fn map_other(self) -> Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultExt<T> for Result<T, E> {
    #[inline]
    fn map_not_found(self) -> Result<T> {
        self.map_err(err!(@not_found))
    }
    #[inline]
    fn map_permission_denied(self) -> Result<T> {
        self.map_err(err!(@permission_denied))
    }
    #[inline]
    fn map_conn_refused(self) -> Result<T> {
        self.map_err(err!(@conn_refused))
    }
    #[inline]
    fn map_conn_reset(self) -> Result<T> {
        self.map_err(err!(@conn_reset))
    }
    #[inline]
    fn map_conn_aborted(self) -> Result<T> {
        self.map_err(err!(@conn_aborted))
    }
    #[inline]
    fn map_not_connected(self) -> Result<T> {
        self.map_err(err!(@not_connected))
    }
    #[inline]
    fn map_in_use(self) -> Result<T> {
        self.map_err(err!(@in_use))
    }
    #[inline]
    fn map_addr_not_available(self) -> Result<T> {
        self.map_err(err!(@addr_not_available))
    }
    #[inline]
    fn map_broken_pipe(self) -> Result<T> {
        self.map_err(err!(@broken_pipe))
    }
    #[inline]
    fn map_already_exists(self) -> Result<T> {
        self.map_err(err!(@already_exists))
    }
    #[inline]
    fn map_would_block(self) -> Result<T> {
        self.map_err(err!(@would_block))
    }
    #[inline]
    fn map_invalid_input(self) -> Result<T> {
        self.map_err(err!(@invalid_input))
    }
    #[inline]
    fn map_invalid_data(self) -> Result<T> {
        self.map_err(err!(@invalid_data))
    }
    #[inline]
    fn map_timeout(self) -> Result<T> {
        self.map_err(err!(@timeout))
    }
    #[inline]
    fn map_write_zero(self) -> Result<T> {
        self.map_err(err!(@write_zero))
    }
    #[inline]
    fn map_interrupted(self) -> Result<T> {
        self.map_err(err!(@interrupted))
    }
    #[inline]
    fn map_unsupported(self) -> Result<T> {
        self.map_err(err!(@unsupported))
    }
    #[inline]
    fn map_unexpected_eof(self) -> Result<T> {
        self.map_err(err!(@unexpected_eof))
    }
    #[inline]
    fn map_out_of_memory(self) -> Result<T> {
        self.map_err(err!(@out_of_memory))
    }
    #[inline]
    fn map_other(self) -> Result<T> {
        self.map_err(err!(@other))
    }
}
