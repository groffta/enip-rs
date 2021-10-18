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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
