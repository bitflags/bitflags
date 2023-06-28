#[cfg(test)]
mod tests {
    use binrw::BinReaderExt;

    pub mod little_endian {
        bitflags! {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, binrw::BinRead)]
            #[br(little)]
            pub struct ACL: u32 {
                const Read = 0b0000_0001;
                const Write = 0b0000_0010;
                const Execute = 0b0000_0100;
                const Traverse = 0b0000_1000;

                //const All = Self::Read | Self::Write | Self::Execute | Self::Traverse;
            }
        }
    }

    pub mod big_endian {
        bitflags! {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, binrw::BinRead)]
            #[br(big)]
            pub struct ACL: u32 {
                const Read = 0b0000_0001;
                const Write = 0b0000_0010;
                const Execute = 0b0000_0100;
                const Traverse = 0b0000_1000;

               // const All = Self::Read | Self::Write | Self::Execute | Self::Traverse;
            }
        }
    }

    #[test]
    fn test_binreader_ext_little_endian() {
        use little_endian::ACL;
        use std::io::Cursor;
        let flags = ACL::Read | ACL::Execute;
        println!("{:?}", flags.bits().to_le_bytes());
        let buf: [u8; 4] = [0x05, 0x00, 0x00, 0x00];
        let mut cursor = Cursor::new(&buf[..]);
        let acl: ACL = cursor.read_ne().unwrap();
        assert_eq!(acl, ACL::Read | ACL::Execute);
    }

    #[test]
    fn test_binreader_ext_big_endian() {
        use big_endian::ACL;
        use std::io::Cursor;
        let buf: [u8; 4] = [0x00, 0x00, 0x00, 0x05];
        let mut cursor = Cursor::new(&buf[..]);
        let acl: ACL = cursor.read_ne().unwrap();
        assert_eq!(acl, ACL::Read | ACL::Execute);
    }
}
