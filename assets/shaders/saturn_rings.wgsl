@group(0) @binding(0) var<uniform> color: vec4<f32>;
@group(0) @binding(1) var<uniform> ring_params: vec4<f32>; // (inner_radius, outer_radius, fade, gap_factor)

@fragment
fn fragment(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_coord.xy / vec2<f32>(10.0, 10.0) * 2.0 - 1.0;
    let dist = length(uv);

    let inner = ring_params.x;
    let outer = ring_params.y;
    let fade = ring_params.z;
    let gap_factor = ring_params.w;

    // Generate ring mask
    let ring_mask = smoothstep(inner - fade, inner, dist) * (1.0 - smoothstep(outer, outer + fade, dist));

    // Add gaps in the rings
    let angle = atan2(uv.y, uv.x);
    let gap = abs(sin(angle * gap_factor)) * 0.5;
    let final_mask = ring_mask * (1.0 - gap);

    return vec4(1.0, 1.0, 1.0, 1.0);
}
