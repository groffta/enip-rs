
/**
 * Request Addressing:
 * 
 * - Node Address (SocketAddr)
 * - ClassID (int)
 * - InstanceID (int)
 * - AttributeID (int)
 * - ServiceCode (int) 
 */

/**
 * Public ClassIDs: 0x0000 - 0x0063, 0x00F0 - 0x02FF
 * Vendor ClassIDs: 0x0064 - 0x00C7, 0x0300 - 0x04FF
 * Public InstanceIDs: 0x0001 - 0x0063, 0x00C8 - 0x02FF
 * Vendor InstanceIDs: 0x0064 - 0x00C7, 0x0300 - 0x04FF
 * Public AttributeIDs: 0x0000 - 0x0063, 0x0100 - 0x02FF, 0x0500 - 0x08FF
 * Vendor AttributeIDs: 0x0064 - 0x00C7, 0x0300 - 0x04FF, 0x0900 - 0x0CFF
 */

use primitive_enum::primitive_enum;
primitive_enum! { ServiceCode u16 ;
    NoCode                 = 0x00,
    GetAttributesAll       = 0x01,
    SetAttributesAll       = 0x02,
    GetAttributeList       = 0x03,
    SetAttributeList       = 0x04,
    Reset                  = 0x05,
    Start                  = 0x06,
    Stop                   = 0x07,
    Create                 = 0x08,
    Delete                 = 0x09,
    MultipleServicePacket  = 0x0A,
    ApplyAttributes        = 0x0D,
    GetAttributeSingle     = 0x0E,
    SetAttributeSingle     = 0x10,
    FindNextObjectInstance = 0x11,
    Restore                = 0x15,
    Save                   = 0x16,
    Nop                    = 0x17,
    GetMember              = 0x18,
    SetMember              = 0x19,
    InsertMember           = 0x1A,
    RemoveMember           = 0x1B,
    GroupSync              = 0x1C,
    ForwardClose           = 0x4E,
    UnconnectedSend        = 0x52,
    ForwardOpen            = 0x54,
    LargeForwardOpen       = 0x5B,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
