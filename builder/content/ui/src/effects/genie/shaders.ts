export const vertexShaderSource = `
precision highp float;

attribute vec2 a_pos;
attribute vec2 a_uv;

uniform float u_progress;
uniform vec2 u_resolution;
uniform vec4 u_sourceRect;
uniform vec4 u_dockRect;

varying vec2 v_uv;
varying float v_alpha;

float easeOutExpo(float t) {
  return t >= 1.0 ? 1.0 : 1.0 - pow(2.0, -10.0 * t);
}

float easeInOutCubic(float t) {
  return t < 0.5
    ? 4.0 * t * t * t
    : 1.0 - pow(-2.0 * t + 2.0, 3.0) / 2.0;
}

void main() {
  float p = clamp(u_progress, 0.0, 1.0);

  float ep = easeInOutCubic(p);
  float suction = easeOutExpo(p);

  vec2 uv = a_pos;

  vec2 dockCenter = u_dockRect.xy + u_dockRect.zw * 0.5;
  vec2 sourceCenter = u_sourceRect.xy + u_sourceRect.zw * 0.5;

  float side = (uv.x - 0.5) * 2.0;
  float bottomWeight = pow(uv.y, 1.85);

  float topWidth = mix(u_sourceRect.z, u_dockRect.z * 0.03, ep * 0.96);
  float bottomWidth = mix(u_sourceRect.z, u_dockRect.z * 1.05, suction);
  float localWidth = mix(topWidth, bottomWidth, pow(uv.y, 1.55));

  float topY = mix(u_sourceRect.y, dockCenter.y - u_dockRect.w * 0.32, ep * 0.88);
  float bottomY = mix(u_sourceRect.y + u_sourceRect.w, dockCenter.y, suction);
  float y = mix(topY, bottomY, uv.y);

  float topCenterX = mix(sourceCenter.x, dockCenter.x, ep * 0.85);
  float bottomCenterX = mix(sourceCenter.x, dockCenter.x, suction);
  float centerX = mix(topCenterX, bottomCenterX, bottomWeight);

  float curve =
    sin(uv.y * 3.14159265)
    * side
    * 120.0
    * ep
    * (1.0 - ep * 0.10);

  centerX += curve;

  float x = centerX + side * localWidth * 0.5;

  float verticalStretch =
    sin(uv.y * 3.14159265)
    * (1.0 - abs(side))
    * 100.0
    * ep
    * (1.0 - ep * 0.15);

  y += verticalStretch;

  vec2 finalPosition = vec2(x, y);

  vec2 clip = vec2(
    finalPosition.x / u_resolution.x * 2.0 - 1.0,
    1.0 - finalPosition.y / u_resolution.y * 2.0
  );

  gl_Position = vec4(clip, 0.0, 1.0);

  v_uv = a_uv;
  v_alpha = mix(1.0, 0.0, smoothstep(0.94, 1.0, p));
}
`;

export const fragmentShaderSource = `
precision highp float;

uniform sampler2D u_texture;
uniform float u_velocity;
uniform float u_intensity;

varying vec2 v_uv;
varying float v_alpha;

void main() {
  vec4 color = texture2D(u_texture, v_uv);

  // Chromatic aberration — split R/B channels by velocity from center
  float distToCenter = length(v_uv - 0.5);
  float aberration = u_velocity * 0.012 * u_intensity * distToCenter;
  float r = texture2D(u_texture, v_uv + vec2(aberration, 0.0)).r;
  float b = texture2D(u_texture, v_uv - vec2(aberration, 0.0)).b;
  color.r = mix(color.r, r, 0.6);
  color.b = mix(color.b, b, 0.6);

  // Bloom glow — sample slightly blurred version
  float bloom = 0.0;
  bloom += texture2D(u_texture, v_uv + vec2(0.002, 0.002)).a * 0.065;
  bloom += texture2D(u_texture, v_uv + vec2(-0.002, 0.002)).a * 0.065;
  bloom += texture2D(u_texture, v_uv + vec2(0.002, -0.002)).a * 0.065;
  bloom += texture2D(u_texture, v_uv + vec2(-0.002, -0.002)).a * 0.065;
  color.rgb += vec3(0.44, 0.58, 1.0) * bloom * u_intensity;

  // Vignette glow
  float vignette = 1.0 - length(v_uv - 0.5) * 0.55;
  color.rgb *= 1.0 + vignette * 0.22 * u_intensity;

  // Edge darkening toward dock
  float edgeDark = smoothstep(0.85, 1.0, v_uv.y);
  color.rgb *= 1.0 - edgeDark * 0.3;

  color.a *= v_alpha;
  gl_FragColor = color;
}
`;
