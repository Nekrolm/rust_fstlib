use std::io;
use std::fs::File;
use std::io::{Read, Write, Error, ErrorKind};
use std::mem::size_of;
use std::slice;
use super::VectorFst;
use super::ConstFst;
use super::Arc;
use super::Weight;
use super::ArcIterator;
use super::ArcIteratorTrait;
use std::intrinsics::transmute;
use crate::fst::traits::{kNoStateId, ExpandedFst};


pub fn read_int32(file : &mut File) -> io::Result<i32> {
    let mut buffer = [0 as u8; 4];
    return match file.read_exact(&mut buffer) {
        Err(why) => Err(why),
        Ok(_) => Ok(i32::from_le_bytes(buffer))
    }
}

pub fn read_uint64(file : &mut File) -> io::Result<u64> {
    let mut buffer = [0 as u8; 8];
    return match file.read_exact(&mut buffer) {
        Err(why) => Err(why),
        Ok(_) => Ok(u64::from_le_bytes(buffer))
    }
}

pub fn read_f32(file : &mut File) -> io::Result<f32> {
    let mut buffer = [0 as u8; 4];
    return match file.read_exact(&mut buffer) {
        Err(why) => Err(why),
        Ok(_) => Ok(f32::from_le_bytes(buffer))
    }
}


pub fn write_int32(file : &mut File, value : i32) -> io::Result<()> {
    return file.write_all(value.to_le_bytes().as_ref());
}

pub fn write_uint64(file : &mut File, value : u64) -> io::Result<()> {
    return file.write_all(value.to_le_bytes().as_ref());
}

pub fn write_f32(file : &mut File, value : f32) -> io::Result<()> {
    return file.write_all(value.to_le_bytes().as_ref());
}



pub struct FstSize {
    pub num_states : i32,
    pub num_arcs : u64
}


fn read_arcs(file : &mut File, num_arcs : usize) -> io::Result<Vec<Arc>> {
    if num_arcs == 0 {
        return Ok(Vec::new());
    }
    let mut arcs = vec![Arc::new(0,0,Weight::new(0.),0); num_arcs];
    let raw_arcs : & mut[u8] = unsafe {
        let ptr = arcs.as_ptr() as *mut u8;
        slice::from_raw_parts_mut(ptr, arcs.len() * size_of::<Arc>())
    };
    return file.read_exact(raw_arcs).and_then( |()| return Ok(arcs));
}

fn write_arcs(file : &mut File, arcs : &Vec<Arc> ) -> io::Result<()> {
    if arcs.is_empty() {
        return Ok(())
    }


    let raw_arcs : &[u8] = unsafe {
        let ptr = arcs.as_ptr() as *const u8;
        slice::from_raw_parts(ptr, arcs.len() * size_of::<Arc>())
    };
    return file.write_all(raw_arcs);
}

impl VectorFst {
    pub fn Read(file : &mut File) -> io::Result<VectorFst> {
        return read_int32(file).and_then(|num_states| {
            let mut fst = VectorFst::new();
             for state in 0..num_states {
                 match  read_uint64(file).and_then(|num_arcs| {
                     let weight =  match read_f32(file) {
                         Ok(v) => Weight::new(v),
                         Err(why) => return Err(why)
                     };
                     read_arcs(file,num_arcs as usize).and_then(|arcs|{
                         let s = fst.AddState();
                         fst.SetFinal(s, weight);
                         for arc in arcs {
                             fst.AddArc(s, arc);
                         }
                         return Ok(());
                     })
                 }){
                     Err(why) => return Err(why),
                     _ => continue
                 }
             }
            return read_int32(file).and_then(|start|{
                if start != kNoStateId {
                    fst.SetStart(start);
                }
                return Ok(fst);
            })
        });
    }

    pub fn Write(&self, file : &mut File) -> io::Result<()> {
        return write_int32(file, self.NumStates()).and_then(|()| {
            for state in 0..self.NumStates() {
                match  write_uint64(file, self.NumArcs(state) as u64).and_then(|()|
                    write_f32(file, self.Final(state).Value())).and_then(|()|{
                        let mut arcs : Vec<Arc> = Vec::with_capacity(self.NumArcs(state) as usize);
                        let mut aiter = ArcIterator::new(self, state);
                        while !aiter.Done() {
                            arcs.push(aiter.Value());
                            aiter.Next();
                        }
                        return write_arcs(file,&arcs);
                    }) {
                    Err(why) => return Err(why),
                    _ => continue
                }
            }
            return write_int32(file, self.Start());
        })
    }

}
