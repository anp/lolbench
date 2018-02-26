extern crate alloc_no_stdlib;
extern crate brotli;
extern crate criterion;
#[macro_use]
extern crate wrap_libtest;

use criterion::Bencher;

extern crate core;
extern crate brotli_decompressor;
use alloc_no_stdlib::{Allocator, SliceWrapper, SliceWrapperMut};
use brotli::dictionary::{kBrotliDictionary, kBrotliDictionaryOffsetsByLength};
use brotli::transform::{TransformDictionaryWord};
use brotli_decompressor::HuffmanCode;
use core::{cmp, ops};
use std::fmt;

use std::io::{self, Error, ErrorKind, Read, Write};

use brotli::enc::cluster::HistogramPair;
use brotli::enc::command::Command;
use brotli::enc::entropy_encode::HuffmanTree;
use brotli::enc::histogram::{ContextType, HistogramLiteral, HistogramCommand, HistogramDistance};

pub struct Rebox<T> {
  b: Box<[T]>,
}

impl<T> core::default::Default for Rebox<T> {
  fn default() -> Self {
    let v: Vec<T> = Vec::new();
    let b = v.into_boxed_slice();
    Rebox::<T> { b: b }
  }
}

impl<T> ops::Index<usize> for Rebox<T> {
  type Output = T;
  fn index(&self, index: usize) -> &T {
    &(*self.b)[index]
  }
}

impl<T> ops::IndexMut<usize> for Rebox<T> {
  fn index_mut(&mut self, index: usize) -> &mut T {
    &mut (*self.b)[index]
  }
}

impl<T> alloc_no_stdlib::SliceWrapper<T> for Rebox<T> {
  fn slice(&self) -> &[T] {
    &*self.b
  }
}

impl<T> alloc_no_stdlib::SliceWrapperMut<T> for Rebox<T> {
  fn slice_mut(&mut self) -> &mut [T] {
    &mut *self.b
  }
}

pub struct HeapAllocator<T: core::clone::Clone> {
  pub default_value: T,
}

impl<T: core::clone::Clone> alloc_no_stdlib::Allocator<T> for HeapAllocator<T> {
  type AllocatedMemory = Rebox<T>;
  fn alloc_cell(self: &mut HeapAllocator<T>, len: usize) -> Rebox<T> {
    let v: Vec<T> = vec![self.default_value.clone();len];
    let b = v.into_boxed_slice();
    Rebox::<T> { b: b }
  }
  fn free_cell(self: &mut HeapAllocator<T>, _data: Rebox<T>) {}
}

struct IoWriterWrapper<'a, OutputType: Write + 'a>(&'a mut OutputType);

struct IoReaderWrapper<'a, OutputType: Read + 'a>(&'a mut OutputType);

macro_rules! println_stderr(
    ($($val:tt)*) => { {
        writeln!(&mut ::std::io::stderr(), $($val)*).unwrap();
    } }
);

pub fn decompress<InputType, OutputType>(r: &mut InputType,
                                         w: &mut OutputType,
                                         buffer_size: usize)
                                         -> Result<(), io::Error>
  where InputType: Read,
        OutputType: Write
{
  let mut alloc_u8 = HeapAllocator::<u8> { default_value: 0 };
  let mut input_buffer = alloc_u8.alloc_cell(buffer_size);
  let mut output_buffer = alloc_u8.alloc_cell(buffer_size);
  brotli::BrotliDecompressCustomIo(&mut IoReaderWrapper::<InputType>(r),
                                   &mut IoWriterWrapper::<OutputType>(w),
                                   input_buffer.slice_mut(),
                                   output_buffer.slice_mut(),
                                   alloc_u8,
                                   HeapAllocator::<u32> { default_value: 0 },
                                   HeapAllocator::<HuffmanCode> {
                                     default_value: HuffmanCode::default(),
                                   },
                                   Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
}

pub fn compress<InputType, OutputType>(r: &mut InputType,
                                       w: &mut OutputType,
                                       buffer_size: usize,
                                       params:&brotli::enc::BrotliEncoderParams) -> Result<usize, io::Error>
    where InputType: Read,
          OutputType: Write {
    let mut alloc_u8 = HeapAllocator::<u8> { default_value: 0 };
    let mut input_buffer = alloc_u8.alloc_cell(buffer_size);
    let mut output_buffer = alloc_u8.alloc_cell(buffer_size);
    let mut log = |data:&[brotli::interface::Command<brotli::InputReference>]| {for cmd in data.iter() {write_one(cmd)} };
    if params.log_meta_block {
        println_stderr!("window {} 0 0 0", params.lgwin);
    }
    brotli::BrotliCompressCustomIo(&mut IoReaderWrapper::<InputType>(r),
                                   &mut IoWriterWrapper::<OutputType>(w),
                                   &mut input_buffer.slice_mut(),
                                   &mut output_buffer.slice_mut(),
                                   params,
                                   alloc_u8,
                                   HeapAllocator::<u16>{default_value:0},
                                   HeapAllocator::<i32>{default_value:0},
                                   HeapAllocator::<u32>{default_value:0},
                                   HeapAllocator::<Command>{default_value:Command::default()},
                                   HeapAllocator::<brotli::enc::floatX>{default_value:0.0 as brotli::enc::floatX},
                                   HeapAllocator::<brotli::enc::Mem256f>{default_value:brotli::enc::Mem256f::default()},
                                   HeapAllocator::<HistogramLiteral>{
                                       default_value:HistogramLiteral::default(),
                                   },
                                   HeapAllocator::<HistogramCommand>{
                                       default_value:HistogramCommand::default(),
                                   },
                                   HeapAllocator::<HistogramDistance>{
                                       default_value:HistogramDistance::default(),
                                   },
                                   HeapAllocator::<HistogramPair>{
                                       default_value:HistogramPair::default(),
                                   },
                                   HeapAllocator::<ContextType>{
                                       default_value:ContextType::default(),
                                   },
                                   HeapAllocator::<HuffmanTree>{
                                       default_value:HuffmanTree::default(),
                                   },
                                   &mut log,
                                   Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
}

struct UnlimitedBuffer {
  data: Vec<u8>,
  read_offset: usize,
}

fn _write_all<OutputType>(w: &mut OutputType, buf: &[u8]) -> Result<(), io::Error>
  where OutputType: io::Write
{
  let mut total_written: usize = 0;
  while total_written < buf.len() {
    match w.write(&buf[total_written..]) {
      Err(e) => {
        match e.kind() {
          io::ErrorKind::Interrupted => continue,
          _ => return Err(e),
        }
      }
      Ok(cur_written) => {
        if cur_written == 0 {
          return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Write EOF"));
        }
        total_written += cur_written;
      }
    }
  }
  Ok(())
}
trait Runner {
    fn iter<Fn:FnMut()> (&mut self, cb: &mut Fn);
}

struct BenchmarkPassthrough<'a> (pub &'a mut Bencher);
impl<'a> Runner for BenchmarkPassthrough<'a>{
    fn iter<Fn:FnMut()> (&mut self, cb: &mut Fn) {
        self.0.iter(cb)
    }

}

impl UnlimitedBuffer {
  pub fn new(buf: &[u8]) -> Self {
    let mut ret = UnlimitedBuffer {
      data: Vec::<u8>::new(),
      read_offset: 0,
    };
    ret.data.extend(buf);
    return ret;
  }
  pub fn reset_read(&mut self) {
      self.read_offset = 0;
  }
}
impl io::Read for UnlimitedBuffer {
  fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> {
    let bytes_to_read = cmp::min(buf.len(), self.data.len() - self.read_offset);
    if bytes_to_read > 0 {
      buf[0..bytes_to_read].clone_from_slice(&self.data[self.read_offset..
                                              self.read_offset + bytes_to_read]);
    }
    self.read_offset += bytes_to_read;
    return Ok(bytes_to_read);
  }
}

impl io::Write for UnlimitedBuffer {
  fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> {
    self.data.extend(buf);
    return Ok(buf.len());
  }
  fn flush(self: &mut Self) -> io::Result<()> {
    return Ok(());
  }
}

pub struct LimitedBuffer<'a> {
  pub data: &'a mut [u8],
  pub write_offset: usize,
  pub read_offset: usize,
}

impl<'a> LimitedBuffer<'a> {
  pub fn new(buf: &'a mut [u8]) -> Self {
    LimitedBuffer {
        data: buf,
        write_offset: 0,
        read_offset: 0,
    }
  }
}
impl<'a> LimitedBuffer<'a> {
    fn reset(&mut self) {
        self.write_offset = 0;
        self.read_offset = 0;
        self.data.split_at_mut(32).0.clone_from_slice(&[0u8;32]); // clear the first 256 bits
    }
    fn reset_read(&mut self) {
        self.read_offset = 0;
    }
}
impl<'a> io::Read for LimitedBuffer<'a> {
  fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> {
    let bytes_to_read = cmp::min(buf.len(), self.data.len() - self.read_offset);
    if bytes_to_read > 0 {
      buf[0..bytes_to_read].clone_from_slice(&self.data[self.read_offset..
                                              self.read_offset + bytes_to_read]);
    }
    self.read_offset += bytes_to_read;
    return Ok(bytes_to_read);
  }
}

impl<'a> io::Write for LimitedBuffer<'a> {
  fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> {
      let bytes_to_write = cmp::min(buf.len(), self.data.len() - self.write_offset);
      if bytes_to_write > 0 {
          self.data[self.write_offset..self.write_offset + bytes_to_write].clone_from_slice(
              &buf[..bytes_to_write]);
      } else {
          return Err(io::Error::new(io::ErrorKind::WriteZero, "OutOfBufferSpace"));
      }
      self.write_offset += bytes_to_write;
      Ok(bytes_to_write)
  }
  fn flush(self: &mut Self) -> io::Result<()> {
    return Ok(());
  }
}


fn benchmark_helper<Run: Runner>(input_slice: &[u8],
                                 compress_buffer_size: usize,
                                 decompress_buffer_size: usize,
                                 bench_compress: bool,
                                 bench_decompress: bool,
                                 bench: &mut Run,
                                 quality: i32,
) {
    let mut params = brotli::enc::BrotliEncoderInitParams();
    params.quality = quality;

    let mut input = UnlimitedBuffer::new(&input_slice[..]);
    let mut compressed_array = vec![0;input_slice.len() * 100/99];
    let mut rt_array = vec![0;input_slice.len() + 1];
    let mut compressed = LimitedBuffer::new(&mut compressed_array[..]);
    let mut rt = LimitedBuffer::new(&mut rt_array[..]);
    if !bench_compress {
        match compress(&mut input, &mut compressed, compress_buffer_size, &params) {
            Ok(_) => {}
            Err(e) => panic!("Error {:?}", e),
        }
    }
    bench.iter(&mut || {
        input.reset_read();
        if bench_compress {
            compressed.reset();
            match compress(&mut input, &mut compressed, compress_buffer_size, &params) {
                Ok(_) => {}
                Err(e) => panic!("Error {:?}", e),
            }
        }
        if bench_decompress {
            compressed.reset_read();
            rt.reset();
            match decompress(&mut compressed, &mut rt, decompress_buffer_size) {
                Ok(_) => {}
                Err(e) => panic!("Error {:?}", e),
            }
        }
    });
    if !bench_decompress {
        compressed.reset_read();
        rt.reset();
        match decompress(&mut compressed, &mut rt, decompress_buffer_size) {
            Ok(_) => {}
            Err(e) => panic!("Error {:?}", e),
        }
    }
    assert_eq!(rt.write_offset, input_slice.len());
    assert_eq!(&rt.data[..input_slice.len()],
               input_slice);
}

fn expand_test_data(size: usize) -> Vec<u8> {
    let mut ret = vec![0u8; size];
    let original_data = include_bytes!("testdata/random_then_unicode");
    let mut count = 0;
    let mut iter = 0usize;
    while count < size {
        let to_copy = core::cmp::min(size - count, original_data.len());
        let target = &mut ret[count..(count + to_copy)];
        for (dst, src) in target.iter_mut().zip(original_data[..to_copy].iter()) {
            *dst = src.wrapping_add(iter as u8);
        }
        count += to_copy;
        iter += 1;
    }
    ret
}

wrap_libtest! {
    fn bench_e2e_decode_q9_5_1024k(bench: &mut Bencher) {
        let td = expand_test_data(1024 * 1024);
        benchmark_helper(&td[..],
                        65536,
                        65536,
                        false,
                        true,
                        &mut BenchmarkPassthrough(bench),
                        11);
    }
}

wrap_libtest! {
    fn bench_e2e_decode_q5_1024k(bench: &mut Bencher) {
        let td = expand_test_data(1024 * 1024);
        benchmark_helper(&td[..],
                        65536,
                        65536,
                        false,
                        true,
                        &mut BenchmarkPassthrough(bench),
                        5);
    }
}

wrap_libtest! {
    fn bench_e2e_rt_q9_5_1024k(bench: &mut Bencher) {
        let td = expand_test_data(1024 * 1024);
        benchmark_helper(&td[..],
                        65536,
                        65536,
                        true,
                        true,
                        &mut BenchmarkPassthrough(bench),
                        11);
    }
}

wrap_libtest! {
    fn bench_e2e_rt_q9_1024k(bench: &mut Bencher) {
        let td = expand_test_data(1024 * 1024);
        benchmark_helper(&td[..],
                        65536,
                        65536,
                        true,
                        true,
                        &mut BenchmarkPassthrough(bench),
                        9);
    }
}

wrap_libtest! {
    fn bench_e2e_rt_q5_1024k(bench: &mut Bencher) {
        let td = expand_test_data(1024 * 1024);
        benchmark_helper(&td[..],
                        65536,
                        65536,
                        true,
                        true,
                        &mut BenchmarkPassthrough(bench),
                        5);
    }
}

pub fn write_one<T:SliceWrapper<u8>>(cmd: &brotli::interface::Command<T>) {
    use std::io::Write;
    match cmd {
        &brotli::interface::Command::BlockSwitchLiteral(ref bsl) => {
            println_stderr!("ltype {} {}", bsl.0.block_type(), bsl.1);
        },
        &brotli::interface::Command::BlockSwitchCommand(ref bsc) => {
            println_stderr!("ctype {}", bsc.0);
        },
        &brotli::interface::Command::BlockSwitchDistance(ref bsd) => {
            println_stderr!("dtype {}", bsd.0);
        },
        &brotli::interface::Command::PredictionMode(ref prediction) => {
            println_stderr!("prediction {} lcontextmap{} dcontextmap{}",
                            prediction_mode_str(prediction.literal_prediction_mode),
                            prediction.literal_context_map.slice().iter().fold(::std::string::String::new(),
                                                                               |res, &val| res + " " + &val.to_string()),
                            prediction.distance_context_map.slice().iter().fold(::std::string::String::new(),
                                                                                |res, &val| res + " " + &val.to_string()));
        },
        &brotli::interface::Command::Copy(ref copy) => {
            println_stderr!("copy {} from {}", copy.num_bytes, copy.distance);
        },
        &brotli::interface::Command::Dict(ref dict) => {
            let mut transformed_word = [0u8;38];
            let word_index = dict.word_id as usize * dict.word_size as usize +
                kBrotliDictionaryOffsetsByLength[dict.word_size as usize] as usize;
            let raw_word = &kBrotliDictionary[word_index..(word_index + dict.word_size as usize)];
            let actual_copy_len = TransformDictionaryWord(&mut transformed_word[..],
                                                          raw_word,
                                                          dict.word_size as i32,
                                                          dict.transform as i32) as usize;

            transformed_word.split_at(actual_copy_len).0;
            assert_eq!(dict.final_size as usize, actual_copy_len);
            println_stderr!("dict {} word {},{} {:x} func {} {:x}",
                            actual_copy_len,
                            dict.word_size,
                            dict.word_id,
                            SliceU8Ref(raw_word),
                            dict.transform,
                            SliceU8Ref(transformed_word.split_at(actual_copy_len).0));
        },
        &brotli::interface::Command::Literal(ref lit) => {
            println_stderr!("{} {} {:x}",
                            if lit.high_entropy {"rndins"} else {"insert"},
                            lit.data.slice().len(),
                            SliceU8Ref(lit.data.slice()));
        },
    }
}

struct SliceU8Ref<'a>(pub &'a[u8]);

impl<'a> fmt::LowerHex for SliceU8Ref<'a> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for item in self.0 {
            try!( fmtr.write_fmt(format_args!("{:02x}", item)));
        }
        Ok(())
    }
}



impl<'a, OutputType: Write> brotli::CustomWrite<io::Error> for IoWriterWrapper<'a, OutputType> {
  fn flush(self: &mut Self) -> Result<(), io::Error> {
    loop {
      match self.0.flush() {
        Err(e) => {
          match e.kind() {
            ErrorKind::Interrupted => continue,
            _ => return Err(e),
          }
        }
        Ok(_) => return Ok(()),
      }
    }
  }
  fn write(self: &mut Self, buf: &[u8]) -> Result<usize, io::Error> {
    loop {
      match self.0.write(buf) {
        Err(e) => {
          match e.kind() {
            ErrorKind::Interrupted => continue,
            _ => return Err(e),
          }
        }
        Ok(cur_written) => return Ok(cur_written),
      }
    }
  }
}


impl<'a, InputType: Read> brotli::CustomRead<io::Error> for IoReaderWrapper<'a, InputType> {
  fn read(self: &mut Self, buf: &mut [u8]) -> Result<usize, io::Error> {
    loop {
      match self.0.read(buf) {
        Err(e) => {
          match e.kind() {
            ErrorKind::Interrupted => continue,
            _ => return Err(e),
          }
        }
        Ok(cur_read) => return Ok(cur_read),
      }
    }
  }
}

fn prediction_mode_str(prediction_mode_nibble: brotli::interface::LiteralPredictionModeNibble) -> &'static str {
   match prediction_mode_nibble.prediction_mode() {
         brotli::interface::LITERAL_PREDICTION_MODE_SIGN => "sign",
         brotli::interface::LITERAL_PREDICTION_MODE_LSB6 => "lsb6",
         brotli::interface::LITERAL_PREDICTION_MODE_MSB6 => "msb6",
         brotli::interface::LITERAL_PREDICTION_MODE_UTF8 => "utf8",
         _ => "unknown",
   }
}
