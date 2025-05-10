use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;

use super::*;
use super::binfmt::{RiteBinaryHeader, SectionMiscHeader, SectionIrepHeader, IrepRecord};

fn u16_as_be_bytes(value: u16) -> [u8; 2] {
    unsafe { std::mem::transmute(value.to_be()) }
}

fn u32_as_be_bytes(value: u32) -> [u8; 4] {
    unsafe { std::mem::transmute(value.to_be()) }
}

fn sym_as_bytes(values: &HashMap<usize, String>) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&u16_as_be_bytes(values.len() as u16));

    for idx in 0..values.len() {
        bytes.extend_from_slice(&u16_as_be_bytes(values.get(&idx).unwrap().len() as u16));
        bytes.extend_from_slice(values.get(&idx).unwrap().as_bytes());
        bytes.push(0);
    }
    bytes
}

fn strings_as_pool_bytes(values: &HashMap<usize, String>) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&u16_as_be_bytes(values.len() as u16));
    for idx in 0..values.len() {
        bytes.push(0); // IREP_TT_STR
        bytes.extend_from_slice(&u16_as_be_bytes(values.get(&idx).unwrap().len() as u16));
        bytes.extend_from_slice(values.get(&idx).unwrap().as_bytes());
        bytes.push(0);
    }
    bytes
}

pub struct RitePacker {
    pub buf: Vec<u8>,
}

impl RitePacker {
    pub fn new() -> Self {
        RitePacker {
            buf: Vec::new(),
        }
    }

    pub fn pack(&mut self, reps: &[Rc<RefCell<transformer::IrepBase>>]) -> Result<(), String> {
        let mut binheader = RiteBinaryHeader::default();
        binheader.ident = *b"RITE";
        binheader.major_version = *b"03";
        binheader.minor_version = *b"00";
        binheader.compiler_name = *b"LUNR";
        binheader.compiler_version = *b"0000";
        // fill in the size field lator
        let mut binsize = size_of::<RiteBinaryHeader>();

        let mut irepheader = SectionIrepHeader::default();
        irepheader.ident = *b"IREP";
        irepheader.rite_version = *b"0300";
        // fill in the size field lator
        let mut secsize = size_of::<SectionIrepHeader>();

        let mut ireps = Vec::new();
        for rep in reps {
            let mut irep = IrepRecord::default();
            irep.nlocals = u16_as_be_bytes(rep.borrow().locals as u16);
            irep.nregs = u16_as_be_bytes(rep.borrow().regs as u16);
            irep.rlen = u16_as_be_bytes(rep.borrow().rep_len as u16);
            irep.clen = u16_as_be_bytes(rep.borrow().chandlers as u16);
            // fill in the size, ilen field lator
            let mut insn: Vec<u8> = Vec::new();

            dbg!(&rep.borrow().insn);
            for i in rep.borrow().insn.iter() {
                let mut insn_bytes = i.to_bytes_vec();
                insn.append(&mut insn_bytes);
            }
            irep.ilen = u32_as_be_bytes(insn.len() as u32);

            let syms = sym_as_bytes(&rep.borrow().syms);
            let pool = strings_as_pool_bytes(&rep.borrow().pool);

            let size = size_of::<IrepRecord>() + insn.len() + syms.len() + pool.len();
            irep.size = u32_as_be_bytes(size as u32);
            secsize += size;

            ireps.push((irep, insn, pool, syms));
        }
        irepheader.size = u32_as_be_bytes(secsize as u32);
        binsize += secsize;


        let mut endsection = SectionMiscHeader::default();
        endsection.ident = *b"END\0";
        endsection.size = u32_as_be_bytes(size_of::<SectionMiscHeader>() as u32);
        binsize += size_of::<SectionMiscHeader>();

        binheader.size = u32_as_be_bytes(binsize as u32);

        self.buf.extend_from_slice(unsafe { plain::as_bytes(&binheader) });
        self.buf.extend_from_slice(unsafe { plain::as_bytes(&irepheader) });
        for (irep, insn, pool, syms) in ireps {
            self.buf.extend_from_slice(unsafe { plain::as_bytes(&irep) });
            self.buf.extend_from_slice(&insn);
            self.buf.extend_from_slice(&pool);
            self.buf.extend_from_slice(&syms);
        }
        self.buf.extend_from_slice(unsafe { plain::as_bytes(&endsection) });

        Ok(())
    }

    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(filename)?;
        file.write_all(&self.buf)?;
        Ok(())
    }
}