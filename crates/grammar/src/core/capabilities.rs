use super::preamble::*;
pub const CAPABILITY_MATRIX: Capability = Capability::new("Matrix");
pub const CAPABILITY_SHADER: Capability = Capability::new("Shader");
pub const CAPABILITY_GEOMETRY: Capability = Capability::new("Geometry");
pub const CAPABILITY_TESSELLATION: Capability = Capability::new("Tessellation");
pub const CAPABILITY_ADDRESSES: Capability = Capability::new("Addresses");
pub const CAPABILITY_LINKAGE: Capability = Capability::new("Linkage");
pub const CAPABILITY_KERNEL: Capability = Capability::new("Kernel");
pub const CAPABILITY_VECTOR_16: Capability = Capability::new("Vector16");
pub const CAPABILITY_FLOAT_16_BUFFER: Capability = Capability::new("Float16Buffer");
pub const CAPABILITY_FLOAT_16: Capability = Capability::new("Float16");
pub const CAPABILITY_FLOAT_64: Capability = Capability::new("Float64");
pub const CAPABILITY_INT_64: Capability = Capability::new("Int64");
pub const CAPABILITY_INT_64_ATOMICS: Capability = Capability::new("Int64Atomics");
pub const CAPABILITY_IMAGE_BASIC: Capability = Capability::new("ImageBasic");
pub const CAPABILITY_IMAGE_READ_WRITE: Capability = Capability::new("ImageReadWrite");
pub const CAPABILITY_IMAGE_MIPMAP: Capability = Capability::new("ImageMipmap");
pub const CAPABILITY_PIPES: Capability = Capability::new("Pipes");
pub const CAPABILITY_GROUPS: Capability = Capability::new("Groups");
pub const CAPABILITY_DEVICE_ENQUEUE: Capability = Capability::new("DeviceEnqueue");
pub const CAPABILITY_LITERAL_SAMPLER: Capability = Capability::new("LiteralSampler");
pub const CAPABILITY_ATOMIC_STORAGE: Capability = Capability::new("AtomicStorage");
pub const CAPABILITY_INT_16: Capability = Capability::new("Int16");
pub const CAPABILITY_TESSELLATION_POINT_SIZE: Capability = Capability::new("TessellationPointSize");
pub const CAPABILITY_GEOMETRY_POINT_SIZE: Capability = Capability::new("GeometryPointSize");
pub const CAPABILITY_IMAGE_GATHER_EXTENDED: Capability = Capability::new("ImageGatherExtended");
pub const CAPABILITY_STORAGE_IMAGE_MULTISAMPLE: Capability =
    Capability::new("StorageImageMultisample");
pub const CAPABILITY_UNIFORM_BUFFER_ARRAY_DYNAMIC_INDEXING: Capability =
    Capability::new("UniformBufferArrayDynamicIndexing");
pub const CAPABILITY_SAMPLED_IMAGE_ARRAY_DYNAMIC_INDEXING: Capability =
    Capability::new("SampledImageArrayDynamicIndexing");
pub const CAPABILITY_STORAGE_BUFFER_ARRAY_DYNAMIC_INDEXING: Capability =
    Capability::new("StorageBufferArrayDynamicIndexing");
pub const CAPABILITY_STORAGE_IMAGE_ARRAY_DYNAMIC_INDEXING: Capability =
    Capability::new("StorageImageArrayDynamicIndexing");
pub const CAPABILITY_CLIP_DISTANCE: Capability = Capability::new("ClipDistance");
pub const CAPABILITY_CULL_DISTANCE: Capability = Capability::new("CullDistance");
pub const CAPABILITY_IMAGE_CUBE_ARRAY: Capability = Capability::new("ImageCubeArray");
pub const CAPABILITY_SAMPLE_RATE_SHADING: Capability = Capability::new("SampleRateShading");
pub const CAPABILITY_IMAGE_RECT: Capability = Capability::new("ImageRect");
pub const CAPABILITY_SAMPLED_RECT: Capability = Capability::new("SampledRect");
pub const CAPABILITY_GENERIC_POINTER: Capability = Capability::new("GenericPointer");
pub const CAPABILITY_INT_8: Capability = Capability::new("Int8");
pub const CAPABILITY_INPUT_ATTACHMENT: Capability = Capability::new("InputAttachment");
pub const CAPABILITY_SPARSE_RESIDENCY: Capability = Capability::new("SparseResidency");
pub const CAPABILITY_MIN_LOD: Capability = Capability::new("MinLod");
pub const CAPABILITY_SAMPLED_1_D: Capability = Capability::new("Sampled1D");
pub const CAPABILITY_IMAGE_1_D: Capability = Capability::new("Image1D");
pub const CAPABILITY_SAMPLED_CUBE_ARRAY: Capability = Capability::new("SampledCubeArray");
pub const CAPABILITY_SAMPLED_BUFFER: Capability = Capability::new("SampledBuffer");
pub const CAPABILITY_IMAGE_BUFFER: Capability = Capability::new("ImageBuffer");
pub const CAPABILITY_IMAGE_MS_ARRAY: Capability = Capability::new("ImageMSArray");
pub const CAPABILITY_STORAGE_IMAGE_EXTENDED_FORMATS: Capability =
    Capability::new("StorageImageExtendedFormats");
pub const CAPABILITY_IMAGE_QUERY: Capability = Capability::new("ImageQuery");
pub const CAPABILITY_DERIVATIVE_CONTROL: Capability = Capability::new("DerivativeControl");
pub const CAPABILITY_INTERPOLATION_FUNCTION: Capability = Capability::new("InterpolationFunction");
pub const CAPABILITY_TRANSFORM_FEEDBACK: Capability = Capability::new("TransformFeedback");
pub const CAPABILITY_GEOMETRY_STREAMS: Capability = Capability::new("GeometryStreams");
pub const CAPABILITY_STORAGE_IMAGE_READ_WITHOUT_FORMAT: Capability =
    Capability::new("StorageImageReadWithoutFormat");
pub const CAPABILITY_STORAGE_IMAGE_WRITE_WITHOUT_FORMAT: Capability =
    Capability::new("StorageImageWriteWithoutFormat");
pub const CAPABILITY_MULTI_VIEWPORT: Capability = Capability::new("MultiViewport");
pub const CAPABILITY_SUBGROUP_DISPATCH: Capability = Capability::new("SubgroupDispatch");
pub const CAPABILITY_NAMED_BARRIER: Capability = Capability::new("NamedBarrier");
pub const CAPABILITY_PIPE_STORAGE: Capability = Capability::new("PipeStorage");
pub const CAPABILITY_GROUP_NON_UNIFORM: Capability = Capability::new("GroupNonUniform");
pub const CAPABILITY_GROUP_NON_UNIFORM_VOTE: Capability = Capability::new("GroupNonUniformVote");
pub const CAPABILITY_GROUP_NON_UNIFORM_ARITHMETIC: Capability =
    Capability::new("GroupNonUniformArithmetic");
pub const CAPABILITY_GROUP_NON_UNIFORM_BALLOT: Capability =
    Capability::new("GroupNonUniformBallot");
pub const CAPABILITY_GROUP_NON_UNIFORM_SHUFFLE: Capability =
    Capability::new("GroupNonUniformShuffle");
pub const CAPABILITY_GROUP_NON_UNIFORM_SHUFFLE_RELATIVE: Capability =
    Capability::new("GroupNonUniformShuffleRelative");
pub const CAPABILITY_GROUP_NON_UNIFORM_CLUSTERED: Capability =
    Capability::new("GroupNonUniformClustered");
pub const CAPABILITY_GROUP_NON_UNIFORM_QUAD: Capability = Capability::new("GroupNonUniformQuad");
pub const CAPABILITY_SHADER_LAYER: Capability = Capability::new("ShaderLayer");
pub const CAPABILITY_SHADER_VIEWPORT_INDEX: Capability = Capability::new("ShaderViewportIndex");
pub const CAPABILITY_UNIFORM_DECORATION: Capability = Capability::new("UniformDecoration");
pub const CAPABILITY_CORE_BUILTINS_ARM: Capability = Capability::new("CoreBuiltinsARM");
pub const CAPABILITY_TILE_IMAGE_COLOR_READ_ACCESS_EXT: Capability =
    Capability::new("TileImageColorReadAccessEXT");
pub const CAPABILITY_TILE_IMAGE_DEPTH_READ_ACCESS_EXT: Capability =
    Capability::new("TileImageDepthReadAccessEXT");
pub const CAPABILITY_TILE_IMAGE_STENCIL_READ_ACCESS_EXT: Capability =
    Capability::new("TileImageStencilReadAccessEXT");
pub const CAPABILITY_TENSORS_ARM: Capability = Capability::new("TensorsARM");
pub const CAPABILITY_STORAGE_TENSOR_ARRAY_DYNAMIC_INDEXING_ARM: Capability =
    Capability::new("StorageTensorArrayDynamicIndexingARM");
pub const CAPABILITY_STORAGE_TENSOR_ARRAY_NON_UNIFORM_INDEXING_ARM: Capability =
    Capability::new("StorageTensorArrayNonUniformIndexingARM");
pub const CAPABILITY_GRAPH_ARM: Capability = Capability::new("GraphARM");
pub const CAPABILITY_COOPERATIVE_MATRIX_LAYOUTS_ARM: Capability =
    Capability::new("CooperativeMatrixLayoutsARM");
pub const CAPABILITY_FLOAT_8_EXT: Capability = Capability::new("Float8EXT");
pub const CAPABILITY_FLOAT_8_COOPERATIVE_MATRIX_EXT: Capability =
    Capability::new("Float8CooperativeMatrixEXT");
pub const CAPABILITY_FRAGMENT_SHADING_RATE_KHR: Capability =
    Capability::new("FragmentShadingRateKHR");
pub const CAPABILITY_SUBGROUP_BALLOT_KHR: Capability = Capability::new("SubgroupBallotKHR");
pub const CAPABILITY_DRAW_PARAMETERS: Capability = Capability::new("DrawParameters");
pub const CAPABILITY_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_KHR: Capability =
    Capability::new("WorkgroupMemoryExplicitLayoutKHR");
pub const CAPABILITY_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_8_BIT_ACCESS_KHR: Capability =
    Capability::new("WorkgroupMemoryExplicitLayout8BitAccessKHR");
pub const CAPABILITY_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_16_BIT_ACCESS_KHR: Capability =
    Capability::new("WorkgroupMemoryExplicitLayout16BitAccessKHR");
pub const CAPABILITY_SUBGROUP_VOTE_KHR: Capability = Capability::new("SubgroupVoteKHR");
pub const CAPABILITY_STORAGE_BUFFER_16_BIT_ACCESS: Capability =
    Capability::new("StorageBuffer16BitAccess");
pub const CAPABILITY_UNIFORM_AND_STORAGE_BUFFER_16_BIT_ACCESS: Capability =
    Capability::new("UniformAndStorageBuffer16BitAccess");
pub const CAPABILITY_STORAGE_PUSH_CONSTANT_16: Capability =
    Capability::new("StoragePushConstant16");
pub const CAPABILITY_STORAGE_INPUT_OUTPUT_16: Capability = Capability::new("StorageInputOutput16");
pub const CAPABILITY_DEVICE_GROUP: Capability = Capability::new("DeviceGroup");
pub const CAPABILITY_MULTI_VIEW: Capability = Capability::new("MultiView");
pub const CAPABILITY_VARIABLE_POINTERS_STORAGE_BUFFER: Capability =
    Capability::new("VariablePointersStorageBuffer");
pub const CAPABILITY_VARIABLE_POINTERS: Capability = Capability::new("VariablePointers");
pub const CAPABILITY_ATOMIC_STORAGE_OPS: Capability = Capability::new("AtomicStorageOps");
pub const CAPABILITY_SAMPLE_MASK_POST_DEPTH_COVERAGE: Capability =
    Capability::new("SampleMaskPostDepthCoverage");
pub const CAPABILITY_STORAGE_BUFFER_8_BIT_ACCESS: Capability =
    Capability::new("StorageBuffer8BitAccess");
pub const CAPABILITY_UNIFORM_AND_STORAGE_BUFFER_8_BIT_ACCESS: Capability =
    Capability::new("UniformAndStorageBuffer8BitAccess");
pub const CAPABILITY_STORAGE_PUSH_CONSTANT_8: Capability = Capability::new("StoragePushConstant8");
pub const CAPABILITY_DENORM_PRESERVE: Capability = Capability::new("DenormPreserve");
pub const CAPABILITY_DENORM_FLUSH_TO_ZERO: Capability = Capability::new("DenormFlushToZero");
pub const CAPABILITY_SIGNED_ZERO_INF_NAN_PRESERVE: Capability =
    Capability::new("SignedZeroInfNanPreserve");
pub const CAPABILITY_ROUNDING_MODE_RTE: Capability = Capability::new("RoundingModeRTE");
pub const CAPABILITY_ROUNDING_MODE_RTZ: Capability = Capability::new("RoundingModeRTZ");
pub const CAPABILITY_RAY_QUERY_PROVISIONAL_KHR: Capability =
    Capability::new("RayQueryProvisionalKHR");
pub const CAPABILITY_RAY_QUERY_KHR: Capability = Capability::new("RayQueryKHR");
pub const CAPABILITY_UNTYPED_POINTERS_KHR: Capability = Capability::new("UntypedPointersKHR");
pub const CAPABILITY_RAY_TRAVERSAL_PRIMITIVE_CULLING_KHR: Capability =
    Capability::new("RayTraversalPrimitiveCullingKHR");
pub const CAPABILITY_RAY_TRACING_KHR: Capability = Capability::new("RayTracingKHR");
pub const CAPABILITY_TEXTURE_SAMPLE_WEIGHTED_QCOM: Capability =
    Capability::new("TextureSampleWeightedQCOM");
pub const CAPABILITY_TEXTURE_BOX_FILTER_QCOM: Capability = Capability::new("TextureBoxFilterQCOM");
pub const CAPABILITY_TEXTURE_BLOCK_MATCH_QCOM: Capability =
    Capability::new("TextureBlockMatchQCOM");
pub const CAPABILITY_TILE_SHADING_QCOM: Capability = Capability::new("TileShadingQCOM");
pub const CAPABILITY_COOPERATIVE_MATRIX_CONVERSION_QCOM: Capability =
    Capability::new("CooperativeMatrixConversionQCOM");
pub const CAPABILITY_TEXTURE_BLOCK_MATCH_2_QCOM: Capability =
    Capability::new("TextureBlockMatch2QCOM");
pub const CAPABILITY_FLOAT_16_IMAGE_AMD: Capability = Capability::new("Float16ImageAMD");
pub const CAPABILITY_IMAGE_GATHER_BIAS_LOD_AMD: Capability =
    Capability::new("ImageGatherBiasLodAMD");
pub const CAPABILITY_FRAGMENT_MASK_AMD: Capability = Capability::new("FragmentMaskAMD");
pub const CAPABILITY_STENCIL_EXPORT_EXT: Capability = Capability::new("StencilExportEXT");
pub const CAPABILITY_IMAGE_READ_WRITE_LOD_AMD: Capability = Capability::new("ImageReadWriteLodAMD");
pub const CAPABILITY_INT_64_IMAGE_EXT: Capability = Capability::new("Int64ImageEXT");
pub const CAPABILITY_SHADER_CLOCK_KHR: Capability = Capability::new("ShaderClockKHR");
pub const CAPABILITY_SHADER_ENQUEUE_AMDX: Capability = Capability::new("ShaderEnqueueAMDX");
pub const CAPABILITY_QUAD_CONTROL_KHR: Capability = Capability::new("QuadControlKHR");
pub const CAPABILITY_INT_4_TYPE_INTEL: Capability = Capability::new("Int4TypeINTEL");
pub const CAPABILITY_INT_4_COOPERATIVE_MATRIX_INTEL: Capability =
    Capability::new("Int4CooperativeMatrixINTEL");
pub const CAPABILITY_B_FLOAT_16_TYPE_KHR: Capability = Capability::new("BFloat16TypeKHR");
pub const CAPABILITY_B_FLOAT_16_DOT_PRODUCT_KHR: Capability =
    Capability::new("BFloat16DotProductKHR");
pub const CAPABILITY_B_FLOAT_16_COOPERATIVE_MATRIX_KHR: Capability =
    Capability::new("BFloat16CooperativeMatrixKHR");
pub const CAPABILITY_DESCRIPTOR_HEAP_EXT: Capability = Capability::new("DescriptorHeapEXT");
pub const CAPABILITY_SAMPLE_MASK_OVERRIDE_COVERAGE_NV: Capability =
    Capability::new("SampleMaskOverrideCoverageNV");
pub const CAPABILITY_GEOMETRY_SHADER_PASSTHROUGH_NV: Capability =
    Capability::new("GeometryShaderPassthroughNV");
pub const CAPABILITY_SHADER_VIEWPORT_INDEX_LAYER_EXT: Capability =
    Capability::new("ShaderViewportIndexLayerEXT");
pub const CAPABILITY_SHADER_VIEWPORT_MASK_NV: Capability = Capability::new("ShaderViewportMaskNV");
pub const CAPABILITY_SHADER_STEREO_VIEW_NV: Capability = Capability::new("ShaderStereoViewNV");
pub const CAPABILITY_PER_VIEW_ATTRIBUTES_NV: Capability = Capability::new("PerViewAttributesNV");
pub const CAPABILITY_FRAGMENT_FULLY_COVERED_EXT: Capability =
    Capability::new("FragmentFullyCoveredEXT");
pub const CAPABILITY_MESH_SHADING_NV: Capability = Capability::new("MeshShadingNV");
pub const CAPABILITY_IMAGE_FOOTPRINT_NV: Capability = Capability::new("ImageFootprintNV");
pub const CAPABILITY_MESH_SHADING_EXT: Capability = Capability::new("MeshShadingEXT");
pub const CAPABILITY_FRAGMENT_BARYCENTRIC_KHR: Capability =
    Capability::new("FragmentBarycentricKHR");
pub const CAPABILITY_COMPUTE_DERIVATIVE_GROUP_QUADS_KHR: Capability =
    Capability::new("ComputeDerivativeGroupQuadsKHR");
pub const CAPABILITY_FRAGMENT_DENSITY_EXT: Capability = Capability::new("FragmentDensityEXT");
pub const CAPABILITY_GROUP_NON_UNIFORM_PARTITIONED_EXT: Capability =
    Capability::new("GroupNonUniformPartitionedEXT");
pub const CAPABILITY_SHADER_NON_UNIFORM: Capability = Capability::new("ShaderNonUniform");
pub const CAPABILITY_RUNTIME_DESCRIPTOR_ARRAY: Capability =
    Capability::new("RuntimeDescriptorArray");
pub const CAPABILITY_INPUT_ATTACHMENT_ARRAY_DYNAMIC_INDEXING: Capability =
    Capability::new("InputAttachmentArrayDynamicIndexing");
pub const CAPABILITY_UNIFORM_TEXEL_BUFFER_ARRAY_DYNAMIC_INDEXING: Capability =
    Capability::new("UniformTexelBufferArrayDynamicIndexing");
pub const CAPABILITY_STORAGE_TEXEL_BUFFER_ARRAY_DYNAMIC_INDEXING: Capability =
    Capability::new("StorageTexelBufferArrayDynamicIndexing");
pub const CAPABILITY_UNIFORM_BUFFER_ARRAY_NON_UNIFORM_INDEXING: Capability =
    Capability::new("UniformBufferArrayNonUniformIndexing");
pub const CAPABILITY_SAMPLED_IMAGE_ARRAY_NON_UNIFORM_INDEXING: Capability =
    Capability::new("SampledImageArrayNonUniformIndexing");
pub const CAPABILITY_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING: Capability =
    Capability::new("StorageBufferArrayNonUniformIndexing");
pub const CAPABILITY_STORAGE_IMAGE_ARRAY_NON_UNIFORM_INDEXING: Capability =
    Capability::new("StorageImageArrayNonUniformIndexing");
pub const CAPABILITY_INPUT_ATTACHMENT_ARRAY_NON_UNIFORM_INDEXING: Capability =
    Capability::new("InputAttachmentArrayNonUniformIndexing");
pub const CAPABILITY_UNIFORM_TEXEL_BUFFER_ARRAY_NON_UNIFORM_INDEXING: Capability =
    Capability::new("UniformTexelBufferArrayNonUniformIndexing");
pub const CAPABILITY_STORAGE_TEXEL_BUFFER_ARRAY_NON_UNIFORM_INDEXING: Capability =
    Capability::new("StorageTexelBufferArrayNonUniformIndexing");
pub const CAPABILITY_RAY_TRACING_POSITION_FETCH_KHR: Capability =
    Capability::new("RayTracingPositionFetchKHR");
pub const CAPABILITY_RAY_TRACING_NV: Capability = Capability::new("RayTracingNV");
pub const CAPABILITY_RAY_TRACING_MOTION_BLUR_NV: Capability =
    Capability::new("RayTracingMotionBlurNV");
pub const CAPABILITY_VULKAN_MEMORY_MODEL: Capability = Capability::new("VulkanMemoryModel");
pub const CAPABILITY_VULKAN_MEMORY_MODEL_DEVICE_SCOPE: Capability =
    Capability::new("VulkanMemoryModelDeviceScope");
pub const CAPABILITY_PHYSICAL_STORAGE_BUFFER_ADDRESSES: Capability =
    Capability::new("PhysicalStorageBufferAddresses");
pub const CAPABILITY_COMPUTE_DERIVATIVE_GROUP_LINEAR_KHR: Capability =
    Capability::new("ComputeDerivativeGroupLinearKHR");
pub const CAPABILITY_RAY_TRACING_PROVISIONAL_KHR: Capability =
    Capability::new("RayTracingProvisionalKHR");
pub const CAPABILITY_COOPERATIVE_MATRIX_NV: Capability = Capability::new("CooperativeMatrixNV");
pub const CAPABILITY_FRAGMENT_SHADER_SAMPLE_INTERLOCK_EXT: Capability =
    Capability::new("FragmentShaderSampleInterlockEXT");
pub const CAPABILITY_FRAGMENT_SHADER_SHADING_RATE_INTERLOCK_EXT: Capability =
    Capability::new("FragmentShaderShadingRateInterlockEXT");
pub const CAPABILITY_SHADER_SM_BUILTINS_NV: Capability = Capability::new("ShaderSMBuiltinsNV");
pub const CAPABILITY_FRAGMENT_SHADER_PIXEL_INTERLOCK_EXT: Capability =
    Capability::new("FragmentShaderPixelInterlockEXT");
pub const CAPABILITY_DEMOTE_TO_HELPER_INVOCATION: Capability =
    Capability::new("DemoteToHelperInvocation");
pub const CAPABILITY_DISPLACEMENT_MICROMAP_NV: Capability =
    Capability::new("DisplacementMicromapNV");
pub const CAPABILITY_RAY_TRACING_OPACITY_MICROMAP_EXT: Capability =
    Capability::new("RayTracingOpacityMicromapEXT");
pub const CAPABILITY_SHADER_INVOCATION_REORDER_NV: Capability =
    Capability::new("ShaderInvocationReorderNV");
pub const CAPABILITY_SHADER_INVOCATION_REORDER_EXT: Capability =
    Capability::new("ShaderInvocationReorderEXT");
pub const CAPABILITY_BINDLESS_TEXTURE_NV: Capability = Capability::new("BindlessTextureNV");
pub const CAPABILITY_RAY_QUERY_POSITION_FETCH_KHR: Capability =
    Capability::new("RayQueryPositionFetchKHR");
pub const CAPABILITY_COOPERATIVE_VECTOR_NV: Capability = Capability::new("CooperativeVectorNV");
pub const CAPABILITY_ATOMIC_FLOAT_16_VECTOR_NV: Capability =
    Capability::new("AtomicFloat16VectorNV");
pub const CAPABILITY_RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Capability =
    Capability::new("RayTracingDisplacementMicromapNV");
pub const CAPABILITY_RAW_ACCESS_CHAINS_NV: Capability = Capability::new("RawAccessChainsNV");
pub const CAPABILITY_RAY_TRACING_SPHERES_GEOMETRY_NV: Capability =
    Capability::new("RayTracingSpheresGeometryNV");
pub const CAPABILITY_RAY_TRACING_LINEAR_SWEPT_SPHERES_GEOMETRY_NV: Capability =
    Capability::new("RayTracingLinearSweptSpheresGeometryNV");
pub const CAPABILITY_PUSH_CONSTANT_BANKS_NV: Capability = Capability::new("PushConstantBanksNV");
pub const CAPABILITY_LONG_VECTOR_EXT: Capability = Capability::new("LongVectorEXT");
pub const CAPABILITY_SHADER_64_BIT_INDEXING_EXT: Capability =
    Capability::new("Shader64BitIndexingEXT");
pub const CAPABILITY_COOPERATIVE_MATRIX_REDUCTIONS_NV: Capability =
    Capability::new("CooperativeMatrixReductionsNV");
pub const CAPABILITY_COOPERATIVE_MATRIX_CONVERSIONS_NV: Capability =
    Capability::new("CooperativeMatrixConversionsNV");
pub const CAPABILITY_COOPERATIVE_MATRIX_PER_ELEMENT_OPERATIONS_NV: Capability =
    Capability::new("CooperativeMatrixPerElementOperationsNV");
pub const CAPABILITY_COOPERATIVE_MATRIX_TENSOR_ADDRESSING_NV: Capability =
    Capability::new("CooperativeMatrixTensorAddressingNV");
pub const CAPABILITY_COOPERATIVE_MATRIX_BLOCK_LOADS_NV: Capability =
    Capability::new("CooperativeMatrixBlockLoadsNV");
pub const CAPABILITY_COOPERATIVE_VECTOR_TRAINING_NV: Capability =
    Capability::new("CooperativeVectorTrainingNV");
pub const CAPABILITY_RAY_TRACING_CLUSTER_ACCELERATION_STRUCTURE_NV: Capability =
    Capability::new("RayTracingClusterAccelerationStructureNV");
pub const CAPABILITY_TENSOR_ADDRESSING_NV: Capability = Capability::new("TensorAddressingNV");
pub const CAPABILITY_SUBGROUP_SHUFFLE_INTEL: Capability = Capability::new("SubgroupShuffleINTEL");
pub const CAPABILITY_SUBGROUP_BUFFER_BLOCK_IOINTEL: Capability =
    Capability::new("SubgroupBufferBlockIOINTEL");
pub const CAPABILITY_SUBGROUP_IMAGE_BLOCK_IOINTEL: Capability =
    Capability::new("SubgroupImageBlockIOINTEL");
pub const CAPABILITY_SUBGROUP_IMAGE_MEDIA_BLOCK_IOINTEL: Capability =
    Capability::new("SubgroupImageMediaBlockIOINTEL");
pub const CAPABILITY_ROUND_TO_INFINITY_INTEL: Capability = Capability::new("RoundToInfinityINTEL");
pub const CAPABILITY_FLOATING_POINT_MODE_INTEL: Capability =
    Capability::new("FloatingPointModeINTEL");
pub const CAPABILITY_INTEGER_FUNCTIONS_2_INTEL: Capability =
    Capability::new("IntegerFunctions2INTEL");
pub const CAPABILITY_FUNCTION_POINTERS_INTEL: Capability = Capability::new("FunctionPointersINTEL");
pub const CAPABILITY_INDIRECT_REFERENCES_INTEL: Capability =
    Capability::new("IndirectReferencesINTEL");
pub const CAPABILITY_ASM_INTEL: Capability = Capability::new("AsmINTEL");
pub const CAPABILITY_ATOMIC_FLOAT_32_MIN_MAX_EXT: Capability =
    Capability::new("AtomicFloat32MinMaxEXT");
pub const CAPABILITY_ATOMIC_FLOAT_64_MIN_MAX_EXT: Capability =
    Capability::new("AtomicFloat64MinMaxEXT");
pub const CAPABILITY_ATOMIC_FLOAT_16_MIN_MAX_EXT: Capability =
    Capability::new("AtomicFloat16MinMaxEXT");
pub const CAPABILITY_VECTOR_COMPUTE_INTEL: Capability = Capability::new("VectorComputeINTEL");
pub const CAPABILITY_VECTOR_ANY_INTEL: Capability = Capability::new("VectorAnyINTEL");
pub const CAPABILITY_EXPECT_ASSUME_KHR: Capability = Capability::new("ExpectAssumeKHR");
pub const CAPABILITY_SUBGROUP_AVC_MOTION_ESTIMATION_INTEL: Capability =
    Capability::new("SubgroupAvcMotionEstimationINTEL");
pub const CAPABILITY_SUBGROUP_AVC_MOTION_ESTIMATION_INTRA_INTEL: Capability =
    Capability::new("SubgroupAvcMotionEstimationIntraINTEL");
pub const CAPABILITY_SUBGROUP_AVC_MOTION_ESTIMATION_CHROMA_INTEL: Capability =
    Capability::new("SubgroupAvcMotionEstimationChromaINTEL");
pub const CAPABILITY_VARIABLE_LENGTH_ARRAY_INTEL: Capability =
    Capability::new("VariableLengthArrayINTEL");
pub const CAPABILITY_FUNCTION_FLOAT_CONTROL_INTEL: Capability =
    Capability::new("FunctionFloatControlINTEL");
pub const CAPABILITY_FPGA_MEMORY_ATTRIBUTES_ALTERA: Capability =
    Capability::new("FPGAMemoryAttributesALTERA");
pub const CAPABILITY_FP_FAST_MATH_MODE_INTEL: Capability = Capability::new("FPFastMathModeINTEL");
pub const CAPABILITY_ARBITRARY_PRECISION_INTEGERS_ALTERA: Capability =
    Capability::new("ArbitraryPrecisionIntegersALTERA");
pub const CAPABILITY_ARBITRARY_PRECISION_FLOATING_POINT_ALTERA: Capability =
    Capability::new("ArbitraryPrecisionFloatingPointALTERA");
pub const CAPABILITY_UNSTRUCTURED_LOOP_CONTROLS_INTEL: Capability =
    Capability::new("UnstructuredLoopControlsINTEL");
pub const CAPABILITY_FPGA_LOOP_CONTROLS_ALTERA: Capability =
    Capability::new("FPGALoopControlsALTERA");
pub const CAPABILITY_KERNEL_ATTRIBUTES_INTEL: Capability = Capability::new("KernelAttributesINTEL");
pub const CAPABILITY_FPGA_KERNEL_ATTRIBUTES_INTEL: Capability =
    Capability::new("FPGAKernelAttributesINTEL");
pub const CAPABILITY_FPGA_MEMORY_ACCESSES_ALTERA: Capability =
    Capability::new("FPGAMemoryAccessesALTERA");
pub const CAPABILITY_FPGA_CLUSTER_ATTRIBUTES_ALTERA: Capability =
    Capability::new("FPGAClusterAttributesALTERA");
pub const CAPABILITY_LOOP_FUSE_ALTERA: Capability = Capability::new("LoopFuseALTERA");
pub const CAPABILITY_FPGADSP_CONTROL_ALTERA: Capability = Capability::new("FPGADSPControlALTERA");
pub const CAPABILITY_MEMORY_ACCESS_ALIASING_INTEL: Capability =
    Capability::new("MemoryAccessAliasingINTEL");
pub const CAPABILITY_FPGA_INVOCATION_PIPELINING_ATTRIBUTES_ALTERA: Capability =
    Capability::new("FPGAInvocationPipeliningAttributesALTERA");
pub const CAPABILITY_FPGA_BUFFER_LOCATION_ALTERA: Capability =
    Capability::new("FPGABufferLocationALTERA");
pub const CAPABILITY_ARBITRARY_PRECISION_FIXED_POINT_ALTERA: Capability =
    Capability::new("ArbitraryPrecisionFixedPointALTERA");
pub const CAPABILITY_USM_STORAGE_CLASSES_ALTERA: Capability =
    Capability::new("USMStorageClassesALTERA");
pub const CAPABILITY_RUNTIME_ALIGNED_ATTRIBUTE_ALTERA: Capability =
    Capability::new("RuntimeAlignedAttributeALTERA");
pub const CAPABILITY_IO_PIPES_ALTERA: Capability = Capability::new("IOPipesALTERA");
pub const CAPABILITY_BLOCKING_PIPES_ALTERA: Capability = Capability::new("BlockingPipesALTERA");
pub const CAPABILITY_FPGA_REG_ALTERA: Capability = Capability::new("FPGARegALTERA");
pub const CAPABILITY_DOT_PRODUCT_INPUT_ALL: Capability = Capability::new("DotProductInputAll");
pub const CAPABILITY_DOT_PRODUCT_INPUT_4_X_8_BIT: Capability =
    Capability::new("DotProductInput4x8Bit");
pub const CAPABILITY_DOT_PRODUCT_INPUT_4_X_8_BIT_PACKED: Capability =
    Capability::new("DotProductInput4x8BitPacked");
pub const CAPABILITY_DOT_PRODUCT: Capability = Capability::new("DotProduct");
pub const CAPABILITY_RAY_CULL_MASK_KHR: Capability = Capability::new("RayCullMaskKHR");
pub const CAPABILITY_COOPERATIVE_MATRIX_KHR: Capability = Capability::new("CooperativeMatrixKHR");
pub const CAPABILITY_REPLICATED_COMPOSITES_EXT: Capability =
    Capability::new("ReplicatedCompositesEXT");
pub const CAPABILITY_BIT_INSTRUCTIONS: Capability = Capability::new("BitInstructions");
pub const CAPABILITY_GROUP_NON_UNIFORM_ROTATE_KHR: Capability =
    Capability::new("GroupNonUniformRotateKHR");
pub const CAPABILITY_FLOAT_CONTROLS_2: Capability = Capability::new("FloatControls2");
pub const CAPABILITY_FMAKHR: Capability = Capability::new("FMAKHR");
pub const CAPABILITY_ATOMIC_FLOAT_32_ADD_EXT: Capability = Capability::new("AtomicFloat32AddEXT");
pub const CAPABILITY_ATOMIC_FLOAT_64_ADD_EXT: Capability = Capability::new("AtomicFloat64AddEXT");
pub const CAPABILITY_LONG_COMPOSITES_INTEL: Capability = Capability::new("LongCompositesINTEL");
pub const CAPABILITY_OPT_NONE_EXT: Capability = Capability::new("OptNoneEXT");
pub const CAPABILITY_ATOMIC_FLOAT_16_ADD_EXT: Capability = Capability::new("AtomicFloat16AddEXT");
pub const CAPABILITY_DEBUG_INFO_MODULE_INTEL: Capability = Capability::new("DebugInfoModuleINTEL");
pub const CAPABILITY_B_FLOAT_16_CONVERSION_INTEL: Capability =
    Capability::new("BFloat16ConversionINTEL");
pub const CAPABILITY_SPLIT_BARRIER_INTEL: Capability = Capability::new("SplitBarrierINTEL");
pub const CAPABILITY_ARITHMETIC_FENCE_EXT: Capability = Capability::new("ArithmeticFenceEXT");
pub const CAPABILITY_FPGA_CLUSTER_ATTRIBUTES_V_2_ALTERA: Capability =
    Capability::new("FPGAClusterAttributesV2ALTERA");
pub const CAPABILITY_FPGA_KERNEL_ATTRIBUTESV_2_INTEL: Capability =
    Capability::new("FPGAKernelAttributesv2INTEL");
pub const CAPABILITY_TASK_SEQUENCE_ALTERA: Capability = Capability::new("TaskSequenceALTERA");
pub const CAPABILITY_FP_MAX_ERROR_INTEL: Capability = Capability::new("FPMaxErrorINTEL");
pub const CAPABILITY_FPGA_LATENCY_CONTROL_ALTERA: Capability =
    Capability::new("FPGALatencyControlALTERA");
pub const CAPABILITY_FPGA_ARGUMENT_INTERFACES_ALTERA: Capability =
    Capability::new("FPGAArgumentInterfacesALTERA");
pub const CAPABILITY_GLOBAL_VARIABLE_HOST_ACCESS_INTEL: Capability =
    Capability::new("GlobalVariableHostAccessINTEL");
pub const CAPABILITY_GLOBAL_VARIABLE_FPGA_DECORATIONS_ALTERA: Capability =
    Capability::new("GlobalVariableFPGADecorationsALTERA");
pub const CAPABILITY_SUBGROUP_BUFFER_PREFETCH_INTEL: Capability =
    Capability::new("SubgroupBufferPrefetchINTEL");
pub const CAPABILITY_SUBGROUP_2_D_BLOCK_IOINTEL: Capability =
    Capability::new("Subgroup2DBlockIOINTEL");
pub const CAPABILITY_SUBGROUP_2_D_BLOCK_TRANSFORM_INTEL: Capability =
    Capability::new("Subgroup2DBlockTransformINTEL");
pub const CAPABILITY_SUBGROUP_2_D_BLOCK_TRANSPOSE_INTEL: Capability =
    Capability::new("Subgroup2DBlockTransposeINTEL");
pub const CAPABILITY_SUBGROUP_MATRIX_MULTIPLY_ACCUMULATE_INTEL: Capability =
    Capability::new("SubgroupMatrixMultiplyAccumulateINTEL");
pub const CAPABILITY_TERNARY_BITWISE_FUNCTION_INTEL: Capability =
    Capability::new("TernaryBitwiseFunctionINTEL");
pub const CAPABILITY_UNTYPED_VARIABLE_LENGTH_ARRAY_INTEL: Capability =
    Capability::new("UntypedVariableLengthArrayINTEL");
pub const CAPABILITY_SPEC_CONDITIONAL_INTEL: Capability = Capability::new("SpecConditionalINTEL");
pub const CAPABILITY_FUNCTION_VARIANTS_INTEL: Capability = Capability::new("FunctionVariantsINTEL");
pub const CAPABILITY_GROUP_UNIFORM_ARITHMETIC_KHR: Capability =
    Capability::new("GroupUniformArithmeticKHR");
pub const CAPABILITY_TENSOR_FLOAT_32_ROUNDING_INTEL: Capability =
    Capability::new("TensorFloat32RoundingINTEL");
pub const CAPABILITY_MASKED_GATHER_SCATTER_INTEL: Capability =
    Capability::new("MaskedGatherScatterINTEL");
pub const CAPABILITY_CACHE_CONTROLS_INTEL: Capability = Capability::new("CacheControlsINTEL");
pub const CAPABILITY_REGISTER_LIMITS_INTEL: Capability = Capability::new("RegisterLimitsINTEL");
pub const CAPABILITY_BINDLESS_IMAGES_INTEL: Capability = Capability::new("BindlessImagesINTEL");
