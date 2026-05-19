import type { GenieRendererOptions, Rect } from "./types";
import { fragmentShaderSource, vertexShaderSource } from "./shaders";

export class GenieRenderer {
  private canvas: HTMLCanvasElement;
  private gl: WebGLRenderingContext;
  private program: WebGLProgram;

  private texture: WebGLTexture;
  private positionBuffer: WebGLBuffer;
  private uvBuffer: WebGLBuffer;
  private indexBuffer: WebGLBuffer;

  private indexCount: number;

  private aPos: number;
  private aUv: number;

  private uProgress: WebGLUniformLocation;
  private uResolution: WebGLUniformLocation;
  private uSourceRect: WebGLUniformLocation;
  private uDockRect: WebGLUniformLocation;
  private uTexture: WebGLUniformLocation;
  private uVelocity: WebGLUniformLocation;
  private uIntensity: WebGLUniformLocation;

  constructor(
    canvas: HTMLCanvasElement,
    bitmap: HTMLCanvasElement,
    options: GenieRendererOptions = {}
  ) {
    const cols = options.cols ?? 96;
    const rows = options.rows ?? 42;

    this.canvas = canvas;
    this.cssWidth = canvas.width;
    this.cssHeight = canvas.height;

    const gl = canvas.getContext("webgl", {
      alpha: true,
      antialias: true,
      premultipliedAlpha: true,
      preserveDrawingBuffer: false,
    });

    if (!gl) {
      throw new Error("WebGL is not available.");
    }

    this.gl = gl;

    this.program = this.createProgram(vertexShaderSource, fragmentShaderSource);

    this.texture = this.createTexture(bitmap);
    this.positionBuffer = gl.createBuffer()!;
    this.uvBuffer = gl.createBuffer()!;
    this.indexBuffer = gl.createBuffer()!;

    const grid = this.createGrid(cols, rows);
    this.indexCount = grid.indices.length;

    gl.useProgram(this.program);

    gl.bindBuffer(gl.ARRAY_BUFFER, this.positionBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, grid.positions, gl.STATIC_DRAW);

    gl.bindBuffer(gl.ARRAY_BUFFER, this.uvBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, grid.uvs, gl.STATIC_DRAW);

    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, grid.indices, gl.STATIC_DRAW);

    this.aPos = gl.getAttribLocation(this.program, "a_pos");
    this.aUv = gl.getAttribLocation(this.program, "a_uv");

    this.uProgress = gl.getUniformLocation(this.program, "u_progress")!;
    this.uResolution = gl.getUniformLocation(this.program, "u_resolution")!;
    this.uSourceRect = gl.getUniformLocation(this.program, "u_sourceRect")!;
    this.uDockRect = gl.getUniformLocation(this.program, "u_dockRect")!;
    this.uTexture = gl.getUniformLocation(this.program, "u_texture")!;
    this.uVelocity = gl.getUniformLocation(this.program, "u_velocity")!;
    this.uIntensity = gl.getUniformLocation(this.program, "u_intensity")!;

    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
  }

  private cssWidth: number;
  private cssHeight: number;

  resize(width: number, height: number, dpr = window.devicePixelRatio || 1) {
    this.cssWidth = width;
    this.cssHeight = height;
    this.canvas.width = Math.round(width * dpr);
    this.canvas.height = Math.round(height * dpr);

    this.canvas.style.width = `${width}px`;
    this.canvas.style.height = `${height}px`;

    this.gl.viewport(0, 0, this.canvas.width, this.canvas.height);
  }

  render(progress: number, sourceRect: Rect, dockRect: Rect, velocity = 0, intensity = 1) {
    const gl = this.gl;

    gl.useProgram(this.program);

    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, this.texture);
    gl.uniform1i(this.uTexture, 0);

    gl.uniform1f(this.uProgress, progress);

    gl.uniform2f(this.uResolution, this.cssWidth || this.canvas.width, this.cssHeight || this.canvas.height);
    gl.uniform1f(this.uVelocity, velocity);
    gl.uniform1f(this.uIntensity, intensity);

    gl.uniform4f(
      this.uSourceRect,
      sourceRect.x,
      sourceRect.y,
      sourceRect.width,
      sourceRect.height
    );

    gl.uniform4f(
      this.uDockRect,
      dockRect.x,
      dockRect.y,
      dockRect.width,
      dockRect.height
    );

    gl.bindBuffer(gl.ARRAY_BUFFER, this.positionBuffer);
    gl.enableVertexAttribArray(this.aPos);
    gl.vertexAttribPointer(this.aPos, 2, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ARRAY_BUFFER, this.uvBuffer);
    gl.enableVertexAttribArray(this.aUv);
    gl.vertexAttribPointer(this.aUv, 2, gl.FLOAT, false, 0, 0);

    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);

    gl.drawElements(gl.TRIANGLES, this.indexCount, gl.UNSIGNED_SHORT, 0);
  }

  clear() {
    const gl = this.gl;
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);
  }

  dispose() {
    const gl = this.gl;

    gl.deleteTexture(this.texture);
    gl.deleteBuffer(this.positionBuffer);
    gl.deleteBuffer(this.uvBuffer);
    gl.deleteBuffer(this.indexBuffer);
    gl.deleteProgram(this.program);
  }

  private createTexture(bitmap: HTMLCanvasElement) {
    const gl = this.gl;
    const texture = gl.createTexture()!;

    gl.bindTexture(gl.TEXTURE_2D, texture);

    gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);

    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

    gl.texImage2D(
      gl.TEXTURE_2D,
      0,
      gl.RGBA,
      gl.RGBA,
      gl.UNSIGNED_BYTE,
      bitmap
    );

    return texture;
  }

  private createGrid(cols: number, rows: number) {
    const positions: number[] = [];
    const uvs: number[] = [];
    const indices: number[] = [];

    for (let y = 0; y <= rows; y++) {
      for (let x = 0; x <= cols; x++) {
        const u = x / cols;
        const v = y / rows;

        positions.push(u, v);
        uvs.push(u, v);
      }
    }

    for (let y = 0; y < rows; y++) {
      for (let x = 0; x < cols; x++) {
        const i = y * (cols + 1) + x;

        indices.push(i, i + 1, i + cols + 1);
        indices.push(i + 1, i + cols + 2, i + cols + 1);
      }
    }

    return {
      positions: new Float32Array(positions),
      uvs: new Float32Array(uvs),
      indices: new Uint16Array(indices),
    };
  }

  private createProgram(vertexSource: string, fragmentSource: string) {
    const gl = this.gl;

    const vertexShader = this.compileShader(gl.VERTEX_SHADER, vertexSource);
    const fragmentShader = this.compileShader(
      gl.FRAGMENT_SHADER,
      fragmentSource
    );

    const program = gl.createProgram()!;

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      const info = gl.getProgramInfoLog(program) || "Unknown WebGL link error";
      gl.deleteProgram(program);
      throw new Error(info);
    }

    gl.deleteShader(vertexShader);
    gl.deleteShader(fragmentShader);

    return program;
  }

  private compileShader(type: number, source: string) {
    const gl = this.gl;

    const shader = gl.createShader(type)!;

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      const info =
        gl.getShaderInfoLog(shader) || "Unknown WebGL shader compile error";

      gl.deleteShader(shader);

      throw new Error(info);
    }

    return shader;
  }
}
