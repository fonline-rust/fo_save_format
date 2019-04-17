#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use serdebug::SerDebug;

#[allow(non_camel_case_types)]
pub type uchar = ::std::os::raw::c_uchar;
#[allow(non_camel_case_types)]
pub type ushort = ::std::os::raw::c_ushort;
#[allow(non_camel_case_types)]
pub type uint = ::std::os::raw::c_uint;

use serde_big_array::big_array;
big_array! { BigArray; 3, 4, 5, 8, 10, 20, 29, 30, 40, 50, 100, 128, 250, 400, 1000, 2500,}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct NpcBagItem {
    pub ItemPid: uint,
    pub MinCnt: uint,
    pub MaxCnt: uint,
    pub ItemSlot: uint,
}
#[repr(C)]
#[derive(SerDebug, Clone, Serialize, Deserialize)]
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
    pub Reserved1: [u32; 4],
    pub MapId: uint,
    pub MapPid: ushort,
    pub Reserved2: ushort,
    #[serde(with = "BigArray")]
    pub Params: [i32; 1000],
    pub Anim1Life: uint,
    pub Anim1Knockout: uint,
    pub Anim1Dead: uint,
    pub Anim2Life: uint,
    pub Anim2Knockout: uint,
    pub Anim2Dead: uint,
    pub Anim2KnockoutEnd: uint,
    pub Reserved3: [u32; 3],
    #[serde(with = "BigArray")]
    pub Lexems: [i8; 128],
    pub Reserved4: [u32; 8],
    pub ClientToDelete: bool,
    pub Reserved5: uchar,
    pub Reserved6: ushort,
    pub Temp: uint,
    pub Reserved8: ushort,
    pub HoloInfoCount: ushort,
    #[serde(with = "BigArray")]
    pub HoloInfo: [u32; 250],
    pub Reserved9: [u32; 10],
    #[serde(with = "BigArray")]
    pub Scores: [i32; 50],
    #[serde(with = "BigArray")]
    pub UserData: [u8; 400],
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
    pub Reserved18: [u32; 8],
    pub FavoriteItemPid: [ushort; 4],
    pub Reserved19: [u32; 10],
    pub EnemyStackCount: uint,
    pub EnemyStack: [u32; 30],
    pub Reserved20: [u32; 5],
    pub BagCurrentSet: [u8; 20],
    pub BagRefreshTime: ::std::os::raw::c_short,
    pub Reserved21: uchar,
    pub BagSize: uchar,
    #[serde(with = "BigArray")]
    pub Bag: [NpcBagItem; 50],
    #[serde(with = "BigArray")]
    pub Reserved22: [u32; 100],
}
#[repr(C)]
#[derive(SerDebug, Clone, Serialize, Deserialize)]
pub struct CritDataExt {
    pub Reserved23: [u32; 10],
    #[serde(with = "BigArray")]
    pub GlobalMapFog: [u8; 2500],
    pub Reserved24: ushort,
    pub LocationsCount: ushort,
    #[serde(with = "BigArray")]
    pub LocationsId: [u32; 1000],
    #[serde(with = "BigArray")]
    pub Reserved25: [u32; 40],
    pub PlayIp: [u32; 20],
    pub PlayPort: [ushort; 20],
    pub CurrentIp: uint,
    pub Reserved26: [u32; 29],
}
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrTimeEvent {
    pub FuncNum: uint,
    pub Rate: uint,
    pub NextTime: uint,
    pub Identifier: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSaveData {
    pub signature: [u8; 4],
    //pub Name: [::std::os::raw::c_char; 31usize],
    pub password_hash: [u8; 32],
    pub data: Box<CritData>,
    pub data_ext: Box<CritDataExt>,
    pub time_events: Vec<CrTimeEvent>,
}

use std::{
    io::{self, Read, Write},
    mem::size_of,
};

const SIGNATURE: [u8; 4] = [70, 79, 0, 2];
const DATA_SIZE: usize = 7404;
const DATA_EXT_SIZE: usize = 6944;

fn invalid_data<T>() -> io::Result<T> {
    Err(std::io::ErrorKind::InvalidData.into())
}

impl ClientSaveData {
    pub fn read_unsafe<R: Read>(reader: &mut R) -> io::Result<ClientSaveData> {
        let mut signature = [0u8; 4];
        reader.read_exact(&mut signature[..])?;
        if signature != SIGNATURE {
            invalid_data()?;
        }
        let mut password_hash = [0u8; 32];
        reader.read_exact(&mut password_hash[..])?;

        let mut data = vec![0u8; size_of::<CritData>()];
        reader.read_exact(&mut data[..])?;

        let mut data_ext = vec![0u8; size_of::<CritDataExt>()];
        reader.read_exact(&mut data_ext[..])?;

        let mut te_count = [0u8; size_of::<u32>()];
        reader.read_exact(&mut te_count[..])?;
        let te_count = u32::from_ne_bytes(te_count) as usize;
        if te_count > 0xFFFF {
            invalid_data()?;
        }
        let mut time_events = Vec::with_capacity(te_count);
        let mut time_event_buffer = [0u8; size_of::<CrTimeEvent>()];
        for _ in 0..te_count {
            reader.read_exact(&mut time_event_buffer[..])?;
            let time_event = unsafe { std::mem::transmute(time_event_buffer) };
            time_events.push(time_event);
        }
        if reader.bytes().next().is_some() {
            invalid_data()?;
        }
        let data = unsafe { transmute_from_vec(data) }?;
        let data_ext = unsafe { transmute_from_vec(data_ext) }?;
        Ok(ClientSaveData {
            signature,
            password_hash,
            data,
            data_ext,
            time_events,
        })
    }

    pub fn read_bincode<R: Read>(reader: &mut R) -> io::Result<ClientSaveData> {
        let mut signature = [0u8; 4];
        reader.read_exact(&mut signature[..])?;
        if signature != SIGNATURE {
            invalid_data()?;
        }
        let mut password_hash = [0u8; 32];
        reader.read_exact(&mut password_hash[..])?;

        let mut data = [0u8; size_of::<CritData>()];
        reader.read_exact(&mut data[..])?;
        let data = bincode::deserialize_from(&mut &data[..])
            .map_err(|_| std::io::ErrorKind::InvalidData)?;

        let mut data_ext = [0u8; size_of::<CritDataExt>()];
        reader.read_exact(&mut data_ext[..])?;
        let data_ext = bincode::deserialize_from(&mut &data_ext[..])
            .map_err(|_| std::io::ErrorKind::InvalidData)?;

        let mut te_count = [0u8; size_of::<u32>()];
        reader.read_exact(&mut te_count[..])?;
        let te_count = u32::from_ne_bytes(te_count) as usize;
        if te_count > 0xFFFF {
            invalid_data()?;
        }
        let mut time_events = Vec::with_capacity(te_count);
        let mut time_event_buffer = [0u8; size_of::<CrTimeEvent>()];
        for _ in 0..te_count {
            reader.read_exact(&mut time_event_buffer[..])?;
            let time_event = bincode::deserialize_from(&mut &time_event_buffer[..])
                .map_err(|_| std::io::ErrorKind::InvalidData)?;
            time_events.push(time_event);
        }
        if reader.bytes().next().is_some() {
            invalid_data()?;
        }
        Ok(ClientSaveData {
            signature,
            password_hash,
            data,
            data_ext,
            time_events,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let full_size = 4
            + 32
            + DATA_SIZE
            + DATA_EXT_SIZE
            + 4
            + self.time_events.len() * size_of::<CrTimeEvent>();
        let mut vec = Vec::with_capacity(full_size);
        vec.write(&self.signature[..]).unwrap();
        vec.write(&self.password_hash[..]).unwrap();
        bincode::serialize_into(&mut vec, &self.data).unwrap();
        //assert_eq!(data.len(), 7404);
        bincode::serialize_into(&mut vec, &self.data_ext).unwrap();
        //assert_eq!(data_ext.len(), 6944);
        vec.write(&(self.time_events.len() as u32).to_ne_bytes()[..])
            .unwrap();
        for event in &self.time_events {
            bincode::serialize_into(&mut vec, &event).unwrap();
        }
        assert_eq!(vec.len(), full_size);
        vec
    }
}

unsafe fn transmute_from_vec<T>(data: Vec<u8>) -> io::Result<Box<T>> {
    if data.len() != size_of::<T>() {
        invalid_data()?;
    }
    let mut boxed_slice = data.into_boxed_slice();
    let ptr: *mut T = std::mem::transmute(boxed_slice.as_mut_ptr());
    std::mem::forget(boxed_slice);
    Ok(Box::from_raw(ptr))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_unsafe_write_hybrid() {
        let vec = std::fs::read("test/qthree.client").unwrap();
        let mut slice = &vec[..];
        let client = ClientSaveData::read_unsafe(&mut slice).unwrap();

        let vec2 = client.write();
        assert_eq!(vec, vec2);
        /*
        for i in 0..vec.len().max(vec3.len()) {
            match (vec.get(i), vec3.get(i)) {
                (Some(a), Some(b)) if a==b => {},
                (a, b) => {
                    println!("{}# Original: {:?}, Bincode: {:?}", i, a, b);
                }
            }
        }*/
    }

    #[test]
    fn read_unsafe_read_bincode() {
        let vec = std::fs::read("test/qthree.client").unwrap();
        let mut slice = &vec[..];
        let client = ClientSaveData::read_unsafe(&mut slice).unwrap();
        let mut slice = &vec[..];
        let client2 = ClientSaveData::read_bincode(&mut slice).unwrap();

        let json1 = serde_json::to_string(&client).unwrap();
        let json2 = serde_json::to_string(&client2).unwrap();
        assert_eq!(json1, json2);

        let vec1 = client.write();
        let vec2 = client.write();
        assert_eq!(vec, vec1);
        assert_eq!(vec, vec2);
        /*
        for i in 0..vec.len().max(vec3.len()) {
            match (vec.get(i), vec3.get(i)) {
                (Some(a), Some(b)) if a==b => {},
                (a, b) => {
                    println!("{}# Original: {:?}, Bincode: {:?}", i, a, b);
                }
            }
        }*/
    }

    #[test]
    fn sizeof() {
        assert_eq!(size_of::<::std::os::raw::c_char>(), size_of::<u8>());
        assert_eq!(size_of::<uint>(), size_of::<u32>());
        assert_eq!(size_of::<CritData>(), DATA_SIZE);
        assert_eq!(size_of::<CritDataExt>(), DATA_EXT_SIZE);
    }
}
