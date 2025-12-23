use std::io::{Read, Write};

//  Packet Receive
use super::{YmodemControlCode, YmodemError};

pub fn wait_msg<T>(port: &mut T) -> u8
where
    T: Read + Write,
{
    let mut response = [0; 1];
    port.read_exact(&mut response).unwrap();
    response[0]
}
pub fn wait_for_ack<T>(port: &mut T) -> Result<(), YmodemError>
where
    T: Read + Write,
{
    let response = wait_msg(port);
    if response == YmodemControlCode::Ack as u8 {
        Ok(())
    } else if response == YmodemControlCode::Nak as u8 {
        Err(YmodemError::RequestReSend)
    } else if response == YmodemControlCode::Can as u8 {
        Err(YmodemError::SendFailed)
    } else {
        Err(YmodemError::InvalidResponse)
    }
}
#[cfg(feature = "async")]
pub mod r#async {
    use super::{YmodemControlCode, YmodemError};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    pub async fn wait_msg<T>(port: &mut T) -> u8
    where
        T: AsyncReadExt + AsyncWriteExt + Unpin,
    {
        let mut response = [0; 1];
        port.read_exact(&mut response).await.unwrap();
        response[0]
    }
    pub async fn wait_for_ack<T>(port: &mut T) -> Result<(), YmodemError>
    where
        T: AsyncReadExt + AsyncWriteExt + Unpin,
    {
        let response = wait_msg(port).await;
        if response == YmodemControlCode::Ack as u8 {
            Ok(())
        } else if response == YmodemControlCode::Nak as u8 {
            Err(YmodemError::RequestReSend)
        } else if response == YmodemControlCode::Can as u8 {
            Err(YmodemError::SendFailed)
        } else {
            Err(YmodemError::InvalidResponse)
        }
    }
}
