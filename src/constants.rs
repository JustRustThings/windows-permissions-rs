use winapi::um::accctrl::*;
use winapi::um::winnt::*;

/// Create an enum from a list of constants. Generated enums get a method
/// `from_raw` that allows them to be converted from a value.
macro_rules! constant_enum {
    ( $name:ident; $int:ident; $( $item:ident),* ) => {
        #[derive(Debug, PartialEq)]
        #[allow(non_camel_case_types)]
        #[repr(C)]
        pub enum $name {

        $(
            $item = $item as isize,
        )*

        }

        impl $name {
            pub fn from_raw(raw: $int) -> Option<Self> {
                match raw {
                    $( $item => Some($name::$item), )*
                    _ => None,
                }
            }
        }
    }
}

constant_enum!(TrusteeForm; u32;
               TRUSTEE_IS_SID,
               TRUSTEE_IS_NAME,
               TRUSTEE_BAD_FORM,
               TRUSTEE_IS_OBJECTS_AND_SID,
               TRUSTEE_IS_OBJECTS_AND_NAME);

constant_enum!(TrusteeType; u32;
              TRUSTEE_IS_UNKNOWN,
              TRUSTEE_IS_USER,
              TRUSTEE_IS_GROUP,
              TRUSTEE_IS_DOMAIN,
              TRUSTEE_IS_ALIAS,
              TRUSTEE_IS_WELL_KNOWN_GROUP,
              TRUSTEE_IS_DELETED,
              TRUSTEE_IS_INVALID,
              TRUSTEE_IS_COMPUTER);

constant_enum!(MultipleTrusteeOperation; u32;
               NO_MULTIPLE_TRUSTEE,
               TRUSTEE_IS_IMPERSONATE);

constant_enum!(SeObjectType; u32;
               SE_UNKNOWN_OBJECT_TYPE,
               SE_FILE_OBJECT,
               SE_SERVICE,
               SE_PRINTER,
               SE_REGISTRY_KEY,
               SE_LMSHARE,
               SE_KERNEL_OBJECT,
               SE_WINDOW_OBJECT,
               SE_DS_OBJECT,
               SE_DS_OBJECT_ALL,
               SE_PROVIDER_DEFINED_OBJECT,
               SE_WMIGUID_OBJECT,
               SE_REGISTRY_WOW64_32KEY,
               SE_REGISTRY_WOW64_64KEY);

bitflags! {
    pub struct SecurityInformation: u32 {
        const Attribute = ATTRIBUTE_SECURITY_INFORMATION;
        const Backup = BACKUP_SECURITY_INFORMATION;
        const Dacl = DACL_SECURITY_INFORMATION;
        const Group = GROUP_SECURITY_INFORMATION;
        const Label = LABEL_SECURITY_INFORMATION;
        const Owner = OWNER_SECURITY_INFORMATION;
        const ProtectedDacl = PROTECTED_DACL_SECURITY_INFORMATION;
        const ProtectedSacl = PROTECTED_SACL_SECURITY_INFORMATION;
        const Sacl = SACL_SECURITY_INFORMATION;
        const Scope = SCOPE_SECURITY_INFORMATION;
        const UnprotectedDacl = UNPROTECTED_DACL_SECURITY_INFORMATION;
        const UnprotectedSacl = UNPROTECTED_SACL_SECURITY_INFORMATION;
    }
}

bitflags! {
    pub struct AccessRights: u32 {
        // All
        const All = 0xFFFF_FFFF;

        // Bits 31-28: Generic rights
        const GenericRead = GENERIC_READ;
        const GenericWrite = GENERIC_WRITE;
        const GenericExecute = GENERIC_EXECUTE;
        const GenericAll = GENERIC_ALL;

        // Bits 27-25: Reserved

        // Bit 24: Access system security
        const AccessSystemSecurity = ACCESS_SYSTEM_SECURITY;

        // Bits 23-16: Standard access rights
        const Delete = DELETE;
        const ReadControl = READ_CONTROL;
        const WriteDac = WRITE_DAC;
        const WriteOwner = WRITE_OWNER;
        const Synchronize = SYNCHRONIZE;
        const StandardRightsRequired = STANDARD_RIGHTS_REQUIRED;
        const StandardRightsRead = STANDARD_RIGHTS_READ;
        const StandardRightsWrite = STANDARD_RIGHTS_WRITE;
        const StandardRightsExecute = STANDARD_RIGHTS_EXECUTE;
        const StartardRightsAll = STANDARD_RIGHTS_ALL;

        // Object-specific access rights
        const SpecificRightsAll = SPECIFIC_RIGHTS_ALL;
        const Bit0  = 1 << 00;
        const Bit1  = 1 << 01;
        const Bit2  = 1 << 02;
        const Bit3  = 1 << 03;
        const Bit4  = 1 << 04;
        const Bit5  = 1 << 05;
        const Bit6  = 1 << 06;
        const Bit7  = 1 << 07;
        const Bit8  = 1 << 08;
        const Bit9  = 1 << 09;
        const Bit10 = 1 << 10;
        const Bit11 = 1 << 11;
        const Bit12 = 1 << 12;
        const Bit13 = 1 << 13;
        const Bit14 = 1 << 14;
        const Bit15 = 1 << 15;
    }
}

#[cfg(test)]
mod test {
    const A: u8 = 5;
    const B: u8 = 10;
    const C: u8 = 15;
    const INVALID: u8 = 100;

    constant_enum!(TestEnum; u8; A, B, C);

    #[test]
    fn constant_enum_works() {
        let enum_a = TestEnum::A;
        let enum_b = TestEnum::B;
        let enum_c = TestEnum::C;

        assert_eq!(None, TestEnum::from_raw(INVALID));
        assert_eq!(enum_a, TestEnum::from_raw(A).unwrap());
        assert_eq!(enum_b, TestEnum::from_raw(B).unwrap());
        assert_eq!(enum_c, TestEnum::from_raw(C).unwrap());
    }
}