#![allow(non_snake_case)]

use bytemuck::{
    Pod, Zeroable, allocation::zeroed_box, bytes_of, bytes_of_mut, must_cast_slice,
    must_cast_slice_mut, zeroed_vec,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::{io, mem::size_of};

const fn assert_size<T>(size: usize) -> usize {
    let expected = size_of::<T>();
    if expected != size {
        panic!("unexpected size");
    }
    size
}

const SIGNATURE: [u8; 4] = [70, 79, 0, 2];
const DATA_SIZE: usize = assert_size::<CritData>(7404);
const DATA_EXT_SIZE: usize = assert_size::<CritDataExt>(6944);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bool(u8);

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self(if value { 1 } else { 0 })
    }
}
impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        value.0 != 0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, bytemuck::TransparentWrapper, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct BigArray<T, const LEN: usize>(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "serde_big_array::BigArray",
            bound(
                serialize = "T: Serialize",
                deserialize = "T: serde::de::DeserializeOwned"
            )
        )
    )]
    pub [T; LEN],
);

impl<T: Zeroable, const LEN: usize> Default for BigArray<T, LEN> {
    fn default() -> Self {
        Self(Zeroable::zeroed())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NpcBagItem {
    pub ItemPid: u32,
    pub MinCnt: u32,
    pub MaxCnt: u32,
    pub ItemSlot: u32,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CritData {
    pub Id: u32,
    pub HexX: u16,
    pub HexY: u16,
    pub WorldX: u16,
    pub WorldY: u16,
    pub BaseType: u32,
    pub Dir: u8,
    pub Cond: u8,
    pub ReservedCE: u8,
    pub Reserved0: i8,
    pub ScriptId: u32,
    pub ShowCritterDist1: u32,
    pub ShowCritterDist2: u32,
    pub ShowCritterDist3: u32,
    pub Reserved00: u16,
    pub Multihex: i16,
    pub GlobalGroupUid: u32,
    pub LastHexX: u16,
    pub LastHexY: u16,
    pub Reserved1: [u32; 4],
    pub MapId: u32,
    pub MapPid: u16,
    pub Reserved2: u16,
    pub Params: BigArray<i32, 1000>,
    pub Anim1Life: u32,
    pub Anim1Knockout: u32,
    pub Anim1Dead: u32,
    pub Anim2Life: u32,
    pub Anim2Knockout: u32,
    pub Anim2Dead: u32,
    pub Anim2KnockoutEnd: u32,
    pub Reserved3: [u32; 3],
    pub Lexems: BigArray<i8, 128>,
    pub Reserved4: [u32; 8],
    pub ClientToDelete: Bool,
    pub Reserved5: u8,
    pub Reserved6: u16,
    pub Temp: u32,
    pub Reserved8: u16,
    pub HoloInfoCount: u16,
    pub HoloInfo: BigArray<u32, 250>,
    pub Reserved9: [u32; 10],
    pub Scores: BigArray<i32, 50>,
    pub UserData: BigArray<u8, 400>,
    pub HomeMap: u32,
    pub HomeX: u16,
    pub HomeY: u16,
    pub HomeOri: u8,
    pub Reserved11: u8,
    pub ProtoId: u16,
    pub Reserved12: u32,
    pub Reserved13: u32,
    pub Reserved14: u32,
    pub Reserved15: u32,
    pub IsDataExt: Bool,
    pub Reserved16: u8,
    pub Reserved17: u16,
    pub Reserved18: [u32; 8],
    pub FavoriteItemPid: [u16; 4],
    pub Reserved19: [u32; 10],
    pub EnemyStackCount: u32,
    pub EnemyStack: [u32; 30],
    pub Reserved20: [u32; 5],
    pub BagCurrentSet: [u8; 20],
    pub BagRefreshTime: i16,
    pub Reserved21: u8,
    pub BagSize: u8,
    pub Bag: BigArray<NpcBagItem, 50>,
    pub Reserved22: BigArray<u32, 100>,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CritDataExt {
    pub Reserved23: [u32; 10],
    pub GlobalMapFog: BigArray<u8, 2500>,
    pub Reserved24: u16,
    pub LocationsCount: u16,
    pub LocationsId: BigArray<u32, 1000>,
    pub Reserved25: BigArray<u32, 40>,
    pub PlayIp: [u32; 20],
    pub PlayPort: [u16; 20],
    pub CurrentIp: u32,
    pub Reserved26: [u32; 29],
}
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CrTimeEvent {
    pub FuncNum: u32,
    pub Rate: u32,
    pub NextTime: u32,
    pub Identifier: i32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClientSaveData {
    pub signature: [u8; 4],
    //pub Name: [i16; 31usize],
    pub password_hash: [u8; 32],
    pub data: Box<CritData>,
    pub data_ext: Box<CritDataExt>,
    pub time_events: Vec<CrTimeEvent>,
}

impl ClientSaveData {
    fn time_events_count(&self) -> u32 {
        self.time_events.len() as u32
    }
}

fn invalid_data<T>() -> io::Result<T> {
    Err(std::io::ErrorKind::InvalidData.into())
}

#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[duplicate::duplicate_item(
    async   await           _feature            _read           _Read;
    []      [identity()]    [all()]             [sync_read]     [io::Read];
    [async] [await]         [feature = "tokio"] [async_read]    [AsyncRead + Unpin];
)]
impl ClientSaveData {
    #[cfg(_feature)]
    pub async fn _read<R: _Read>(reader: &mut R) -> io::Result<ClientSaveData> {
        let mut signature = [0u8; 4];
        reader.read_exact(&mut signature[..]).await?;
        if signature != SIGNATURE {
            invalid_data()?;
        }
        let mut password_hash = [0u8; 32];
        reader.read_exact(&mut password_hash[..]).await?;

        let mut data = zeroed_box::<CritData>();
        reader.read_exact(bytes_of_mut(&mut *data)).await?;

        let mut data_ext = zeroed_box::<CritDataExt>();
        reader.read_exact(bytes_of_mut(&mut *data_ext)).await?;

        let mut te_count = 0u32;
        reader.read_exact(bytes_of_mut(&mut te_count)).await?;
        if te_count > 0xFFFF {
            invalid_data()?;
        }
        let mut time_events = zeroed_vec::<CrTimeEvent>(te_count as usize);
        reader
            .read_exact(must_cast_slice_mut(time_events.as_mut_slice()))
            .await?;

        Ok(ClientSaveData {
            signature,
            password_hash,
            data,
            data_ext,
            time_events,
        })
    }
}

#[duplicate::duplicate_item(
    async   await           _feature            _write           _Write;
    []      [identity()]    [all()]             [sync_write]     [io::Write];
    [async] [await]         [feature = "tokio"] [async_write]    [AsyncWrite + Unpin];
)]
impl ClientSaveData {
    #[cfg(_feature)]
    pub async fn _write<W: _Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.signature[..]).await?;
        writer.write_all(&self.password_hash[..]).await?;
        writer.write_all(bytes_of(&*self.data)).await?;
        writer.write_all(bytes_of(&*self.data_ext)).await?;
        writer
            .write_all(bytes_of(&self.time_events_count()))
            .await?;
        writer.write_all(must_cast_slice(&self.time_events)).await?;
        Ok(())
    }
}

impl ClientSaveData {
    pub fn write_to_vec(&self) -> Vec<u8> {
        let full_size = 4
            + 32
            + DATA_SIZE
            + DATA_EXT_SIZE
            + 4
            + self.time_events.len() * size_of::<CrTimeEvent>();
        let mut vec = Vec::with_capacity(full_size);
        self.sync_write(&mut vec).expect("write to Vec can't fail");
        assert_eq!(vec.len(), full_size);
        vec
    }
}

trait Identity: Sized {
    fn identity(self) -> Self;
}
impl<T> Identity for T {
    fn identity(self) -> Self {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_write_hybrid() {
        let vec = std::fs::read("../FO4RP/save/clients/qthree.client").unwrap();
        let mut slice = &vec[..];
        let client = ClientSaveData::sync_read(&mut slice).unwrap();
        eprintln!("{client:?}");
        assert_eq!(slice.len(), 0);

        let vec2 = client.write_to_vec();
        assert_eq!(vec, vec2);
    }
}
