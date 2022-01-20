/* automatically generated by rust-bindgen 0.59.2 */

#[doc = " Vertex attribute stream, similar to glVertexPointer"]
#[doc = " Each element takes size bytes, with stride controlling the spacing between successive elements."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct meshopt_Stream {
    pub data: *const ::std::os::raw::c_void,
    pub size: usize,
    pub stride: usize,
}
extern "C" {
    #[doc = " Generates a vertex remap table from the vertex buffer and an optional index buffer and returns number of unique vertices"]
    #[doc = " As a result, all vertices that are binary equivalent map to the same (new) location, with no gaps in the resulting sequence."]
    #[doc = " Resulting remap table maps old vertices to new vertices and can be used in meshopt_remapVertexBuffer/meshopt_remapIndexBuffer."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting remap table (vertex_count elements)"]
    #[doc = " indices can be NULL if the input is unindexed"]
    pub fn meshopt_generateVertexRemap(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertices: *const ::std::os::raw::c_void,
        vertex_count: usize,
        vertex_size: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " Generates a vertex remap table from multiple vertex streams and an optional index buffer and returns number of unique vertices"]
    #[doc = " As a result, all vertices that are binary equivalent map to the same (new) location, with no gaps in the resulting sequence."]
    #[doc = " Resulting remap table maps old vertices to new vertices and can be used in meshopt_remapVertexBuffer/meshopt_remapIndexBuffer."]
    #[doc = " To remap vertex buffers, you will need to call meshopt_remapVertexBuffer for each vertex stream."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting remap table (vertex_count elements)"]
    #[doc = " indices can be NULL if the input is unindexed"]
    pub fn meshopt_generateVertexRemapMulti(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_count: usize,
        streams: *const meshopt_Stream,
        stream_count: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " Generates vertex buffer from the source vertex buffer and remap table generated by meshopt_generateVertexRemap"]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting vertex buffer (unique_vertex_count elements, returned by meshopt_generateVertexRemap)"]
    #[doc = " vertex_count should be the initial vertex count and not the value returned by meshopt_generateVertexRemap"]
    pub fn meshopt_remapVertexBuffer(
        destination: *mut ::std::os::raw::c_void,
        vertices: *const ::std::os::raw::c_void,
        vertex_count: usize,
        vertex_size: usize,
        remap: *const ::std::os::raw::c_uint,
    );
}
extern "C" {
    #[doc = " Generate index buffer from the source index buffer and remap table generated by meshopt_generateVertexRemap"]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting index buffer (index_count elements)"]
    #[doc = " indices can be NULL if the input is unindexed"]
    pub fn meshopt_remapIndexBuffer(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        remap: *const ::std::os::raw::c_uint,
    );
}
extern "C" {
    #[doc = " Generate index buffer that can be used for more efficient rendering when only a subset of the vertex attributes is necessary"]
    #[doc = " All vertices that are binary equivalent (wrt first vertex_size bytes) map to the first vertex in the original vertex buffer."]
    #[doc = " This makes it possible to use the index buffer for Z pre-pass or shadowmap rendering, while using the original index buffer for regular rendering."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting index buffer (index_count elements)"]
    pub fn meshopt_generateShadowIndexBuffer(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertices: *const ::std::os::raw::c_void,
        vertex_count: usize,
        vertex_size: usize,
        vertex_stride: usize,
    );
}
extern "C" {
    #[doc = " Generate index buffer that can be used for more efficient rendering when only a subset of the vertex attributes is necessary"]
    #[doc = " All vertices that are binary equivalent (wrt specified streams) map to the first vertex in the original vertex buffer."]
    #[doc = " This makes it possible to use the index buffer for Z pre-pass or shadowmap rendering, while using the original index buffer for regular rendering."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting index buffer (index_count elements)"]
    pub fn meshopt_generateShadowIndexBufferMulti(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_count: usize,
        streams: *const meshopt_Stream,
        stream_count: usize,
    );
}
extern "C" {
    #[doc = " Vertex transform cache optimizer"]
    #[doc = " Reorders indices to reduce the number of GPU vertex shader invocations"]
    #[doc = " If index buffer contains multiple ranges for multiple draw calls, this functions needs to be called on each range individually."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting index buffer (index_count elements)"]
    pub fn meshopt_optimizeVertexCache(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_count: usize,
    );
}
extern "C" {
    #[doc = " Vertex transform cache optimizer for FIFO caches"]
    #[doc = " Reorders indices to reduce the number of GPU vertex shader invocations"]
    #[doc = " Generally takes ~3x less time to optimize meshes but produces inferior results compared to meshopt_optimizeVertexCache"]
    #[doc = " If index buffer contains multiple ranges for multiple draw calls, this functions needs to be called on each range individually."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting index buffer (index_count elements)"]
    #[doc = " cache_size should be less than the actual GPU cache size to avoid cache thrashing"]
    pub fn meshopt_optimizeVertexCacheFifo(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_count: usize,
        cache_size: ::std::os::raw::c_uint,
    );
}
extern "C" {
    #[doc = " Overdraw optimizer"]
    #[doc = " Reorders indices to reduce the number of GPU vertex shader invocations and the pixel overdraw"]
    #[doc = " If index buffer contains multiple ranges for multiple draw calls, this functions needs to be called on each range individually."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting index buffer (index_count elements)"]
    #[doc = " indices must contain index data that is the result of meshopt_optimizeVertexCache (*not* the original mesh indices!)"]
    #[doc = " vertex_positions should have float3 position in the first 12 bytes of each vertex - similar to glVertexPointer"]
    #[doc = " threshold indicates how much the overdraw optimizer can degrade vertex cache efficiency (1.05 = up to 5%) to reduce overdraw more efficiently"]
    pub fn meshopt_optimizeOverdraw(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_positions: *const f32,
        vertex_count: usize,
        vertex_positions_stride: usize,
        threshold: f32,
    );
}
extern "C" {
    #[doc = " Vertex fetch cache optimizer"]
    #[doc = " Reorders vertices and changes indices to reduce the amount of GPU memory fetches during vertex processing"]
    #[doc = " Returns the number of unique vertices, which is the same as input vertex count unless some vertices are unused"]
    #[doc = " This functions works for a single vertex stream; for multiple vertex streams, use meshopt_optimizeVertexFetchRemap + meshopt_remapVertexBuffer for each stream."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting vertex buffer (vertex_count elements)"]
    #[doc = " indices is used both as an input and as an output index buffer"]
    pub fn meshopt_optimizeVertexFetch(
        destination: *mut ::std::os::raw::c_void,
        indices: *mut ::std::os::raw::c_uint,
        index_count: usize,
        vertices: *const ::std::os::raw::c_void,
        vertex_count: usize,
        vertex_size: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " Vertex fetch cache optimizer"]
    #[doc = " Generates vertex remap to reduce the amount of GPU memory fetches during vertex processing"]
    #[doc = " Returns the number of unique vertices, which is the same as input vertex count unless some vertices are unused"]
    #[doc = " The resulting remap table should be used to reorder vertex/index buffers using meshopt_remapVertexBuffer/meshopt_remapIndexBuffer"]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting remap table (vertex_count elements)"]
    pub fn meshopt_optimizeVertexFetchRemap(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_count: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " Index buffer encoder"]
    #[doc = " Encodes index data into an array of bytes that is generally much smaller (<1.5 bytes/triangle) and compresses better (<1 bytes/triangle) compared to original."]
    #[doc = " Returns encoded data size on success, 0 on error; the only error condition is if buffer doesn't have enough space"]
    #[doc = " For maximum efficiency the index buffer being encoded has to be optimized for vertex cache and vertex fetch first."]
    #[doc = ""]
    #[doc = " buffer must contain enough space for the encoded index buffer (use meshopt_encodeIndexBufferBound to compute worst case size)"]
    pub fn meshopt_encodeIndexBuffer(
        buffer: *mut ::std::os::raw::c_uchar,
        buffer_size: usize,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
    ) -> usize;
}
extern "C" {
    pub fn meshopt_encodeIndexBufferBound(index_count: usize, vertex_count: usize) -> usize;
}
extern "C" {
    #[doc = " Index buffer decoder"]
    #[doc = " Decodes index data from an array of bytes generated by meshopt_encodeIndexBuffer"]
    #[doc = " Returns 0 if decoding was successful, and an error code otherwise"]
    #[doc = " The decoder is safe to use for untrusted input, but it may produce garbage data (e.g. out of range indices)."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting index buffer (index_count elements)"]
    pub fn meshopt_decodeIndexBuffer(
        destination: *mut ::std::os::raw::c_void,
        index_count: usize,
        index_size: usize,
        buffer: *const ::std::os::raw::c_uchar,
        buffer_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Vertex buffer encoder"]
    #[doc = " Encodes vertex data into an array of bytes that is generally smaller and compresses better compared to original."]
    #[doc = " Returns encoded data size on success, 0 on error; the only error condition is if buffer doesn't have enough space"]
    #[doc = " This function works for a single vertex stream; for multiple vertex streams, call meshopt_encodeVertexBuffer for each stream."]
    #[doc = ""]
    #[doc = " buffer must contain enough space for the encoded vertex buffer (use meshopt_encodeVertexBufferBound to compute worst case size)"]
    pub fn meshopt_encodeVertexBuffer(
        buffer: *mut ::std::os::raw::c_uchar,
        buffer_size: usize,
        vertices: *const ::std::os::raw::c_void,
        vertex_count: usize,
        vertex_size: usize,
    ) -> usize;
}
extern "C" {
    pub fn meshopt_encodeVertexBufferBound(vertex_count: usize, vertex_size: usize) -> usize;
}
extern "C" {
    #[doc = " Vertex buffer decoder"]
    #[doc = " Decodes vertex data from an array of bytes generated by meshopt_encodeVertexBuffer"]
    #[doc = " Returns 0 if decoding was successful, and an error code otherwise"]
    #[doc = " The decoder is safe to use for untrusted input, but it may produce garbage data."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting vertex buffer (vertex_count * vertex_size bytes)"]
    pub fn meshopt_decodeVertexBuffer(
        destination: *mut ::std::os::raw::c_void,
        vertex_count: usize,
        vertex_size: usize,
        buffer: *const ::std::os::raw::c_uchar,
        buffer_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Experimental: Mesh simplifier"]
    #[doc = " Reduces the number of triangles in the mesh, attempting to preserve mesh appearance as much as possible"]
    #[doc = " The algorithm tries to preserve mesh topology and can stop short of the target goal based on topology constraints or target error."]
    #[doc = " If not all attributes from the input mesh are required, it's recommended to reindex the mesh using meshopt_generateShadowIndexBuffer prior to simplification."]
    #[doc = " Returns the number of indices after simplification, with destination containing new index data"]
    #[doc = " The resulting index buffer references vertices from the original vertex buffer."]
    #[doc = " If the original vertex data isn't required, creating a compact vertex buffer using meshopt_optimizeVertexFetch is recommended."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the *source* index buffer (since optimization is iterative, this means index_count elements - *not* target_index_count!)"]
    #[doc = " vertex_positions should have float3 position in the first 12 bytes of each vertex - similar to glVertexPointer"]
    pub fn meshopt_simplify(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_positions: *const f32,
        vertex_count: usize,
        vertex_positions_stride: usize,
        target_index_count: usize,
        target_error: f32,
    ) -> usize;
}
extern "C" {
    #[doc = " Experimental: Mesh simplifier (sloppy)"]
    #[doc = " Reduces the number of triangles in the mesh, sacrificing mesh apperance for simplification performance"]
    #[doc = " The algorithm doesn't preserve mesh topology but is always able to reach target triangle count."]
    #[doc = " Returns the number of indices after simplification, with destination containing new index data"]
    #[doc = " The resulting index buffer references vertices from the original vertex buffer."]
    #[doc = " If the original vertex data isn't required, creating a compact vertex buffer using meshopt_optimizeVertexFetch is recommended."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the target index buffer"]
    #[doc = " vertex_positions should have float3 position in the first 12 bytes of each vertex - similar to glVertexPointer"]
    pub fn meshopt_simplifySloppy(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_positions: *const f32,
        vertex_count: usize,
        vertex_positions_stride: usize,
        target_index_count: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " Experimental: Point cloud simplifier"]
    #[doc = " Reduces the number of points in the cloud to reach the given target"]
    #[doc = " Returns the number of points after simplification, with destination containing new index data"]
    #[doc = " The resulting index buffer references vertices from the original vertex buffer."]
    #[doc = " If the original vertex data isn't required, creating a compact vertex buffer using meshopt_optimizeVertexFetch is recommended."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the target index buffer"]
    #[doc = " vertex_positions should have float3 position in the first 12 bytes of each vertex - similar to glVertexPointer"]
    pub fn meshopt_simplifyPoints(
        destination: *mut ::std::os::raw::c_uint,
        vertex_positions: *const f32,
        vertex_count: usize,
        vertex_positions_stride: usize,
        target_vertex_count: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " Mesh stripifier"]
    #[doc = " Converts a previously vertex cache optimized triangle list to triangle strip, stitching strips using restart index or degenerate triangles"]
    #[doc = " Returns the number of indices in the resulting strip, with destination containing new index data"]
    #[doc = " For maximum efficiency the index buffer being converted has to be optimized for vertex cache first."]
    #[doc = " Using restart indices can result in ~10% smaller index buffers, but on some GPUs restart indices may result in decreased performance."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the target index buffer, worst case can be computed with meshopt_stripifyBound"]
    #[doc = " restart_index should be 0xffff or 0xffffffff depending on index size, or 0 to use degenerate triangles"]
    pub fn meshopt_stripify(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_count: usize,
        restart_index: ::std::os::raw::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn meshopt_stripifyBound(index_count: usize) -> usize;
}
extern "C" {
    #[doc = " Mesh unstripifier"]
    #[doc = " Converts a triangle strip to a triangle list"]
    #[doc = " Returns the number of indices in the resulting list, with destination containing new index data"]
    #[doc = ""]
    #[doc = " destination must contain enough space for the target index buffer, worst case can be computed with meshopt_unstripifyBound"]
    pub fn meshopt_unstripify(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        restart_index: ::std::os::raw::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn meshopt_unstripifyBound(index_count: usize) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct meshopt_VertexCacheStatistics {
    pub vertices_transformed: ::std::os::raw::c_uint,
    pub warps_executed: ::std::os::raw::c_uint,
    pub acmr: f32,
    pub atvr: f32,
}
extern "C" {
    #[doc = " Vertex transform cache analyzer"]
    #[doc = " Returns cache hit statistics using a simplified FIFO model"]
    #[doc = " Results may not match actual GPU performance"]
    pub fn meshopt_analyzeVertexCache(
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_count: usize,
        cache_size: ::std::os::raw::c_uint,
        warp_size: ::std::os::raw::c_uint,
        primgroup_size: ::std::os::raw::c_uint,
    ) -> meshopt_VertexCacheStatistics;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct meshopt_OverdrawStatistics {
    pub pixels_covered: ::std::os::raw::c_uint,
    pub pixels_shaded: ::std::os::raw::c_uint,
    pub overdraw: f32,
}
extern "C" {
    #[doc = " Overdraw analyzer"]
    #[doc = " Returns overdraw statistics using a software rasterizer"]
    #[doc = " Results may not match actual GPU performance"]
    #[doc = ""]
    #[doc = " vertex_positions should have float3 position in the first 12 bytes of each vertex - similar to glVertexPointer"]
    pub fn meshopt_analyzeOverdraw(
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_positions: *const f32,
        vertex_count: usize,
        vertex_positions_stride: usize,
    ) -> meshopt_OverdrawStatistics;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct meshopt_VertexFetchStatistics {
    pub bytes_fetched: ::std::os::raw::c_uint,
    pub overfetch: f32,
}
extern "C" {
    #[doc = " Vertex fetch cache analyzer"]
    #[doc = " Returns cache hit statistics using a simplified direct mapped model"]
    #[doc = " Results may not match actual GPU performance"]
    pub fn meshopt_analyzeVertexFetch(
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_count: usize,
        vertex_size: usize,
    ) -> meshopt_VertexFetchStatistics;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct meshopt_Meshlet {
    pub vertices: [::std::os::raw::c_uint; 64usize],
    pub indices: [[::std::os::raw::c_uchar; 3usize]; 126usize],
    pub triangle_count: ::std::os::raw::c_uchar,
    pub vertex_count: ::std::os::raw::c_uchar,
}
extern "C" {
    #[doc = " Experimental: Meshlet builder"]
    #[doc = " Splits the mesh into a set of meshlets where each meshlet has a micro index buffer indexing into meshlet vertices that refer to the original vertex buffer"]
    #[doc = " The resulting data can be used to render meshes using NVidia programmable mesh shading pipeline, or in other cluster-based renderers."]
    #[doc = " For maximum efficiency the index buffer being converted has to be optimized for vertex cache first."]
    #[doc = ""]
    #[doc = " destination must contain enough space for all meshlets, worst case size can be computed with meshopt_buildMeshletsBound"]
    #[doc = " max_vertices and max_triangles can't exceed limits statically declared in meshopt_Meshlet (max_vertices <= 64, max_triangles <= 126)"]
    pub fn meshopt_buildMeshlets(
        destination: *mut meshopt_Meshlet,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_count: usize,
        max_vertices: usize,
        max_triangles: usize,
    ) -> usize;
}
extern "C" {
    pub fn meshopt_buildMeshletsBound(
        index_count: usize,
        max_vertices: usize,
        max_triangles: usize,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct meshopt_Bounds {
    pub center: [f32; 3usize],
    pub radius: f32,
    pub cone_apex: [f32; 3usize],
    pub cone_axis: [f32; 3usize],
    pub cone_cutoff: f32,
    pub cone_axis_s8: [::std::os::raw::c_schar; 3usize],
    pub cone_cutoff_s8: ::std::os::raw::c_schar,
}
extern "C" {
    #[doc = " Experimental: Cluster bounds generator"]
    #[doc = " Creates bounding volumes that can be used for frustum, backface and occlusion culling."]
    #[doc = ""]
    #[doc = " For backface culling with orthographic projection, use the following formula to reject backfacing clusters:"]
    #[doc = "   dot(view, cone_axis) >= cone_cutoff"]
    #[doc = ""]
    #[doc = " For perspective projection, you can the formula that needs cone apex in addition to axis & cutoff:"]
    #[doc = "   dot(normalize(cone_apex - camera_position), cone_axis) >= cone_cutoff"]
    #[doc = ""]
    #[doc = " Alternatively, you can use the formula that doesn't need cone apex and uses bounding sphere instead:"]
    #[doc = "   dot(normalize(center - camera_position), cone_axis) >= cone_cutoff + radius / length(center - camera_position)"]
    #[doc = " or an equivalent formula that doesn't have a singularity at center = camera_position:"]
    #[doc = "   dot(center - camera_position, cone_axis) >= cone_cutoff * length(center - camera_position) + radius"]
    #[doc = ""]
    #[doc = " The formula that uses the apex is slightly more accurate but needs the apex; if you are already using bounding sphere"]
    #[doc = " to do frustum/occlusion culling, the formula that doesn't use the apex may be preferable."]
    #[doc = ""]
    #[doc = " vertex_positions should have float3 position in the first 12 bytes of each vertex - similar to glVertexPointer"]
    #[doc = " index_count should be less than or equal to 256*3 (the function assumes clusters of limited size)"]
    pub fn meshopt_computeClusterBounds(
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_positions: *const f32,
        vertex_count: usize,
        vertex_positions_stride: usize,
    ) -> meshopt_Bounds;
}
extern "C" {
    pub fn meshopt_computeMeshletBounds(
        meshlet: *const meshopt_Meshlet,
        vertex_positions: *const f32,
        vertex_count: usize,
        vertex_positions_stride: usize,
    ) -> meshopt_Bounds;
}
extern "C" {
    #[doc = " Experimental: Spatial sorter"]
    #[doc = " Generates a remap table that can be used to reorder points for spatial locality."]
    #[doc = " Resulting remap table maps old vertices to new vertices and can be used in meshopt_remapVertexBuffer."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting remap table (vertex_count elements)"]
    pub fn meshopt_spatialSortRemap(
        destination: *mut ::std::os::raw::c_uint,
        vertex_positions: *const f32,
        vertex_count: usize,
        vertex_positions_stride: usize,
    );
}
extern "C" {
    #[doc = " Experimental: Spatial sorter"]
    #[doc = " Reorders triangles for spatial locality, and generates a new index buffer. The resulting index buffer can be used with other functions like optimizeVertexCache."]
    #[doc = ""]
    #[doc = " destination must contain enough space for the resulting index buffer (index_count elements)"]
    #[doc = " indices must contain index data that is the result of meshopt_optimizeVertexCache (*not* the original mesh indices!)"]
    #[doc = " vertex_positions should have float3 position in the first 12 bytes of each vertex - similar to glVertexPointer"]
    pub fn meshopt_spatialSortTriangles(
        destination: *mut ::std::os::raw::c_uint,
        indices: *const ::std::os::raw::c_uint,
        index_count: usize,
        vertex_positions: *const f32,
        vertex_count: usize,
        vertex_positions_stride: usize,
    );
}
extern "C" {
    #[doc = " Set allocation callbacks"]
    #[doc = " These callbacks will be used instead of the default operator new/operator delete for all temporary allocations in the library."]
    #[doc = " Note that all algorithms only allocate memory for temporary use."]
    #[doc = " allocate/deallocate are always called in a stack-like order - last pointer to be allocated is deallocated first."]
    pub fn meshopt_setAllocator(
        allocate: ::std::option::Option<
            unsafe extern "C" fn(arg1: usize) -> *mut ::std::os::raw::c_void,
        >,
        deallocate: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
