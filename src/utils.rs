pub const fn offset_align_to(offset: usize, alignment: usize) -> usize {
    let remainder = offset % alignment;
    if remainder != 0 {
        return offset - remainder + alignment;
    }

    offset
}
