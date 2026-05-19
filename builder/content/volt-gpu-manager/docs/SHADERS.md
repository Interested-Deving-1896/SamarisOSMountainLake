# Shaders

## Builtin Shaders

The Volt GPU Manager ships with 12 builtin shaders registered via
`BuiltinShaderRegistry`:

| Name | Type | Critical |
|------|------|----------|
| `blur_gaussian` | Compute/Fragment | No |
| `shadow_box` | Fragment | No |
| `gradient_linear` | Fragment | No |
| `transform_2d` | Vertex | Yes |
| `composite_alpha` | Fragment | Yes |
| `border_radius_vert` | Vertex | Yes |
| `border_radius_frag` | Fragment | Yes |
| `text_glyph_vert` | Vertex | Yes |
| `text_glyph_frag` | Fragment | Yes |
| `ai_matmul` | Compute | No |
| `vram_compress` | Compute | No |
| `vram_decompress` | Compute | No |

Critical shaders (6 total) are warmed up at initialization.

## Shader Cache

The `ShaderCache` provides:

- **Insert**: Store compiled shader bytecode by `ShaderId`
- **Get**: Lookup with hit/miss tracking
- **Capacity limit**: Bounded by configurable megabyte limit
- **Eviction**: When cache exceeds capacity, all entries are cleared (simple policy)

```rust
let cache = ShaderCache::new(128); // 128 MB max
cache.insert(id, spirv_data)?;
let data = cache.get(&id); // Option<Vec<u8>>
```

## Pipeline Cache

The `PipelineCache` handles graphics/compute pipeline objects:
- Built on top of shader cache entries
- Maps shader combinations to pipeline objects
- Supports cache warming from builtin shaders

## Warmup Process

1. At initialization, `Warmup::warmup_critical()` is called
2. Iterates through `BuiltinShaderRegistry::critical_shaders()`
3. Inserts each critical shader into the `ShaderCache`
4. Pre-compiles pipelines for critical shader combinations

## JIT Compilation

The `ShaderCompiler` module handles on-the-fly shader compilation:
- Compiles GLSL/WGSL source to SPIR-V
- Returns `ShaderId` on success
- Returns `VgmError::ShaderCompileFailed` on compilation error
- Compilation is cached — repeated compiles for the same source return cached result

## Backend Support

- **Wgpu backend**: Full support via `naga` compiler
- **Vulkan backend**: Raw SPIR-V support
- **Metal backend**: MSL conversion via naga
- **Null backend**: Stub — all operations succeed with empty data
