use super::preamble::*;
pub const EXTENSION_SPV_ALTERA_ARBITRARY_PRECISION_FIXED_POINT: Extension =
    Extension::new("SPV_ALTERA_arbitrary_precision_fixed_point");
pub const EXTENSION_SPV_ALTERA_ARBITRARY_PRECISION_FLOATING_POINT: Extension =
    Extension::new("SPV_ALTERA_arbitrary_precision_floating_point");
pub const EXTENSION_SPV_ALTERA_ARBITRARY_PRECISION_INTEGERS: Extension =
    Extension::new("SPV_ALTERA_arbitrary_precision_integers");
pub const EXTENSION_SPV_ALTERA_BLOCKING_PIPES: Extension =
    Extension::new("SPV_ALTERA_blocking_pipes");
pub const EXTENSION_SPV_ALTERA_FPGA_ARGUMENT_INTERFACES: Extension =
    Extension::new("SPV_ALTERA_fpga_argument_interfaces");
pub const EXTENSION_SPV_ALTERA_FPGA_BUFFER_LOCATION: Extension =
    Extension::new("SPV_ALTERA_fpga_buffer_location");
pub const EXTENSION_SPV_ALTERA_FPGA_CLUSTER_ATTRIBUTES: Extension =
    Extension::new("SPV_ALTERA_fpga_cluster_attributes");
pub const EXTENSION_SPV_ALTERA_FPGA_DSP_CONTROL: Extension =
    Extension::new("SPV_ALTERA_fpga_dsp_control");
pub const EXTENSION_SPV_ALTERA_FPGA_INVOCATION_PIPELINING_ATTRIBUTES: Extension =
    Extension::new("SPV_ALTERA_fpga_invocation_pipelining_attributes");
pub const EXTENSION_SPV_ALTERA_FPGA_LATENCY_CONTROL: Extension =
    Extension::new("SPV_ALTERA_fpga_latency_control");
pub const EXTENSION_SPV_ALTERA_FPGA_LOOP_CONTROLS: Extension =
    Extension::new("SPV_ALTERA_fpga_loop_controls");
pub const EXTENSION_SPV_ALTERA_FPGA_MEMORY_ACCESSES: Extension =
    Extension::new("SPV_ALTERA_fpga_memory_accesses");
pub const EXTENSION_SPV_ALTERA_FPGA_MEMORY_ATTRIBUTES: Extension =
    Extension::new("SPV_ALTERA_fpga_memory_attributes");
pub const EXTENSION_SPV_ALTERA_FPGA_REG: Extension = Extension::new("SPV_ALTERA_fpga_reg");
pub const EXTENSION_SPV_ALTERA_GLOBAL_VARIABLE_FPGA_DECORATIONS: Extension =
    Extension::new("SPV_ALTERA_global_variable_fpga_decorations");
pub const EXTENSION_SPV_ALTERA_IO_PIPES: Extension = Extension::new("SPV_ALTERA_io_pipes");
pub const EXTENSION_SPV_ALTERA_LOOP_FUSE: Extension = Extension::new("SPV_ALTERA_loop_fuse");
pub const EXTENSION_SPV_ALTERA_RUNTIME_ALIGNED: Extension =
    Extension::new("SPV_ALTERA_runtime_aligned");
pub const EXTENSION_SPV_ALTERA_TASK_SEQUENCE: Extension =
    Extension::new("SPV_ALTERA_task_sequence");
pub const EXTENSION_SPV_ALTERA_USM_STORAGE_CLASSES: Extension =
    Extension::new("SPV_ALTERA_usm_storage_classes");
pub const EXTENSION_SPV_AMDX_SHADER_ENQUEUE: Extension = Extension::new("SPV_AMDX_shader_enqueue");
pub const EXTENSION_SPV_AMD_GPU_SHADER_HALF_FLOAT_FETCH: Extension =
    Extension::new("SPV_AMD_gpu_shader_half_float_fetch");
pub const EXTENSION_SPV_AMD_SHADER_BALLOT: Extension = Extension::new("SPV_AMD_shader_ballot");
pub const EXTENSION_SPV_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS: Extension =
    Extension::new("SPV_AMD_shader_early_and_late_fragment_tests");
pub const EXTENSION_SPV_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER: Extension =
    Extension::new("SPV_AMD_shader_explicit_vertex_parameter");
pub const EXTENSION_SPV_AMD_SHADER_FRAGMENT_MASK: Extension =
    Extension::new("SPV_AMD_shader_fragment_mask");
pub const EXTENSION_SPV_AMD_SHADER_IMAGE_LOAD_STORE_LOD: Extension =
    Extension::new("SPV_AMD_shader_image_load_store_lod");
pub const EXTENSION_SPV_AMD_TEXTURE_GATHER_BIAS_LOD: Extension =
    Extension::new("SPV_AMD_texture_gather_bias_lod");
pub const EXTENSION_SPV_AMD_WEAK_LINKAGE: Extension = Extension::new("SPV_AMD_weak_linkage");
pub const EXTENSION_SPV_ARM_COOPERATIVE_MATRIX_LAYOUTS: Extension =
    Extension::new("SPV_ARM_cooperative_matrix_layouts");
pub const EXTENSION_SPV_ARM_CORE_BUILTINS: Extension = Extension::new("SPV_ARM_core_builtins");
pub const EXTENSION_SPV_ARM_GRAPH: Extension = Extension::new("SPV_ARM_graph");
pub const EXTENSION_SPV_ARM_TENSORS: Extension = Extension::new("SPV_ARM_tensors");
pub const EXTENSION_SPV_EXT_ARITHMETIC_FENCE: Extension =
    Extension::new("SPV_EXT_arithmetic_fence");
pub const EXTENSION_SPV_EXT_DEMOTE_TO_HELPER_INVOCATION: Extension =
    Extension::new("SPV_EXT_demote_to_helper_invocation");
pub const EXTENSION_SPV_EXT_DESCRIPTOR_HEAP: Extension = Extension::new("SPV_EXT_descriptor_heap");
pub const EXTENSION_SPV_EXT_DESCRIPTOR_INDEXING: Extension =
    Extension::new("SPV_EXT_descriptor_indexing");
pub const EXTENSION_SPV_EXT_FLOAT_8: Extension = Extension::new("SPV_EXT_float8");
pub const EXTENSION_SPV_EXT_FRAGMENT_FULLY_COVERED: Extension =
    Extension::new("SPV_EXT_fragment_fully_covered");
pub const EXTENSION_SPV_EXT_FRAGMENT_INVOCATION_DENSITY: Extension =
    Extension::new("SPV_EXT_fragment_invocation_density");
pub const EXTENSION_SPV_EXT_FRAGMENT_SHADER_INTERLOCK: Extension =
    Extension::new("SPV_EXT_fragment_shader_interlock");
pub const EXTENSION_SPV_EXT_LONG_VECTOR: Extension = Extension::new("SPV_EXT_long_vector");
pub const EXTENSION_SPV_EXT_MESH_SHADER: Extension = Extension::new("SPV_EXT_mesh_shader");
pub const EXTENSION_SPV_EXT_OCP_MICROSCALING_TYPES: Extension =
    Extension::new("SPV_EXT_ocp_microscaling_types");
pub const EXTENSION_SPV_EXT_OPACITY_MICROMAP: Extension =
    Extension::new("SPV_EXT_opacity_micromap");
pub const EXTENSION_SPV_EXT_OPTNONE: Extension = Extension::new("SPV_EXT_optnone");
pub const EXTENSION_SPV_EXT_PHYSICAL_STORAGE_BUFFER: Extension =
    Extension::new("SPV_EXT_physical_storage_buffer");
pub const EXTENSION_SPV_EXT_REPLICATED_COMPOSITES: Extension =
    Extension::new("SPV_EXT_replicated_composites");
pub const EXTENSION_SPV_EXT_SHADER_64_BIT_INDEXING: Extension =
    Extension::new("SPV_EXT_shader_64bit_indexing");
pub const EXTENSION_SPV_EXT_SHADER_ATOMIC_FLOAT_16_ADD: Extension =
    Extension::new("SPV_EXT_shader_atomic_float16_add");
pub const EXTENSION_SPV_EXT_SHADER_ATOMIC_FLOAT_ADD: Extension =
    Extension::new("SPV_EXT_shader_atomic_float_add");
pub const EXTENSION_SPV_EXT_SHADER_ATOMIC_FLOAT_MIN_MAX: Extension =
    Extension::new("SPV_EXT_shader_atomic_float_min_max");
pub const EXTENSION_SPV_EXT_SHADER_IMAGE_INT_64: Extension =
    Extension::new("SPV_EXT_shader_image_int64");
pub const EXTENSION_SPV_EXT_SHADER_INVOCATION_REORDER: Extension =
    Extension::new("SPV_EXT_shader_invocation_reorder");
pub const EXTENSION_SPV_EXT_SHADER_STENCIL_EXPORT: Extension =
    Extension::new("SPV_EXT_shader_stencil_export");
pub const EXTENSION_SPV_EXT_SHADER_SUBGROUP_PARTITIONED: Extension =
    Extension::new("SPV_EXT_shader_subgroup_partitioned");
pub const EXTENSION_SPV_EXT_SHADER_TILE_IMAGE: Extension =
    Extension::new("SPV_EXT_shader_tile_image");
pub const EXTENSION_SPV_EXT_SHADER_VIEWPORT_INDEX_LAYER: Extension =
    Extension::new("SPV_EXT_shader_viewport_index_layer");
pub const EXTENSION_SPV_EXT_SPLIT_BARRIER: Extension = Extension::new("SPV_EXT_split_barrier");
pub const EXTENSION_SPV_GOOGLE_DECORATE_STRING: Extension =
    Extension::new("SPV_GOOGLE_decorate_string");
pub const EXTENSION_SPV_GOOGLE_HLSL_FUNCTIONALITY_1: Extension =
    Extension::new("SPV_GOOGLE_hlsl_functionality1");
pub const EXTENSION_SPV_GOOGLE_USER_TYPE: Extension = Extension::new("SPV_GOOGLE_user_type");
pub const EXTENSION_SPV_INTEL_2_D_BLOCK_IO: Extension = Extension::new("SPV_INTEL_2d_block_io");
pub const EXTENSION_SPV_INTEL_ARBITRARY_PRECISION_FIXED_POINT: Extension =
    Extension::new("SPV_INTEL_arbitrary_precision_fixed_point");
pub const EXTENSION_SPV_INTEL_ARBITRARY_PRECISION_FLOATING_POINT: Extension =
    Extension::new("SPV_INTEL_arbitrary_precision_floating_point");
pub const EXTENSION_SPV_INTEL_ARBITRARY_PRECISION_INTEGERS: Extension =
    Extension::new("SPV_INTEL_arbitrary_precision_integers");
pub const EXTENSION_SPV_INTEL_BFLOAT_16_CONVERSION: Extension =
    Extension::new("SPV_INTEL_bfloat16_conversion");
pub const EXTENSION_SPV_INTEL_BINDLESS_IMAGES: Extension =
    Extension::new("SPV_INTEL_bindless_images");
pub const EXTENSION_SPV_INTEL_BLOCKING_PIPES: Extension =
    Extension::new("SPV_INTEL_blocking_pipes");
pub const EXTENSION_SPV_INTEL_CACHE_CONTROLS: Extension =
    Extension::new("SPV_INTEL_cache_controls");
pub const EXTENSION_SPV_INTEL_DEBUG_MODULE: Extension = Extension::new("SPV_INTEL_debug_module");
pub const EXTENSION_SPV_INTEL_DEVICE_SIDE_AVC_MOTION_ESTIMATION: Extension =
    Extension::new("SPV_INTEL_device_side_avc_motion_estimation");
pub const EXTENSION_SPV_INTEL_FLOAT_CONTROLS_2: Extension =
    Extension::new("SPV_INTEL_float_controls2");
pub const EXTENSION_SPV_INTEL_FP_FAST_MATH_MODE: Extension =
    Extension::new("SPV_INTEL_fp_fast_math_mode");
pub const EXTENSION_SPV_INTEL_FP_MAX_ERROR: Extension = Extension::new("SPV_INTEL_fp_max_error");
pub const EXTENSION_SPV_INTEL_FPGA_ARGUMENT_INTERFACES: Extension =
    Extension::new("SPV_INTEL_fpga_argument_interfaces");
pub const EXTENSION_SPV_INTEL_FPGA_BUFFER_LOCATION: Extension =
    Extension::new("SPV_INTEL_fpga_buffer_location");
pub const EXTENSION_SPV_INTEL_FPGA_CLUSTER_ATTRIBUTES: Extension =
    Extension::new("SPV_INTEL_fpga_cluster_attributes");
pub const EXTENSION_SPV_INTEL_FPGA_DSP_CONTROL: Extension =
    Extension::new("SPV_INTEL_fpga_dsp_control");
pub const EXTENSION_SPV_INTEL_FPGA_INVOCATION_PIPELINING_ATTRIBUTES: Extension =
    Extension::new("SPV_INTEL_fpga_invocation_pipelining_attributes");
pub const EXTENSION_SPV_INTEL_FPGA_LATENCY_CONTROL: Extension =
    Extension::new("SPV_INTEL_fpga_latency_control");
pub const EXTENSION_SPV_INTEL_FPGA_LOOP_CONTROLS: Extension =
    Extension::new("SPV_INTEL_fpga_loop_controls");
pub const EXTENSION_SPV_INTEL_FPGA_MEMORY_ACCESSES: Extension =
    Extension::new("SPV_INTEL_fpga_memory_accesses");
pub const EXTENSION_SPV_INTEL_FPGA_MEMORY_ATTRIBUTES: Extension =
    Extension::new("SPV_INTEL_fpga_memory_attributes");
pub const EXTENSION_SPV_INTEL_FPGA_REG: Extension = Extension::new("SPV_INTEL_fpga_reg");
pub const EXTENSION_SPV_INTEL_FUNCTION_POINTERS: Extension =
    Extension::new("SPV_INTEL_function_pointers");
pub const EXTENSION_SPV_INTEL_FUNCTION_VARIANTS: Extension =
    Extension::new("SPV_INTEL_function_variants");
pub const EXTENSION_SPV_INTEL_GLOBAL_VARIABLE_FPGA_DECORATIONS: Extension =
    Extension::new("SPV_INTEL_global_variable_fpga_decorations");
pub const EXTENSION_SPV_INTEL_GLOBAL_VARIABLE_HOST_ACCESS: Extension =
    Extension::new("SPV_INTEL_global_variable_host_access");
pub const EXTENSION_SPV_INTEL_INLINE_ASSEMBLY: Extension =
    Extension::new("SPV_INTEL_inline_assembly");
pub const EXTENSION_SPV_INTEL_INT_4: Extension = Extension::new("SPV_INTEL_int4");
pub const EXTENSION_SPV_INTEL_IO_PIPES: Extension = Extension::new("SPV_INTEL_io_pipes");
pub const EXTENSION_SPV_INTEL_KERNEL_ATTRIBUTES: Extension =
    Extension::new("SPV_INTEL_kernel_attributes");
pub const EXTENSION_SPV_INTEL_LONG_COMPOSITES: Extension =
    Extension::new("SPV_INTEL_long_composites");
pub const EXTENSION_SPV_INTEL_LOOP_FUSE: Extension = Extension::new("SPV_INTEL_loop_fuse");
pub const EXTENSION_SPV_INTEL_MASKED_GATHER_SCATTER: Extension =
    Extension::new("SPV_INTEL_masked_gather_scatter");
pub const EXTENSION_SPV_INTEL_MAXIMUM_REGISTERS: Extension =
    Extension::new("SPV_INTEL_maximum_registers");
pub const EXTENSION_SPV_INTEL_MEDIA_BLOCK_IO: Extension =
    Extension::new("SPV_INTEL_media_block_io");
pub const EXTENSION_SPV_INTEL_MEMORY_ACCESS_ALIASING: Extension =
    Extension::new("SPV_INTEL_memory_access_aliasing");
pub const EXTENSION_SPV_INTEL_OPTNONE: Extension = Extension::new("SPV_INTEL_optnone");
pub const EXTENSION_SPV_INTEL_PREDICATED_IO: Extension = Extension::new("SPV_INTEL_predicated_io");
pub const EXTENSION_SPV_INTEL_ROUNDED_DIVIDE_SQRT: Extension =
    Extension::new("SPV_INTEL_rounded_divide_sqrt");
pub const EXTENSION_SPV_INTEL_RUNTIME_ALIGNED: Extension =
    Extension::new("SPV_INTEL_runtime_aligned");
pub const EXTENSION_SPV_INTEL_SHADER_INTEGER_FUNCTIONS_2: Extension =
    Extension::new("SPV_INTEL_shader_integer_functions2");
pub const EXTENSION_SPV_INTEL_SPLIT_BARRIER: Extension = Extension::new("SPV_INTEL_split_barrier");
pub const EXTENSION_SPV_INTEL_SUBGROUP_BUFFER_PREFETCH: Extension =
    Extension::new("SPV_INTEL_subgroup_buffer_prefetch");
pub const EXTENSION_SPV_INTEL_SUBGROUP_MATRIX_MULTIPLY_ACCUMULATE: Extension =
    Extension::new("SPV_INTEL_subgroup_matrix_multiply_accumulate");
pub const EXTENSION_SPV_INTEL_SUBGROUPS: Extension = Extension::new("SPV_INTEL_subgroups");
pub const EXTENSION_SPV_INTEL_TASK_SEQUENCE: Extension = Extension::new("SPV_INTEL_task_sequence");
pub const EXTENSION_SPV_INTEL_TENSOR_FLOAT_32_CONVERSION: Extension =
    Extension::new("SPV_INTEL_tensor_float32_conversion");
pub const EXTENSION_SPV_INTEL_TERNARY_BITWISE_FUNCTION: Extension =
    Extension::new("SPV_INTEL_ternary_bitwise_function");
pub const EXTENSION_SPV_INTEL_UNSTRUCTURED_LOOP_CONTROLS: Extension =
    Extension::new("SPV_INTEL_unstructured_loop_controls");
pub const EXTENSION_SPV_INTEL_USM_STORAGE_CLASSES: Extension =
    Extension::new("SPV_INTEL_usm_storage_classes");
pub const EXTENSION_SPV_INTEL_VARIABLE_LENGTH_ARRAY: Extension =
    Extension::new("SPV_INTEL_variable_length_array");
pub const EXTENSION_SPV_INTEL_VECTOR_COMPUTE: Extension =
    Extension::new("SPV_INTEL_vector_compute");
pub const EXTENSION_SPV_KHR_16_BIT_STORAGE: Extension = Extension::new("SPV_KHR_16bit_storage");
pub const EXTENSION_SPV_KHR_8_BIT_STORAGE: Extension = Extension::new("SPV_KHR_8bit_storage");
pub const EXTENSION_SPV_KHR_ABORT: Extension = Extension::new("SPV_KHR_abort");
pub const EXTENSION_SPV_KHR_BFLOAT_16: Extension = Extension::new("SPV_KHR_bfloat16");
pub const EXTENSION_SPV_KHR_BIT_INSTRUCTIONS: Extension =
    Extension::new("SPV_KHR_bit_instructions");
pub const EXTENSION_SPV_KHR_COMPUTE_SHADER_DERIVATIVES: Extension =
    Extension::new("SPV_KHR_compute_shader_derivatives");
pub const EXTENSION_SPV_KHR_CONSTANT_DATA: Extension = Extension::new("SPV_KHR_constant_data");
pub const EXTENSION_SPV_KHR_COOPERATIVE_MATRIX: Extension =
    Extension::new("SPV_KHR_cooperative_matrix");
pub const EXTENSION_SPV_KHR_DEVICE_GROUP: Extension = Extension::new("SPV_KHR_device_group");
pub const EXTENSION_SPV_KHR_EXPECT_ASSUME: Extension = Extension::new("SPV_KHR_expect_assume");
pub const EXTENSION_SPV_KHR_FLOAT_CONTROLS: Extension = Extension::new("SPV_KHR_float_controls");
pub const EXTENSION_SPV_KHR_FLOAT_CONTROLS_2: Extension = Extension::new("SPV_KHR_float_controls2");
pub const EXTENSION_SPV_KHR_FMA: Extension = Extension::new("SPV_KHR_fma");
pub const EXTENSION_SPV_KHR_FRAGMENT_SHADER_BARYCENTRIC: Extension =
    Extension::new("SPV_KHR_fragment_shader_barycentric");
pub const EXTENSION_SPV_KHR_FRAGMENT_SHADING_RATE: Extension =
    Extension::new("SPV_KHR_fragment_shading_rate");
pub const EXTENSION_SPV_KHR_INTEGER_DOT_PRODUCT: Extension =
    Extension::new("SPV_KHR_integer_dot_product");
pub const EXTENSION_SPV_KHR_LINKONCE_ODR: Extension = Extension::new("SPV_KHR_linkonce_odr");
pub const EXTENSION_SPV_KHR_MAXIMAL_RECONVERGENCE: Extension =
    Extension::new("SPV_KHR_maximal_reconvergence");
pub const EXTENSION_SPV_KHR_MULTIVIEW: Extension = Extension::new("SPV_KHR_multiview");
pub const EXTENSION_SPV_KHR_NO_INTEGER_WRAP_DECORATION: Extension =
    Extension::new("SPV_KHR_no_integer_wrap_decoration");
pub const EXTENSION_SPV_KHR_OPACITY_MICROMAP: Extension =
    Extension::new("SPV_KHR_opacity_micromap");
pub const EXTENSION_SPV_KHR_PHYSICAL_STORAGE_BUFFER: Extension =
    Extension::new("SPV_KHR_physical_storage_buffer");
pub const EXTENSION_SPV_KHR_POISON_FREEZE: Extension = Extension::new("SPV_KHR_poison_freeze");
pub const EXTENSION_SPV_KHR_POST_DEPTH_COVERAGE: Extension =
    Extension::new("SPV_KHR_post_depth_coverage");
pub const EXTENSION_SPV_KHR_QUAD_CONTROL: Extension = Extension::new("SPV_KHR_quad_control");
pub const EXTENSION_SPV_KHR_RAY_CULL_MASK: Extension = Extension::new("SPV_KHR_ray_cull_mask");
pub const EXTENSION_SPV_KHR_RAY_QUERY: Extension = Extension::new("SPV_KHR_ray_query");
pub const EXTENSION_SPV_KHR_RAY_TRACING: Extension = Extension::new("SPV_KHR_ray_tracing");
pub const EXTENSION_SPV_KHR_RAY_TRACING_POSITION_FETCH: Extension =
    Extension::new("SPV_KHR_ray_tracing_position_fetch");
pub const EXTENSION_SPV_KHR_RELAXED_EXTENDED_INSTRUCTION: Extension =
    Extension::new("SPV_KHR_relaxed_extended_instruction");
pub const EXTENSION_SPV_KHR_SHADER_ATOMIC_COUNTER_OPS: Extension =
    Extension::new("SPV_KHR_shader_atomic_counter_ops");
pub const EXTENSION_SPV_KHR_SHADER_BALLOT: Extension = Extension::new("SPV_KHR_shader_ballot");
pub const EXTENSION_SPV_KHR_SHADER_CLOCK: Extension = Extension::new("SPV_KHR_shader_clock");
pub const EXTENSION_SPV_KHR_SHADER_DRAW_PARAMETERS: Extension =
    Extension::new("SPV_KHR_shader_draw_parameters");
pub const EXTENSION_SPV_KHR_STORAGE_BUFFER_STORAGE_CLASS: Extension =
    Extension::new("SPV_KHR_storage_buffer_storage_class");
pub const EXTENSION_SPV_KHR_SUBGROUP_ROTATE: Extension = Extension::new("SPV_KHR_subgroup_rotate");
pub const EXTENSION_SPV_KHR_SUBGROUP_UNIFORM_CONTROL_FLOW: Extension =
    Extension::new("SPV_KHR_subgroup_uniform_control_flow");
pub const EXTENSION_SPV_KHR_SUBGROUP_VOTE: Extension = Extension::new("SPV_KHR_subgroup_vote");
pub const EXTENSION_SPV_KHR_TERMINATE_INVOCATION: Extension =
    Extension::new("SPV_KHR_terminate_invocation");
pub const EXTENSION_SPV_KHR_UNIFORM_GROUP_INSTRUCTIONS: Extension =
    Extension::new("SPV_KHR_uniform_group_instructions");
pub const EXTENSION_SPV_KHR_UNTYPED_POINTERS: Extension =
    Extension::new("SPV_KHR_untyped_pointers");
pub const EXTENSION_SPV_KHR_VARIABLE_POINTERS: Extension =
    Extension::new("SPV_KHR_variable_pointers");
pub const EXTENSION_SPV_KHR_VULKAN_MEMORY_MODEL: Extension =
    Extension::new("SPV_KHR_vulkan_memory_model");
pub const EXTENSION_SPV_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT: Extension =
    Extension::new("SPV_KHR_workgroup_memory_explicit_layout");
pub const EXTENSION_SPV_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES: Extension =
    Extension::new("SPV_NVX_multiview_per_view_attributes");
pub const EXTENSION_SPV_NV_BINDLESS_TEXTURE: Extension = Extension::new("SPV_NV_bindless_texture");
pub const EXTENSION_SPV_NV_CLUSTER_ACCELERATION_STRUCTURE: Extension =
    Extension::new("SPV_NV_cluster_acceleration_structure");
pub const EXTENSION_SPV_NV_COMPUTE_SHADER_DERIVATIVES: Extension =
    Extension::new("SPV_NV_compute_shader_derivatives");
pub const EXTENSION_SPV_NV_COOPERATIVE_MATRIX: Extension =
    Extension::new("SPV_NV_cooperative_matrix");
pub const EXTENSION_SPV_NV_COOPERATIVE_MATRIX_2: Extension =
    Extension::new("SPV_NV_cooperative_matrix2");
pub const EXTENSION_SPV_NV_COOPERATIVE_MATRIX_DECODE_VECTOR: Extension =
    Extension::new("SPV_NV_cooperative_matrix_decode_vector");
pub const EXTENSION_SPV_NV_COOPERATIVE_VECTOR: Extension =
    Extension::new("SPV_NV_cooperative_vector");
pub const EXTENSION_SPV_NV_DISPLACEMENT_MICROMAP: Extension =
    Extension::new("SPV_NV_displacement_micromap");
pub const EXTENSION_SPV_NV_FRAGMENT_SHADER_BARYCENTRIC: Extension =
    Extension::new("SPV_NV_fragment_shader_barycentric");
pub const EXTENSION_SPV_NV_GEOMETRY_SHADER_PASSTHROUGH: Extension =
    Extension::new("SPV_NV_geometry_shader_passthrough");
pub const EXTENSION_SPV_NV_LINEAR_SWEPT_SPHERES: Extension =
    Extension::new("SPV_NV_linear_swept_spheres");
pub const EXTENSION_SPV_NV_MESH_SHADER: Extension = Extension::new("SPV_NV_mesh_shader");
pub const EXTENSION_SPV_NV_PUSH_CONSTANT_BANK: Extension =
    Extension::new("SPV_NV_push_constant_bank");
pub const EXTENSION_SPV_NV_RAW_ACCESS_CHAINS: Extension =
    Extension::new("SPV_NV_raw_access_chains");
pub const EXTENSION_SPV_NV_RAY_TRACING: Extension = Extension::new("SPV_NV_ray_tracing");
pub const EXTENSION_SPV_NV_RAY_TRACING_MOTION_BLUR: Extension =
    Extension::new("SPV_NV_ray_tracing_motion_blur");
pub const EXTENSION_SPV_NV_SAMPLE_MASK_OVERRIDE_COVERAGE: Extension =
    Extension::new("SPV_NV_sample_mask_override_coverage");
pub const EXTENSION_SPV_NV_SHADER_ATOMIC_FP_16_VECTOR: Extension =
    Extension::new("SPV_NV_shader_atomic_fp16_vector");
pub const EXTENSION_SPV_NV_SHADER_IMAGE_FOOTPRINT: Extension =
    Extension::new("SPV_NV_shader_image_footprint");
pub const EXTENSION_SPV_NV_SHADER_INVOCATION_REORDER: Extension =
    Extension::new("SPV_NV_shader_invocation_reorder");
pub const EXTENSION_SPV_NV_SHADER_SM_BUILTINS: Extension =
    Extension::new("SPV_NV_shader_sm_builtins");
pub const EXTENSION_SPV_NV_SHADER_SUBGROUP_PARTITIONED: Extension =
    Extension::new("SPV_NV_shader_subgroup_partitioned");
pub const EXTENSION_SPV_NV_SHADING_RATE: Extension = Extension::new("SPV_NV_shading_rate");
pub const EXTENSION_SPV_NV_STEREO_VIEW_RENDERING: Extension =
    Extension::new("SPV_NV_stereo_view_rendering");
pub const EXTENSION_SPV_NV_TENSOR_ADDRESSING: Extension =
    Extension::new("SPV_NV_tensor_addressing");
pub const EXTENSION_SPV_NV_VIEWPORT_ARRAY_2: Extension = Extension::new("SPV_NV_viewport_array2");
pub const EXTENSION_SPV_QCOM_COOPERATIVE_MATRIX_CONVERSION: Extension =
    Extension::new("SPV_QCOM_cooperative_matrix_conversion");
pub const EXTENSION_SPV_QCOM_IMAGE_PROCESSING: Extension =
    Extension::new("SPV_QCOM_image_processing");
pub const EXTENSION_SPV_QCOM_IMAGE_PROCESSING_2: Extension =
    Extension::new("SPV_QCOM_image_processing2");
pub const EXTENSION_SPV_QCOM_IMAGE_PROCESSING_3: Extension =
    Extension::new("SPV_QCOM_image_processing3");
pub const EXTENSION_SPV_QCOM_MULTIPLE_WAIT_QUEUES: Extension =
    Extension::new("SPV_QCOM_multiple_wait_queues");
pub const EXTENSION_SPV_QCOM_TILE_SHADING: Extension = Extension::new("SPV_QCOM_tile_shading");
pub const EXTENSION_SPV_SAMSUNG_INTRINSIC: Extension = Extension::new("SPV_SAMSUNG_intrinsic");
pub const EXTENSION_SPV_VALVE_MIXED_FLOAT_DOT_PRODUCT: Extension =
    Extension::new("SPV_VALVE_mixed_float_dot_product");
