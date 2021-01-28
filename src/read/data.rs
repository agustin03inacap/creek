use crate::{BLOCK_SIZE, NUM_PREFETCH_BLOCKS};

pub(crate) struct DataBlock {
    pub block: Vec<[f32; BLOCK_SIZE]>,
    pub starting_frame_in_file: usize,
    pub wanted_start_smp: usize,
}

impl DataBlock {
    pub(crate) fn new(num_channels: usize) -> Self {
        let mut block: Vec<[f32; BLOCK_SIZE]> = Vec::with_capacity(num_channels);
        for _ in 0..num_channels {
            // Safe because block will be always filled before it is sent to be read by the client.
            let data: [f32; BLOCK_SIZE] =
                unsafe { std::mem::MaybeUninit::<[f32; BLOCK_SIZE]>::uninit().assume_init() };
            block.push(data);
        }

        DataBlock {
            block,
            starting_frame_in_file: 0,
            wanted_start_smp: 0,
        }
    }
}

pub(crate) struct DataBlockCache {
    pub blocks: [DataBlock; NUM_PREFETCH_BLOCKS],
    pub wanted_start_smp: usize,
}

impl DataBlockCache {
    pub(crate) fn new(num_channels: usize) -> Self {
        // Safe because we initialize this in the next step.
        let mut blocks: [DataBlock; NUM_PREFETCH_BLOCKS] = unsafe {
            std::mem::MaybeUninit::<[DataBlock; NUM_PREFETCH_BLOCKS]>::uninit().assume_init()
        };

        for block in blocks.iter_mut() {
            *block = DataBlock::new(num_channels);
        }

        Self {
            blocks,
            wanted_start_smp: 0,
        }
    }
}

pub(crate) struct DataBlockEntry {
    pub use_cache: Option<usize>,
    pub block: Option<DataBlock>,
    pub wanted_start_smp: usize,
}

pub(crate) struct DataBlockCacheEntry {
    pub cache: Option<DataBlockCache>,
    pub wanted_start_smp: usize,
}

pub(crate) struct HeapData {
    pub read_buffer: DataBlock,
    pub prefetch_buffer: [DataBlockEntry; NUM_PREFETCH_BLOCKS],
    pub caches: Vec<DataBlockCacheEntry>,
}

/// The sample data returned by a `ReadClient`.
pub struct ReadData<'a> {
    data: &'a DataBlock,
    len: usize,
}

impl<'a> ReadData<'a> {
    pub(crate) fn new(data: &'a DataBlock, len: usize) -> Self {
        Self { data, len }
    }

    pub fn read_channel(&self, channel: usize) -> &[f32] {
        &self.data.block[channel][0..self.len]
    }

    pub fn num_channels(&self) -> usize {
        self.data.block.len()
    }

    pub fn buffer_len(&self) -> usize {
        self.len
    }
}