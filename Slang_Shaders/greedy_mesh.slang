module greedyMesher;

#include "gm_utils.slang"
static const uint GRID_SIZE_X = 64; // Grid Size Axis X
static const uint GRID_SIZE_Y = 64; // Grid Size Axis Y
static const uint TOTAL_SIZE = GRID_SIZE_X * GRID_SIZE_Y; // Grid Size 
static const uint TILE_SIZE = 16; // work-group
static const uint MAX_QUADS = 512; // worst-case-senario

// Merged Quad Data
struct Quad {
    int value;  // Identifier
    int2 start; // Top-left Cell Coord
    int2 size;  // Dimensions of Quad
};

// Output Buffer for Merged Quads (SSBO)
RWStructuredBuffer<Quad> quads : register(u0);
// Resource for counting Quads Safely
RWByteAddressBuffer QuadCounter : register(u2);
// Accesses the texture
readonly Texture2D<int> cells : register(t0);

// Group Shared Memory
groupshared int sharedCells[TILE_SIZE][TILE_SIZE];
groupshared Quad localQuads[MAX_QUADS];
groupshared uint localCount;

namespace greedyMesher {

[numthreads(TILE_SIZE, TILE_SIZE, 1)]
void greedyMeshing(
    uint3 DTid: SV_DispatchThreadID,
    uint3 GTid: SV_GroupThreadID,
    uint3 GIndex: SV_GroupID,
    readonly Texture2D<float4> InTex,
    uniform uint2 InDims,
) {
    // Load one TILE_SIZE times TILE_SIZE block into shared mem
    uint gx = GIndex.x * TILE_SIZE + GTid.x;
    uint gy = GIndex.y * TILE_SIZE + GTid.y;
    int v = 0;
    if (gx < GRID_SIZE_X && gy < GRID_SIZE_Y) {
        v = cells.Load(int3(gx, gy, 0));
    }
    sharedCells[GTid.y][GTid.x] = v;
    GroupMemoryBarrierWithGroupSync();
    // Clear per-group counter (once)
    if (all(GTid.xy == 0)) {
        localCount = 0;
    }
    GroupMemoryBarrierWithGroupSync();
    // Each thread runs a tiny greedy meson its own cell
    if (gx < GRID_SIZE_X && gy < GRID_SIZE_Y && v != 0) {
        // skip if merged already
        Quad q;
        q.value = v;
        q.start = int2(gx, gy);
        q.size = int2(1, 1);
        // stash in localQuads[localCount++]
        if (localCount + 1 < MAX_QUADS) {
            localQuads[localCount + 1] = q;
        }
    }
    GroupMemoryBarrierWithGroupSync();
    // One Thread reserves space for dumping it's batch in the global buffer
    if (all(GTid.xy == 0)) {
        // Reserve localCount slots
        uint baseOffset;
        QuadCounter.InterlockedAdd(0, localCount, baseOffset);
        // Copy all quads in one burst
        for (uint i = 0; i < localCount; ++i) {
            quads[baseOffset + i] = localQuads[i];
        }
    }
}
}