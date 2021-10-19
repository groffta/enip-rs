use primitive_enum::primitive_enum;

primitive_enum! { ObjectClass u16;
    /// General Use Objects
    Identity                 = 0x01,
    MessageRouter            = 0x02,
    Assembly                 = 0x04,
    Connection               = 0x05, 
    ConnectionManager        = 0x06,
    Register                 = 0x07,
    Parameter                = 0x0F,
    ParameterGroup           = 0x10,
    AcknowledgeHandler       = 0x2B,
    Selection                = 0x2E,
    File                     = 0x37,
    OriginatorCollectionList = 0x45,
    ConnectionConfiguration  = 0xF3,
    Port                     = 0xF4,

    /// Application Specific Objects
    AcDcDrive = 0x2A,
    

}


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