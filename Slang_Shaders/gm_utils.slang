module gm_utils;

namespace gm_utils {
export void clearMerged(uint mergedMask[]) {
    [unroll]
    for (uint i = 0; i < countof(mergedMask); ++i) {
        mergedMask[i] = 0;
    }
}

export inline bool isMerged(
    uint x,
    uint y,
    uint* mergedMask
) {
    uint idx = y * 64 + x;
    return (mergedMask[idx >> 5] & (1u << (idx & 31))) != 0;
}

export inline void setMerged(
    uint x,
    uint y,
    uint* mergedMask
) {
    uint idx = y * 64 + x;
    mergedMask[idx / 32] |= (1u << (idx & 31));
}

export inline Ptr<uint> ptrConverterUint(uint array[]) {
    return &array[0];
}
}
