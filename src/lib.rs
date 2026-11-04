//! `Supports<IndirectDraw>` for backends that translate [`RhiCommand::Extension`].
//!
//! The payload is little-endian: buffer id `u64`, offset `u64`, then the indexed
//! draw arguments `index_count`, `instance_count`, `first_index` as `u32`,
//! `vertex_offset` as `i32`, and `first_instance` as `u32`. Opcode [`OPCODE`]
//! selects indexed indirect draw. The GPU buffer already holds the native
//! indirect arguments; the payload only names that buffer.

use rotex_core::{CommandEncoder, Feature, Supports};
use rotex_types::{BufferId, FeatureId, IndirectDraw};

pub const OPCODE: u32 = 1;

pub struct IndexedIndirectDraw {
    pub buffer: BufferId,
    pub offset: u64,
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

pub fn draw_indexed_indirect<B>(
    encoder: &mut CommandEncoder<B>,
    draw: IndexedIndirectDraw,
) -> &mut CommandEncoder<B>
where
    B: Supports<IndirectDraw>,
{
    encoder.extension(IndirectDraw::ID, OPCODE, encode(&draw))
}

pub fn encode(draw: &IndexedIndirectDraw) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(36);
    bytes.extend_from_slice(&draw.buffer.0.to_le_bytes());
    bytes.extend_from_slice(&draw.offset.to_le_bytes());
    bytes.extend_from_slice(&draw.index_count.to_le_bytes());
    bytes.extend_from_slice(&draw.instance_count.to_le_bytes());
    bytes.extend_from_slice(&draw.first_index.to_le_bytes());
    bytes.extend_from_slice(&draw.vertex_offset.to_le_bytes());
    bytes.extend_from_slice(&draw.first_instance.to_le_bytes());
    bytes
}

pub fn feature_id() -> FeatureId {
    IndirectDraw::ID
}

// `Supports` is defined in rotex-core and the bridge types live in the backends,
// so the marker impls are in those crates. These features only prove the impls
// are present.
#[cfg(feature = "vulkan")]
#[allow(dead_code)]
fn _vulkan_supports_indirect() {
    fn assert_supports<T: Supports<IndirectDraw>>() {}
    assert_supports::<rotex_vulkan::VulkanBridge>();
}

#[cfg(feature = "wgpu")]
#[allow(dead_code)]
fn _wgpu_supports_indirect() {
    fn assert_supports<T: Supports<IndirectDraw>>() {}
    assert_supports::<rotex_wgpu::WgpuBridge>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rotex_types::RhiCommand;

    #[test]
    fn payload_starts_with_buffer_and_offset() {
        let bytes = encode(&IndexedIndirectDraw {
            buffer: BufferId(9),
            offset: 32,
            index_count: 3,
            instance_count: 1,
            first_index: 0,
            vertex_offset: -1,
            first_instance: 2,
        });
        assert_eq!(bytes.len(), 36);
        assert_eq!(&bytes[0..8], &9u64.to_le_bytes());
        assert_eq!(&bytes[8..16], &32u64.to_le_bytes());
        let _ = RhiCommand::Extension {
            feature: feature_id(),
            opcode: OPCODE,
            payload: bytes,
        };
    }
}
