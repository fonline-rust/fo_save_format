#![allow(non_snake_case)]

#[allow(non_camel_case_types)]
pub type uchar = ::std::os::raw::c_uchar;
#[allow(non_camel_case_types)]
pub type ushort = ::std::os::raw::c_ushort;
#[allow(non_camel_case_types)]
pub type uint = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct NpcBagItem {
    pub ItemPid: uint,
    pub MinCnt: uint,
    pub MaxCnt: uint,
    pub ItemSlot: uint,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CritData {
    pub Id: uint,
    pub HexX: ushort,
    pub HexY: ushort,
    pub WorldX: ushort,
    pub WorldY: ushort,
    pub BaseType: uint,
    pub Dir: uchar,
    pub Cond: uchar,
    pub ReservedCE: uchar,
    pub Reserved0: ::std::os::raw::c_char,
    pub ScriptId: uint,
    pub ShowCritterDist1: uint,
    pub ShowCritterDist2: uint,
    pub ShowCritterDist3: uint,
    pub Reserved00: ushort,
    pub Multihex: ::std::os::raw::c_short,
    pub GlobalGroupUid: uint,
    pub LastHexX: ushort,
    pub LastHexY: ushort,
    pub Reserved1: Array<[u32; 4]>,
    pub MapId: uint,
    pub MapPid: ushort,
    pub Reserved2: ushort,
    pub Params: Array<[i32; 1000]>,
    pub Anim1Life: uint,
    pub Anim1Knockout: uint,
    pub Anim1Dead: uint,
    pub Anim2Life: uint,
    pub Anim2Knockout: uint,
    pub Anim2Dead: uint,
    pub Anim2KnockoutEnd: uint,
    pub Reserved3: Array<[u32; 3]>,
    pub Lexems: Array<[i8; 128]>,
    pub Reserved4: Array<[u32; 8]>,
    pub ClientToDelete: bool,
    pub Reserved5: uchar,
    pub Reserved6: ushort,
    pub Temp: uint,
    pub Reserved8: ushort,
    pub HoloInfoCount: ushort,
    pub HoloInfo: Array<[u32; 250]>,
    pub Reserved9: Array<[u32; 10]>,
    pub Scores: Array<[i32; 50]>,
    pub UserData: Array<[u8; 400]>,
    pub HomeMap: uint,
    pub HomeX: ushort,
    pub HomeY: ushort,
    pub HomeOri: uchar,
    pub Reserved11: uchar,
    pub ProtoId: ushort,
    pub Reserved12: uint,
    pub Reserved13: uint,
    pub Reserved14: uint,
    pub Reserved15: uint,
    pub IsDataExt: bool,
    pub Reserved16: uchar,
    pub Reserved17: ushort,
    pub Reserved18: Array<[u32; 8]>,
    pub FavoriteItemPid: Array<[ushort; 4]>,
    pub Reserved19: Array<[u32; 10]>,
    pub EnemyStackCount: uint,
    pub EnemyStack: Array<[u32; 30]>,
    pub Reserved20: Array<[u32; 5]>,
    pub BagCurrentSet: Array<[u8; 20]>,
    pub BagRefreshTime: ::std::os::raw::c_short,
    pub Reserved21: uchar,
    pub BagSize: uchar,
    pub Bag: Array<[NpcBagItem; 50]>,
    pub Reserved22: Array<[u32; 100]>,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CritDataExt {
    pub Reserved23: Array<[u32; 10]>,
    pub GlobalMapFog: Array<[u8; 2500]>,
    pub Reserved24: ushort,
    pub LocationsCount: ushort,
    pub LocationsId: Array<[u32; 1000]>,
    pub Reserved25: Array<[u32; 40]>,
    pub PlayIp: Array<[u32; 20]>,
    pub PlayPort: Array<[ushort; 20]>,
    pub CurrentIp: uint,
    pub Reserved26: Array<[u32; 29]>,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CrTimeEvent {
    pub FuncNum: uint,
    pub Rate: uint,
    pub NextTime: uint,
    pub Identifier: ::std::os::raw::c_int,
}

#[derive(Debug, Clone)]
pub struct ClientSaveData {
    pub signature: [u8; 4],
    //pub Name: [::std::os::raw::c_char; 31usize],
    pub password_hash: [u8; 32],
    pub data: Box<CritData>,
    pub data_ext: Box<CritDataExt>,
    pub time_events: Vec<CrTimeEvent>,
}

use std::{
    io::{Read, Result},
    mem::size_of,
};

impl ClientSaveData {
    pub fn read<R: Read>(reader: &mut R)-> Result<ClientSaveData> {
        assert_eq!(size_of::<::std::os::raw::c_char>(), size_of::<u8>());
        assert_eq!(size_of::<uint>(), size_of::<u32>());
        assert_eq!(size_of::<CritData>(), 7404);
        assert_eq!(size_of::<CritDataExt>(), 6944);
        let mut signature = [0u8; 4];
        reader.read_exact(&mut signature[..])?;
        let mut password_hash = [0u8; 32];
        reader.read_exact(&mut password_hash[..])?;
        let mut data = vec![0u8; size_of::<CritData>()];
        reader.read_exact(&mut data[..])?;
        let mut data_ext = vec![0u8; size_of::<CritDataExt>()];
        reader.read_exact(&mut data_ext[..])?;
        let mut te_count = [0u8; size_of::<u32>()];
        reader.read_exact(&mut te_count[..])?;
        let te_count = u32::from_ne_bytes(te_count) as usize;
        assert!(te_count < 0xFFFF);
        let mut time_events = Vec::with_capacity(te_count);
        let mut time_event_buffer = [0u8; size_of::<CrTimeEvent>()];
        for _ in 0..te_count {
            reader.read_exact(&mut time_event_buffer[..])?;
            let time_event = unsafe {
                std::mem::transmute(time_event_buffer)
            };
            time_events.push(time_event);
        }
        assert!(reader.bytes().next().is_none());
        let data = unsafe{ transmute_from_vec(data) };
        let data_ext = unsafe{ transmute_from_vec(data_ext) };
        Ok(ClientSaveData {
            signature,
            password_hash,
            data,
            data_ext,
            time_events,
        })
    }
}

unsafe fn transmute_from_vec<T>(data: Vec<u8>) -> Box<T> {
    assert_eq!(data.len(), size_of::<T>());
    let mut boxed_slice = data.into_boxed_slice();
    let ptr: *mut T = std::mem::transmute(boxed_slice.as_mut_ptr());
    std::mem::forget(boxed_slice);
    Box::from_raw(ptr)
}

#[repr(C)]
#[derive(Clone)]
pub struct Array<T>(pub T);

use std::ops::Deref;

impl<T> Deref for Array<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



macro_rules! array {
    ( $($num:expr)+ ) => {
        $(
            impl<T: ::std::fmt::Debug> ::std::fmt::Debug for Array<[T; $num]> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "{:?}", &self.0[..])
                }
            }
        )+
    }
}

array!( 3 4 5 8 10 20 29 30 40 50 100 128 250 400 1000 2500 );