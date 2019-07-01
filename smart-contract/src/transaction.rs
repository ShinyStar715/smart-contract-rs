use crate::payload::{Readable, Writeable};

#[repr(u8)]
pub enum TransactionTag {
    Nop,
    Transfer,
    Contract,
    Stake,
}

pub trait Transaction: Writeable + Readable {
    fn send_transaction(self) {
        let mut payload = vec![];
        self.write_to(&mut payload);

        unsafe {
            crate::sys::_send_transaction(self.tag() as u8, payload.as_ptr(), payload.len());
        }
    }

    fn tag(&self) -> TransactionTag;
}

#[derive(Default)]
pub struct Transfer {
    pub destination: [u8; 32],
    pub amount: u64,

    pub func_name: Vec<u8>,
    pub func_params: Vec<u8>,
}

impl Writeable for Transfer {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        self.destination.write_to(buffer);
        self.amount.write_to(buffer);

        if self.func_name.len() > 0 && self.func_params.len() > 0 {
            0u64.write_to(buffer); // Specify an empty gas limit.

            self.func_name.write_to(buffer);
            self.func_params.write_to(buffer);
        }
    }
}

impl Readable for Transfer {
    fn read_from(buffer: &[u8], pos: &mut u64) -> Transfer {
        let mut params = Transfer::default();

        params.destination = <[u8; 32]>::read_from(buffer, pos);
        params.amount = u64::read_from(buffer, pos);

        if *pos < buffer.len() as u64 {
            u64::read_from(buffer, pos); // Read gas limit.
        }

        if *pos < buffer.len() as u64 {
            params.func_name = Vec::<u8>::read_from(buffer, pos); // Read function name.
        }

        if *pos < buffer.len() as u64 {
            params.func_params = Vec::<u8>::read_from(buffer, pos) // Read function params.
        }

        params
    }
}

impl Transaction for Transfer {
    fn tag(&self) -> TransactionTag {
        TransactionTag::Transfer
    }
}

#[derive(Default)]
pub struct Contract {
    pub payload: Vec<u8>,
    pub code: Vec<u8>,
}

impl Writeable for Contract {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        0u64.write_to(buffer);
        self.payload.write_to(buffer);
        buffer.append(&mut self.code.clone());
    }
}

impl Readable for Contract {
    fn read_from(buffer: &[u8], pos: &mut u64) -> Contract {
        let mut params = Contract::default();

        params.payload = Vec::<u8>::read_from(buffer, pos);
        params.code = buffer[*pos as usize..].to_vec();

        *pos = buffer.len() as u64;

        params
    }
}
