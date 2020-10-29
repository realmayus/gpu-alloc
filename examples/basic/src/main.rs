use {
    galloc::{
        Config, Dedicated, DeviceProperties, GpuAllocator, MemoryHeap, MemoryPropertyFlags,
        MemoryType, Request, UsageFlags,
    },
    galloc_mock::MockMemoryDevice,
};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let device = MockMemoryDevice::new(DeviceProperties {
        memory_types: &[MemoryType {
            heap: 0,
            props: MemoryPropertyFlags::HOST_VISIBLE,
        }],
        memory_heaps: &[MemoryHeap { size: 1024 * 1024 }],
        max_memory_allocation_count: 32,
        max_memory_allocation_size: 1024 * 1024,
        non_coherent_atom_size: 8,
    });

    let config = Config::i_am_potato();

    let mut allocator = GpuAllocator::new(config, device.props());

    // dbg!(&allocator);

    let block = unsafe {
        allocator.alloc(
            &device,
            Request {
                size: 10,
                align_mask: 1,
                usage: UsageFlags::empty(),
                memory_types: 1,
                dedicated: Dedicated::Indifferent,
            },
        )
    }?;

    unsafe { block.write_bytes(&device, 0, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]) }?;

    Ok(())
}
