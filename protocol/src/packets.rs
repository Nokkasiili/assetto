macro_rules! user_type {
    (WideString) => {
        String
    };
    (BytePrefixedVec <$inner:ident>) => {
        Vec<$inner>
    };
    (U16PrefixedVec <$inner:ident>) => {
        Vec<$inner>
    };
    ($typ:ty) => {
        $typ
    };
}

macro_rules! user_type_convert_to_writeable {
    (WideString, $e:expr) => {
        WideString::from($e)
    };
    (BytePrefixedVec <$inner:ident>, $e:expr) => {
        BytePrefixedVec::from($e.as_slice())
    };
    (U16PrefixedVec <$inner:ident>, $e:expr) => {
        U16PrefixedVec::from($e.as_slice())
    };
    ($typ:ty, $e:expr) => {
        $e
    };
}

macro_rules! packets {
    (
        $(
            $packet:ident {
                $(
                    $field:ident $typ:ident $(<$generics:ident>)?
                );* $(;)?
            } $(,)?
        )*
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $packet {
                $(
                    pub $field: user_type!($typ $(<$generics>)?),
                )*
            }

            #[allow(unused_imports, unused_variables)]
            impl crate::Readable for $packet {
                fn read(buffer: &mut ::std::io::Cursor<&[u8]>) -> anyhow::Result<Self>
                where
                    Self: Sized
                {
                    use anyhow::Context as _;
                    $(
                        let $field = <$typ $(<$generics>)?>::read(buffer)
                            .context(concat!("failed to read field `", stringify!($field), "` of packet `", stringify!($packet), "`"))?
                            .into();
                    )*

                    Ok(Self {
                        $(
                            $field,
                        )*
                    })
                }
            }

            #[allow(unused_variables)]
            impl crate::Writeable for $packet {
                fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
                    $(
                        user_type_convert_to_writeable!($typ $(<$generics>)?, &self.$field).write(buffer)?;
                    )*
                    Ok(())
                }
            }
        )*
    };
}

macro_rules! packet_enum {
    (
        $ident:ident {
            $($id:literal = $packet:ident),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $packet($packet),
            )*
        }

        impl $ident {
            /// Returns the packet ID of this packet.
            pub fn id(&self) -> u32 {
                match self {
                    $(
                        $ident::$packet(_) => $id,
                    )*
                }
            }
        }

        impl crate::Readable for $ident {
            fn read(buffer: &mut ::std::io::Cursor<&[u8]>) -> anyhow::Result<Self>
            where
                Self: Sized
            {
                let packet_id = u8::read(buffer)?;
                match packet_id {
                    $(
                        id if id == $id => Ok($ident::$packet($packet::read(buffer)?)),
                    )*
                    _ => Err(anyhow::anyhow!("unknown packet ID {:#x}", packet_id)),
                }
            }
        }

        impl crate::Writeable for $ident {
            fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
                (self.id() as u8).write(buffer)?;
                match self {
                    $(
                        $ident::$packet(packet) => {
                            packet.write(buffer)?;
                        }
                    )*
                }
                Ok(())
            }
        }

        $(
            impl VariantOf<$ident> for $packet {
                fn discriminant_id() -> u32 { $id }

                #[allow(unreachable_patterns)]
                fn destructure(e: $ident) -> Option<Self> {
                    match e {
                        $ident::$packet(p) => Some(p),
                        _ => None,
                    }
                }
            }

            impl From<$packet> for $ident {
                fn from(packet: $packet) -> Self {
                    $ident::$packet(packet)
                }
            }
        )*
    }
}

macro_rules! discriminant_to_literal {
    (String, $discriminant:expr) => {
        &*$discriminant
    };
    ($discriminant_type:ident, $discriminant:expr) => {
        $discriminant.into()
    };
}

macro_rules! def_enum {
    (
        $ident:ident ($discriminant_type:ident) {
            $(
                $discriminant:literal = $variant:ident
                $(
                    {
                        $(
                            $field:ident $typ:ident $(<$generics:ident>)?
                        );* $(;)?
                    }
                )?
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $ident {
            $(
                $variant
                $(
                    {
                        $(
                            $field: user_type!($typ $(<$generics>)?),
                        )*
                    }
                )?,
            )*
        }

        impl crate::Readable for $ident {
            fn read(buffer: &mut ::std::io::Cursor<&[u8]>) -> anyhow::Result<Self>
                where
                    Self: Sized
            {
                use anyhow::Context as _;
                let discriminant = <$discriminant_type>::read(buffer)
                    .context(concat!("failed to read discriminant for enum type ", stringify!($ident)))?;

                match discriminant_to_literal!($discriminant_type, discriminant) {
                    $(
                        $discriminant => {
                            $(
                                $(
                                    let $field = <$typ $(<$generics>)?>::read(buffer, version)
                                        .context(concat!("failed to read field `", stringify!($field),
                                            "` of enum `", stringify!($ident), "::", stringify!($variant), "`"))?
                                            .into();
                                )*
                            )?

                            Ok($ident::$variant $(
                                {
                                    $(
                                        $field,
                                    )*
                                }
                            )?)
                        },
                    )*
                    _ => Err(anyhow::anyhow!(
                        concat!(
                            "no discriminant for enum `", stringify!($ident), "` matched value {:?}"
                        ), discriminant
                    ))
                }
            }
        }

        impl crate::Writeable for $ident {
            fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
                match self {
                    $(
                        $ident::$variant $(
                            {
                                $($field,)*
                            }
                        )? => {
                            let discriminant = <$discriminant_type>::from($discriminant);
                            discriminant.write(buffer)?;

                            $(
                                $(
                                    user_type_convert_to_writeable!($typ $(<$generics>)?, $field).write(buffer, version)?;
                                )*
                            )?
                        }
                    )*
                }
                Ok(())
            }
        }
    };
}

/// Trait implemented for packets which can be converted from a packet
/// enum. For example, `SpawnEntity` implements `VariantOf<ServerPlayPacket>`.
pub trait VariantOf<Enum> {
    /// Returns the unique ID used to determine whether
    /// an enum variant matches this variant.
    fn discriminant_id() -> u32;

    /// Attempts to destructure the `Enum` into this type.
    /// Returns `None` if `enum` is not the correct variant.
    fn destructure(e: Enum) -> Option<Self>
    where
        Self: Sized;
}
use crate::io::BytePrefixedVec;
use crate::io::MD5Array;
use crate::io::U16PrefixedVec;
use crate::io::WideString;
pub mod client;
pub mod common;
pub mod server;
