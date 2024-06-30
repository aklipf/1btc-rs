use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::{io, vec};

#[derive(Debug)]
pub struct Data{
    pub name: [u8;32],
    pub value: i32,
}

pub struct ChunkedIter<const CHUNK_SIZE: usize> {
    fp: File,
    ptr: usize,
    buffer: [u8; CHUNK_SIZE],
    buffer_size: usize,
    count: i32,
}

fn parse_data(town: &[u8], temp: &[u8])->Data{
    let mut temp_value = ((temp[temp.len()-1]-b'0')+(temp[temp.len()-3]-b'0')*10) as i32;

    if temp.len()>3{
        if temp[temp.len()-4]!=b'-' {
            temp_value += ((temp[temp.len()-4]-b'0')as i32)*100;
        }

        if temp[0]==b'-'{
            temp_value=-temp_value;
        }
    }

    let mut name: [u8;32]=Default::default();
    let (left, _) = name.split_at_mut(town.len());
    left.copy_from_slice(town);

    Data {
        name: name,
        value:temp_value,
    }
}

impl<const CHUNK_SIZE: usize> Iterator for ChunkedIter<CHUNK_SIZE> {
    type Item = Data;

    fn next(&mut self) -> Option<Self::Item> {
        let mut start_ptr: usize = self.ptr;
        let mut sep_ptr: usize = usize::max_value();

        while self.ptr != usize::max_value() {
            if self.ptr >= self.buffer_size {
                if (self.count&0xf)==0{
                    let count = self.count;
                    println!("{count}");
                }

                self.buffer.copy_within(start_ptr.., 0);
                self.ptr -=start_ptr;
                if sep_ptr!=usize::max_value(){
                    sep_ptr-=start_ptr;
                }
                start_ptr=0;

                self.buffer_size = self.fp.read(&mut self.buffer[self.ptr..]).unwrap()+self.ptr;

                self.count += 1;
                if self.buffer_size == 0 {
                    self.ptr = usize::max_value();
                    break;
                }
            }

            let current = self.buffer[self.ptr];
            self.ptr += 1;
            if sep_ptr==usize::max_value(){
                if current==b';'{
                    sep_ptr=self.ptr;
                }
            }else{
                if current==b'\n'{
                    return Some(parse_data(& self.buffer[start_ptr..(sep_ptr-1)], & self.buffer[sep_ptr..(self.ptr-1)]));
                }
            }
        }

        None
    }
}

pub fn chunked_iter<const CHUNK_SIZE: usize>(fp: File) -> ChunkedIter<CHUNK_SIZE> {
    ChunkedIter {
        fp: fp,
        ptr: CHUNK_SIZE,
        buffer: [0; CHUNK_SIZE],
        buffer_size: 0,
        count: 0,
    }
}
