#language slang legacy

#include "greedy_mesh.slang"
#include "upscaler.slang"


// Source Image (2D texture)
readonly Texture2D<float4> InTex : register(t1);
// Read Write Texture for writing to the Screen
RWTexture2D<float4> OutTex : register(u1);

// Adding Screen Resizing for the Fun of it
// Holds Scale Factor + If Bilinear Interpolation is going to be used
cbuffer ResizeParams : register(b0) {
    float2 params; // x: scale factor, y: useBilinear (0.0 or 1.0)
};

// Group Shared Variable sotring the Input Textures Dimensions
// This is for Accessing Values without Querying the Texture
groupshared uint2 dims[2];



// Compute Shader
[shader("compute")]
// Thread Group of size 256
[numthreads(16, 16, 1)]
// Inputs:
//   - dtid: The Unique Output Coordinate
//   - gtid: Local Calculations
//   - ltID: The Local Thread Position
func computeMain(
    uint3 dtid: SV_DispatchThreadID,
    uint3 gtid : SV_GroupThreadID,
    uint3 ltID: SV_GroupThreadID,
    )->void {
    // Only 1 Thread per group gets the Texture Dimensions
    if (gtid.x == 0 && gtid.y == 0) {
        uint2 inputDim, outputDim;
        InTex.GetDimensions(inputDim.x, inputDim.y);
        OutTex.GetDimensions(outputDim.x, outputDim.y);
        dims[0] = inputDim;
        dims[1] = outputDim;
    }
    // Makes sure all Threads have the groupshared variables
    GroupMemoryBarrierWithGroupSync();
    // Checks if Coordinates are in Bounds ; Early Exit if yes
    if (dtid.x >= dims[1].x || dtid.y >= dims[1].y) return;
    greedyMesher::greedyMeshing(
        dtid,
        gtid,
        ltID,
        InTex,
        dims[0],
    );

    // Get Parameters
    float scaleFactor = params.x;
    float useBilinear = params.y;
    // If scaleFactor is 1, simply display the picture
    if (scaleFactor == 1.0) {
        OutTex[dtid.xy] = InTex[dtid.xy];
        return;
    }
    upscaler::bipolation_fn(
        useBilinear,
        scaleFactor,
        dtid,
        InTex,
        OutTex,
        dims
    );
}