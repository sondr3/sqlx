use byteorder::LittleEndian;

use crate::io::Buf;
use crate::mysql::io::BufExt;
use crate::mysql::protocol::Decode;

// https://dev.mysql.com/doc/dev/mysql-server/8.0.12/page_protocol_com_stmt_prepare.html#sect_protocol_com_stmt_prepare_response_ok
#[derive(Debug)]
pub struct ComStmtPrepareOk {
    pub statement_id: u32,

    /// Number of columns in the returned result set (or 0 if statement does not return result set).
    pub columns: u16,

    /// Number of prepared statement parameters ('?' placeholders).
    pub params: u16,

    /// Number of warnings.
    pub warnings: u16,
}

impl Decode for ComStmtPrepareOk {
    fn decode(mut buf: &[u8]) -> crate::Result<Self> {
        let header = buf.get_u8()?;

        if header != 0x00 {
            return Err(protocol_err!(
                "expected COM_STMT_PREPARE_OK (0x00); received 0x{:X}",
                header
            ))?;
        }

        let statement_id = buf.get_u32::<LittleEndian>()?;
        let columns = buf.get_u16::<LittleEndian>()?;
        let params = buf.get_u16::<LittleEndian>()?;

        // -not used- : string<1>
        buf.advance(1);

        let warnings = buf.get_u16::<LittleEndian>()?;

        Ok(Self {
            statement_id,
            columns,
            params,
            warnings,
        })
    }
}
