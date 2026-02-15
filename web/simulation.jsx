import { useState, useEffect, useRef, useCallback, useMemo } from "react";
import * as THREE from "three";

// ============================================================
// TERRAIN MAPPING SIMULATION - Core Engine
// ============================================================

// Deterministic seeded random
function seededRandom(seed) {
  let s = seed;
  return () => {
    s = (s * 16807 + 0) % 2147483647;
    return (s - 1) / 2147483646;
  };
}

// Perlin-like noise
function generateHeightmap(size, params, rng) {
  const map = new Float32Array(size * size);
  const { baseHeight, hillScale, hillFreq, roughness, roughFreq } = params;
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const nx = x / size, ny = y / size;
      let h = baseHeight;
      h += hillScale * Math.sin(nx * hillFreq * Math.PI) * Math.cos(ny * hillFreq * 0.7 * Math.PI);
      h += hillScale * 0.5 * Math.sin(nx * hillFreq * 2.3 * Math.PI + 1.7) * Math.sin(ny * hillFreq * 1.9 * Math.PI + 0.8);
      h += roughness * Math.sin(nx * roughFreq * Math.PI) * Math.cos(ny * roughFreq * 1.3 * Math.PI);
      h += roughness * 0.3 * Math.sin(nx * roughFreq * 3.7 * Math.PI + 2.1) * Math.sin(ny * roughFreq * 2.9 * Math.PI);
      map[y * size + x] = h;
    }
  }
  return map;
}

// Terrain type classification
const TERRAIN_TYPES = [
  { id: 0, name: "Packed Dirt", color: "#8B7355", minH: -Infinity, maxH: -0.3, rough: [0, 0.3] },
  { id: 1, name: "Gravel", color: "#A0937D", minH: -0.3, maxH: 0.0, rough: [0.3, 0.7] },
  { id: 2, name: "Grass", color: "#5B8C3E", minH: 0.0, maxH: 0.5, rough: [0, 0.5] },
  { id: 3, name: "Tall Vegetation", color: "#3D6B2E", minH: 0.3, maxH: 1.5, rough: [0.5, 1] },
  { id: 4, name: "Rock", color: "#7C7C7C", minH: 0.5, maxH: Infinity, rough: [0.7, 1] },
  { id: 5, name: "Concrete", color: "#B0B0B0", minH: -0.2, maxH: 0.1, rough: [0, 0.15] },
  { id: 6, name: "Mud", color: "#6B5344", minH: -Infinity, maxH: -0.5, rough: [0.1, 0.4] },
  { id: 7, name: "Water", color: "#3A6B8C", minH: -Infinity, maxH: -0.8, rough: [0, 0.05] },
];

function classifyTerrain(height, roughness) {
  if (height < -0.8 && roughness < 0.1) return 7;
  if (height < -0.5 && roughness < 0.4) return 6;
  if (height > 0.5 && roughness > 0.5) return 4;
  if (height > 0.3 && roughness > 0.3) return 3;
  if (Math.abs(height) < 0.15 && roughness < 0.15) return 5;
  if (height > 0.0) return 2;
  if (roughness > 0.3) return 1;
  return 0;
}

// Camera projection math
function projectPoint(point3D, cameraPose, focalLength, sensorWidth, sensorHeight) {
  const { position, rotation } = cameraPose;
  const dx = point3D[0] - position[0];
  const dy = point3D[1] - position[1];
  const dz = point3D[2] - position[2];
  const cosR = Math.cos(-rotation);
  const sinR = Math.sin(-rotation);
  const cx = dx * cosR - dy * sinR;
  const cy = dx * sinR + dy * cosR;
  const cz = dz;
  if (cy < 0.1) return null;
  const u = (focalLength * cx / cy) + sensorWidth / 2;
  const v = (focalLength * cz / cy) + sensorHeight / 2;
  if (u < 0 || u >= sensorWidth || v < 0 || v >= sensorHeight) return null;
  return { u, v, depth: cy };
}

// Stereo depth error model
function stereoDepthError(trueDepth, baseline, focalLengthPx, disparityNoisePx) {
  const disparity = focalLengthPx * baseline / trueDepth;
  const noisyDisparity = disparity + (Math.random() - 0.5) * 2 * disparityNoisePx;
  if (noisyDisparity <= 0) return null;
  const estimatedDepth = focalLengthPx * baseline / noisyDisparity;
  return estimatedDepth;
}

// ============================================================
// TOP-DOWN MAP CANVAS RENDERER
// ============================================================

function TopDownMap({ heightmap, terrainGrid, size, worldSize, platformAngle, cameras, config, observedCells, viewMode }) {
  const canvasRef = useRef(null);
  const mapSize = 400;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const scale = mapSize / worldSize;
    const cellSize = mapSize / size;

    ctx.fillStyle = "#1a1a2e";
    ctx.fillRect(0, 0, mapSize, mapSize);

    // Draw terrain or coverage
    for (let y = 0; y < size; y++) {
      for (let x = 0; x < size; x++) {
        const idx = y * size + x;
        const wx = (x / size - 0.5) * worldSize;
        const wy = (y / size - 0.5) * worldSize;

        if (viewMode === "terrain") {
          const h = heightmap[idx];
          const tType = terrainGrid[idx];
          const tc = TERRAIN_TYPES[tType];
          ctx.fillStyle = tc.color;
          ctx.fillRect(x * cellSize, (size - 1 - y) * cellSize, cellSize + 0.5, cellSize + 0.5);
        } else if (viewMode === "height") {
          const h = heightmap[idx];
          const norm = Math.max(0, Math.min(1, (h + 2) / 4));
          const r = Math.floor(norm * 200 + 30);
          const g = Math.floor(norm * 180 + 40);
          const b = Math.floor((1 - norm) * 150 + 50);
          ctx.fillStyle = `rgb(${r},${g},${b})`;
          ctx.fillRect(x * cellSize, (size - 1 - y) * cellSize, cellSize + 0.5, cellSize + 0.5);
        } else if (viewMode === "coverage") {
          const observed = observedCells.has(idx);
          if (observed) {
            const tType = terrainGrid[idx];
            const tc = TERRAIN_TYPES[tType];
            ctx.fillStyle = tc.color;
          } else {
            ctx.fillStyle = "#0d0d1a";
          }
          ctx.fillRect(x * cellSize, (size - 1 - y) * cellSize, cellSize + 0.5, cellSize + 0.5);
        } else if (viewMode === "depth_error") {
          const observed = observedCells.has(idx);
          if (observed) {
            const dist = Math.sqrt(wx * wx + wy * wy);
            const baseline = config.stereoBaseline;
            const fp = config.focalLengthPx;
            const errM = (dist * dist) / (fp * baseline);
            const normErr = Math.min(1, errM * 20);
            const r = Math.floor(normErr * 255);
            const g = Math.floor((1 - normErr) * 255);
            ctx.fillStyle = `rgb(${r},${g},40)`;
          } else {
            ctx.fillStyle = "#0d0d1a";
          }
          ctx.fillRect(x * cellSize, (size - 1 - y) * cellSize, cellSize + 0.5, cellSize + 0.5);
        }
      }
    }

    // Draw platform
    const cx = mapSize / 2;
    const cy = mapSize / 2;
    const platSize = (3.048 / worldSize) * mapSize;
    ctx.save();
    ctx.translate(cx, cy);
    ctx.rotate(-platformAngle);
    ctx.strokeStyle = "#FFD700";
    ctx.lineWidth = 2;
    ctx.strokeRect(-platSize / 2, -platSize / 2, platSize, platSize);

    // Label sides
    ctx.fillStyle = "rgba(255,215,0,0.6)";
    ctx.font = "9px monospace";
    ctx.textAlign = "center";
    const sideLabels = ["A", "B", "C", "D"];
    const sidePositions = [
      [0, platSize / 2 + 10],
      [-platSize / 2 - 10, 0],
      [0, -platSize / 2 - 10],
      [platSize / 2 + 10, 0]
    ];
    sideLabels.forEach((label, i) => {
      ctx.fillText(label, sidePositions[i][0], sidePositions[i][1] + 3);
    });

    // Draw cameras and FOV
    cameras.forEach((cam, ci) => {
      const camScale = platSize / 3.048;
      const lx = cam.localX * camScale;
      const ly = cam.localY * camScale;
      
      // Camera dot
      ctx.fillStyle = cam.color;
      ctx.beginPath();
      ctx.arc(lx, -ly, 3, 0, Math.PI * 2);
      ctx.fill();

      // FOV cone
      const fovRad = (cam.fov * Math.PI) / 180;
      const coneLen = (cam.maxRange / worldSize) * mapSize;
      const camAngle = (cam.facing * Math.PI) / 180;
      
      ctx.beginPath();
      ctx.moveTo(lx, -ly);
      ctx.lineTo(
        lx + Math.sin(camAngle - fovRad / 2) * coneLen,
        -ly - Math.cos(camAngle - fovRad / 2) * coneLen
      );
      ctx.lineTo(
        lx + Math.sin(camAngle + fovRad / 2) * coneLen,
        -ly - Math.cos(camAngle + fovRad / 2) * coneLen
      );
      ctx.closePath();
      ctx.fillStyle = cam.color + "18";
      ctx.fill();
      ctx.strokeStyle = cam.color + "60";
      ctx.lineWidth = 0.5;
      ctx.stroke();
    });

    ctx.restore();

    // Range rings
    ctx.strokeStyle = "rgba(255,255,255,0.1)";
    ctx.lineWidth = 0.5;
    [5, 10, 20, 50].forEach(r => {
      const rPx = (r / worldSize) * mapSize;
      ctx.beginPath();
      ctx.arc(cx, cy, rPx, 0, Math.PI * 2);
      ctx.stroke();
      ctx.fillStyle = "rgba(255,255,255,0.2)";
      ctx.font = "8px monospace";
      ctx.fillText(`${r}m`, cx + rPx + 2, cy - 2);
    });

    // North indicator
    ctx.fillStyle = "rgba(255,255,255,0.5)";
    ctx.font = "10px monospace";
    ctx.textAlign = "center";
    ctx.fillText("N", cx, 12);
    ctx.beginPath();
    ctx.moveTo(cx, 16);
    ctx.lineTo(cx - 4, 24);
    ctx.lineTo(cx + 4, 24);
    ctx.closePath();
    ctx.fill();

  }, [heightmap, terrainGrid, size, worldSize, platformAngle, cameras, config, observedCells, viewMode]);

  return <canvas ref={canvasRef} width={mapSize} height={mapSize} style={{ borderRadius: 6 }} />;
}

// ============================================================
// SIMULATED CAMERA VIEW
// ============================================================

function CameraView({ camera, heightmap, terrainGrid, mapSize, worldSize, platformAngle, label }) {
  const canvasRef = useRef(null);
  const viewW = 320;
  const viewH = 200;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = "#000";
    ctx.fillRect(0, 0, viewW, viewH);

    if (!camera) return;

    const cosA = Math.cos(platformAngle);
    const sinA = Math.sin(platformAngle);
    const camWorldX = camera.localX * cosA - camera.localY * sinA;
    const camWorldY = camera.localX * sinA + camera.localY * cosA;
    const camWorldAngle = platformAngle + (camera.facing * Math.PI / 180);
    const fovRad = camera.fov * Math.PI / 180;

    // Raycasting - render terrain from camera perspective
    const numRays = viewW;
    const vertRays = viewH;

    for (let col = 0; col < numRays; col++) {
      const rayAngle = camWorldAngle + (col / numRays - 0.5) * fovRad;
      const cosR = Math.sin(rayAngle);
      const sinR = Math.cos(rayAngle);

      let prevScreenY = viewH;
      for (let d = 1; d < camera.maxRange; d += 0.3) {
        const wx = camWorldX + cosR * d;
        const wy = camWorldY + sinR * d;

        const gx = Math.floor((wx / worldSize + 0.5) * mapSize);
        const gy = Math.floor((wy / worldSize + 0.5) * mapSize);
        if (gx < 0 || gx >= mapSize || gy < 0 || gy >= mapSize) continue;

        const idx = gy * mapSize + gx;
        const h = heightmap[idx];
        const tType = terrainGrid[idx];
        const tc = TERRAIN_TYPES[tType];

        const relH = h - 1.5; // camera height ~1.5m
        const screenY = viewH / 2 - (relH / d) * viewH * 1.5;

        if (screenY < prevScreenY) {
          // Distance fog
          const fog = Math.min(1, d / camera.maxRange);
          const baseColor = tc.color;
          const r = parseInt(baseColor.slice(1, 3), 16);
          const g = parseInt(baseColor.slice(3, 5), 16);
          const b = parseInt(baseColor.slice(5, 7), 16);
          const fogR = 40, fogG = 45, fogB = 60;
          const fr = Math.floor(r * (1 - fog) + fogR * fog);
          const fg = Math.floor(g * (1 - fog) + fogG * fog);
          const fb = Math.floor(b * (1 - fog) + fogB * fog);

          // Simple lighting
          const sunAngle = 0.8;
          const shade = 0.6 + 0.4 * Math.max(0, Math.cos(d * 0.1 + col * 0.01));

          ctx.fillStyle = `rgb(${Math.floor(fr * shade)},${Math.floor(fg * shade)},${Math.floor(fb * shade)})`;
          ctx.fillRect(col, Math.max(0, Math.floor(screenY)), 1, Math.ceil(prevScreenY - screenY));
          prevScreenY = screenY;
        }
      }
      // Sky
      if (prevScreenY > 0) {
        const grad = ctx.createLinearGradient(0, 0, 0, prevScreenY);
        grad.addColorStop(0, "#1a2a4a");
        grad.addColorStop(1, "#2a3a5a");
        ctx.fillStyle = grad;
        ctx.fillRect(col, 0, 1, prevScreenY);
      }
    }

    // Label
    ctx.fillStyle = camera.color;
    ctx.font = "bold 11px monospace";
    ctx.fillText(label, 6, 14);
    ctx.fillStyle = "rgba(255,255,255,0.5)";
    ctx.font = "9px monospace";
    ctx.fillText(`FOV: ${camera.fov}° | Range: ${camera.maxRange}m`, 6, 26);

  }, [camera, heightmap, terrainGrid, mapSize, worldSize, platformAngle, label]);

  return <canvas ref={canvasRef} width={viewW} height={viewH} style={{ borderRadius: 4 }} />;
}

// ============================================================
// DEPTH MAP VIEW
// ============================================================

function DepthMapView({ camera, heightmap, mapSize, worldSize, platformAngle, config, mode, label }) {
  const canvasRef = useRef(null);
  const viewW = 320;
  const viewH = 200;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = "#000";
    ctx.fillRect(0, 0, viewW, viewH);
    if (!camera) return;

    const cosA = Math.cos(platformAngle);
    const sinA = Math.sin(platformAngle);
    const camWorldX = camera.localX * cosA - camera.localY * sinA;
    const camWorldY = camera.localX * sinA + camera.localY * cosA;
    const camWorldAngle = platformAngle + (camera.facing * Math.PI / 180);
    const fovRad = camera.fov * Math.PI / 180;

    for (let col = 0; col < viewW; col++) {
      const rayAngle = camWorldAngle + (col / viewW - 0.5) * fovRad;
      const cosR = Math.sin(rayAngle);
      const sinR = Math.cos(rayAngle);

      let prevScreenY = viewH;
      for (let d = 1; d < camera.maxRange; d += 0.3) {
        const wx = camWorldX + cosR * d;
        const wy = camWorldY + sinR * d;
        const gx = Math.floor((wx / worldSize + 0.5) * mapSize);
        const gy = Math.floor((wy / worldSize + 0.5) * mapSize);
        if (gx < 0 || gx >= mapSize || gy < 0 || gy >= mapSize) continue;

        const idx = gy * mapSize + gx;
        const h = heightmap[idx];
        const relH = h - 1.5;
        const screenY = viewH / 2 - (relH / d) * viewH * 1.5;

        if (screenY < prevScreenY) {
          if (mode === "ground_truth") {
            const norm = Math.min(1, d / camera.maxRange);
            const r = Math.floor((1 - norm) * 255);
            const g = Math.floor((1 - norm) * 200);
            const b = Math.floor(norm * 100 + 50);
            ctx.fillStyle = `rgb(${r},${g},${b})`;
          } else {
            // Estimated depth with stereo error
            const estD = stereoDepthError(d, config.stereoBaseline, config.focalLengthPx, config.disparityNoise);
            if (estD) {
              const norm = Math.min(1, estD / camera.maxRange);
              const err = Math.abs(estD - d);
              const errNorm = Math.min(1, err * 10);
              if (mode === "estimated") {
                const r = Math.floor((1 - norm) * 255);
                const g = Math.floor((1 - norm) * 200);
                const b = Math.floor(norm * 100 + 50);
                ctx.fillStyle = `rgb(${r},${g},${b})`;
              } else {
                // Error visualization
                const r = Math.floor(errNorm * 255);
                const g = Math.floor((1 - errNorm) * 255);
                ctx.fillStyle = `rgb(${r},${g},30)`;
              }
            } else {
              ctx.fillStyle = "#000";
            }
          }
          ctx.fillRect(col, Math.max(0, Math.floor(screenY)), 1, Math.ceil(prevScreenY - screenY));
          prevScreenY = screenY;
        }
      }
    }

    ctx.fillStyle = "#FFD700";
    ctx.font = "bold 10px monospace";
    ctx.fillText(label, 6, 14);
  }, [camera, heightmap, mapSize, worldSize, platformAngle, config, mode, label]);

  return <canvas ref={canvasRef} width={viewW} height={viewH} style={{ borderRadius: 4 }} />;
}

// ============================================================
// STEREO ACCURACY CHART
// ============================================================

function AccuracyChart({ config }) {
  const canvasRef = useRef(null);
  const w = 360, h = 180;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = "#0d0d1a";
    ctx.fillRect(0, 0, w, h);

    const margin = { t: 25, r: 15, b: 30, l: 50 };
    const pw = w - margin.l - margin.r;
    const ph = h - margin.t - margin.b;

    // Axis
    ctx.strokeStyle = "rgba(255,255,255,0.2)";
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(margin.l, margin.t);
    ctx.lineTo(margin.l, margin.t + ph);
    ctx.lineTo(margin.l + pw, margin.t + ph);
    ctx.stroke();

    const maxRange = 100;
    const maxErr = 200; // cm

    // Grid lines
    ctx.strokeStyle = "rgba(255,255,255,0.06)";
    for (let e = 0; e <= maxErr; e += 50) {
      const y = margin.t + ph - (e / maxErr) * ph;
      ctx.beginPath();
      ctx.moveTo(margin.l, y);
      ctx.lineTo(margin.l + pw, y);
      ctx.stroke();
      ctx.fillStyle = "rgba(255,255,255,0.35)";
      ctx.font = "8px monospace";
      ctx.textAlign = "right";
      ctx.fillText(`${e}cm`, margin.l - 4, y + 3);
    }

    for (let r = 0; r <= maxRange; r += 20) {
      const x = margin.l + (r / maxRange) * pw;
      ctx.fillStyle = "rgba(255,255,255,0.35)";
      ctx.font = "8px monospace";
      ctx.textAlign = "center";
      ctx.fillText(`${r}m`, x, margin.t + ph + 14);
    }

    // 1-inch line
    ctx.strokeStyle = "rgba(100,255,100,0.3)";
    ctx.setLineDash([3, 3]);
    const inchY = margin.t + ph - (2.54 / maxErr) * ph;
    ctx.beginPath();
    ctx.moveTo(margin.l, inchY);
    ctx.lineTo(margin.l + pw, inchY);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = "rgba(100,255,100,0.5)";
    ctx.font = "7px monospace";
    ctx.textAlign = "left";
    ctx.fillText("1 inch", margin.l + pw - 30, inchY - 3);

    // Plot curves for current stereo and temporal stereo
    const configs_to_plot = [
      { baseline: config.stereoBaseline, noise: config.disparityNoise, color: "#FFD700", label: "Stereo Pair" },
      { baseline: config.temporalBaseline, noise: config.disparityNoise * 1.5, color: "#FF6B6B", label: "Temporal Stereo" },
    ];

    configs_to_plot.forEach(({ baseline, noise, color, label }) => {
      ctx.strokeStyle = color;
      ctx.lineWidth = 2;
      ctx.beginPath();
      let started = false;
      for (let r = 1; r <= maxRange; r += 0.5) {
        const errM = (r * r * noise) / (config.focalLengthPx * baseline);
        const errCm = errM * 100;
        const x = margin.l + (r / maxRange) * pw;
        const y = margin.t + ph - Math.min(1, errCm / maxErr) * ph;
        if (!started) { ctx.moveTo(x, y); started = true; }
        else ctx.lineTo(x, y);
      }
      ctx.stroke();

      // Legend
      const li = configs_to_plot.indexOf(configs_to_plot.find(c => c.color === color));
      ctx.fillStyle = color;
      ctx.font = "9px monospace";
      ctx.textAlign = "left";
      ctx.fillText(`── ${label} (B=${baseline.toFixed(2)}m)`, margin.l + 5, margin.t + 12 + li * 12);
    });

    ctx.fillStyle = "rgba(255,255,255,0.5)";
    ctx.font = "9px monospace";
    ctx.textAlign = "center";
    ctx.fillText("Range (m)", margin.l + pw / 2, h - 3);

    ctx.save();
    ctx.translate(10, margin.t + ph / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.fillText("Depth Error (cm)", 0, 0);
    ctx.restore();

  }, [config]);

  return <canvas ref={canvasRef} width={w} height={h} style={{ borderRadius: 4 }} />;
}

// ============================================================
// COVERAGE OVER TIME CHART
// ============================================================

function CoverageChart({ coverageHistory }) {
  const canvasRef = useRef(null);
  const w = 360, h = 120;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = "#0d0d1a";
    ctx.fillRect(0, 0, w, h);

    if (coverageHistory.length < 2) return;

    const margin = { t: 15, r: 15, b: 25, l: 45 };
    const pw = w - margin.l - margin.r;
    const ph = h - margin.t - margin.b;

    ctx.strokeStyle = "rgba(255,255,255,0.2)";
    ctx.beginPath();
    ctx.moveTo(margin.l, margin.t);
    ctx.lineTo(margin.l, margin.t + ph);
    ctx.lineTo(margin.l + pw, margin.t + ph);
    ctx.stroke();

    // Y axis labels
    for (let p = 0; p <= 100; p += 25) {
      const y = margin.t + ph - (p / 100) * ph;
      ctx.fillStyle = "rgba(255,255,255,0.3)";
      ctx.font = "8px monospace";
      ctx.textAlign = "right";
      ctx.fillText(`${p}%`, margin.l - 4, y + 3);
    }

    // Plot
    ctx.strokeStyle = "#00CC88";
    ctx.lineWidth = 2;
    ctx.beginPath();
    const maxT = coverageHistory[coverageHistory.length - 1]?.t || 1;
    coverageHistory.forEach((pt, i) => {
      const x = margin.l + (pt.t / maxT) * pw;
      const y = margin.t + ph - (pt.pct / 100) * ph;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    ctx.stroke();

    ctx.fillStyle = "rgba(255,255,255,0.4)";
    ctx.font = "8px monospace";
    ctx.textAlign = "center";
    ctx.fillText("Rotation (degrees)", margin.l + pw / 2, h - 3);

  }, [coverageHistory]);

  return <canvas ref={canvasRef} width={w} height={h} style={{ borderRadius: 4 }} />;
}

// ============================================================
// MAIN SIMULATION COMPONENT
// ============================================================

const DEFAULT_CAMERAS = [
  // Side A (bottom, facing south/down)
  { id: "A1", side: "A", localX: -0.8, localY: -1.524, facing: 180, fov: 90, maxRange: 60, focalMM: 4, color: "#FF6B6B" },
  { id: "A2", side: "A", localX: 0.8, localY: -1.524, facing: 180, fov: 90, maxRange: 60, focalMM: 4, color: "#FF9B9B" },
  // Side B (left, facing west/left)
  { id: "B1", side: "B", localX: -1.524, localY: -0.8, facing: 270, fov: 90, maxRange: 60, focalMM: 4, color: "#6B9BFF" },
  { id: "B2", side: "B", localX: -1.524, localY: 0.8, facing: 270, fov: 90, maxRange: 60, focalMM: 4, color: "#9BBBFF" },
  // Side C (top, facing north/up)
  { id: "C1", side: "C", localX: -0.8, localY: 1.524, facing: 0, fov: 90, maxRange: 60, focalMM: 4, color: "#6BFF9B" },
  { id: "C2", side: "C", localX: 0.8, localY: 1.524, facing: 0, fov: 90, maxRange: 60, focalMM: 4, color: "#9BFFBB" },
];

const TERRAIN_PRESETS = {
  gentle: { baseHeight: 0, hillScale: 0.8, hillFreq: 1.5, roughness: 0.2, roughFreq: 6 },
  hilly: { baseHeight: 0, hillScale: 2.0, hillFreq: 2.5, roughness: 0.5, roughFreq: 10 },
  rough: { baseHeight: -0.3, hillScale: 1.2, hillFreq: 3, roughness: 1.0, roughFreq: 15 },
  flat: { baseHeight: 0, hillScale: 0.1, hillFreq: 1, roughness: 0.05, roughFreq: 3 },
};

export default function Simulation() {
  // === STATE ===
  const [terrainPreset, setTerrainPreset] = useState("gentle");
  const [worldSize, setWorldSize] = useState(120); // meters
  const [gridSize] = useState(200);
  const [platformAngle, setPlatformAngle] = useState(0);
  const [isRotating, setIsRotating] = useState(false);
  const [rotationSpeed, setRotationSpeed] = useState(10); // deg/sec
  const [cameras, setCameras] = useState(DEFAULT_CAMERAS);
  const [activeCamSides, setActiveCamSides] = useState({ A: true, B: true, C: true });
  const [stereoBaseline, setStereoBaseline] = useState(1.6);
  const [temporalBaseline, setTemporalBaseline] = useState(0.5);
  const [focalLengthPx, setFocalLengthPx] = useState(1000);
  const [disparityNoise, setDisparityNoise] = useState(0.8);
  const [cameraFOV, setCameraFOV] = useState(90);
  const [cameraRange, setCameraRange] = useState(60);
  const [viewMode, setViewMode] = useState("coverage");
  const [depthViewMode, setDepthViewMode] = useState("ground_truth");
  const [selectedCamView, setSelectedCamView] = useState("A1");
  const [observedCells, setObservedCells] = useState(new Set());
  const [coverageHistory, setCoverageHistory] = useState([{ t: 0, pct: 0 }]);
  const [simTime, setSimTime] = useState(0);
  const [totalCoverage, setTotalCoverage] = useState(0);
  const [tab, setTab] = useState("cameras");

  const animRef = useRef(null);
  const lastTimeRef = useRef(null);

  // Generate terrain
  const rng = useMemo(() => seededRandom(42), []);
  const terrainParams = useMemo(() => TERRAIN_PRESETS[terrainPreset], [terrainPreset]);
  const heightmap = useMemo(() => generateHeightmap(gridSize, terrainParams, rng), [gridSize, terrainParams]);
  const terrainGrid = useMemo(() => {
    const grid = new Uint8Array(gridSize * gridSize);
    for (let i = 0; i < grid.length; i++) {
      const h = heightmap[i];
      const x = (i % gridSize) / gridSize;
      const roughApprox = Math.abs(heightmap[Math.min(i + 1, grid.length - 1)] - h) * 10;
      grid[i] = classifyTerrain(h, roughApprox);
    }
    return grid;
  }, [heightmap, gridSize]);

  const config = useMemo(() => ({
    stereoBaseline,
    temporalBaseline,
    focalLengthPx,
    disparityNoise,
  }), [stereoBaseline, temporalBaseline, focalLengthPx, disparityNoise]);

  // Update cameras when FOV/range changes
  useEffect(() => {
    setCameras(prev => prev.map(c => ({ ...c, fov: cameraFOV, maxRange: cameraRange })));
  }, [cameraFOV, cameraRange]);

  // Active cameras
  const activeCameras = useMemo(() =>
    cameras.filter(c => activeCamSides[c.side]),
    [cameras, activeCamSides]
  );

  // Coverage computation
  const updateCoverage = useCallback((angle) => {
    const newObserved = new Set(observedCells);
    const cosA = Math.cos(angle);
    const sinA = Math.sin(angle);

    activeCameras.forEach(cam => {
      const camWorldX = cam.localX * cosA - cam.localY * sinA;
      const camWorldY = cam.localX * sinA + cam.localY * cosA;
      const camWorldAngle = angle + (cam.facing * Math.PI / 180);
      const fovRad = cam.fov * Math.PI / 180;

      for (let ray = 0; ray < 60; ray++) {
        const rayAngle = camWorldAngle + (ray / 60 - 0.5) * fovRad;
        const cosR = Math.sin(rayAngle);
        const sinR = Math.cos(rayAngle);

        for (let d = 2; d < cam.maxRange; d += 0.5) {
          const wx = camWorldX + cosR * d;
          const wy = camWorldY + sinR * d;
          const gx = Math.floor((wx / worldSize + 0.5) * gridSize);
          const gy = Math.floor((wy / worldSize + 0.5) * gridSize);
          if (gx >= 0 && gx < gridSize && gy >= 0 && gy < gridSize) {
            newObserved.add(gy * gridSize + gx);
          }
        }
      }
    });

    setObservedCells(newObserved);
    const totalCells = gridSize * gridSize;
    const pct = (newObserved.size / totalCells) * 100;
    setTotalCoverage(pct);
    return pct;
  }, [activeCameras, observedCells, gridSize, worldSize]);

  // Animation loop
  useEffect(() => {
    if (!isRotating) {
      if (animRef.current) cancelAnimationFrame(animRef.current);
      lastTimeRef.current = null;
      return;
    }

    const animate = (timestamp) => {
      if (!lastTimeRef.current) lastTimeRef.current = timestamp;
      const dt = (timestamp - lastTimeRef.current) / 1000;
      lastTimeRef.current = timestamp;

      setPlatformAngle(prev => {
        const newAngle = prev + (rotationSpeed * Math.PI / 180) * dt;
        return newAngle;
      });

      setSimTime(prev => prev + dt);
      animRef.current = requestAnimationFrame(animate);
    };

    animRef.current = requestAnimationFrame(animate);
    return () => { if (animRef.current) cancelAnimationFrame(animRef.current); };
  }, [isRotating, rotationSpeed]);

  // Update coverage when angle changes
  useEffect(() => {
    const pct = updateCoverage(platformAngle);
    const angleDeg = (platformAngle * 180 / Math.PI) % 360;
    setCoverageHistory(prev => {
      const last = prev[prev.length - 1];
      if (Math.abs(angleDeg - (last?.t || 0)) > 2) {
        return [...prev.slice(-200), { t: angleDeg + Math.floor(platformAngle / (2 * Math.PI)) * 360, pct }];
      }
      return prev;
    });
  }, [platformAngle]);

  const resetSim = () => {
    setPlatformAngle(0);
    setObservedCells(new Set());
    setCoverageHistory([{ t: 0, pct: 0 }]);
    setSimTime(0);
    setTotalCoverage(0);
    setIsRotating(false);
    lastTimeRef.current = null;
  };

  const selectedCam = activeCameras.find(c => c.id === selectedCamView) || activeCameras[0];

  // Compute stats
  const angleDeg = ((platformAngle * 180 / Math.PI) % 360).toFixed(1);
  const rotations = (platformAngle / (2 * Math.PI)).toFixed(2);

  // Stereo max range for 1-inch accuracy
  const inchRange = Math.sqrt(0.0254 * focalLengthPx * stereoBaseline / disparityNoise);
  const tempInchRange = Math.sqrt(0.0254 * focalLengthPx * temporalBaseline / (disparityNoise * 1.5));

  // Styles
  const panelBg = "rgba(13,13,30,0.95)";
  const borderColor = "rgba(255,215,0,0.15)";
  const accentGold = "#FFD700";
  const textDim = "rgba(255,255,255,0.5)";
  const textBright = "rgba(255,255,255,0.9)";

  return (
    <div style={{
      fontFamily: "'JetBrains Mono', 'Fira Code', 'SF Mono', monospace",
      background: "linear-gradient(135deg, #0a0a1a 0%, #111128 50%, #0a0a1a 100%)",
      color: textBright,
      minHeight: "100vh",
      padding: 16,
      boxSizing: "border-box",
    }}>
      {/* Header */}
      <div style={{ textAlign: "center", marginBottom: 16, paddingBottom: 12, borderBottom: `1px solid ${borderColor}` }}>
        <h1 style={{ fontSize: 18, fontWeight: 700, color: accentGold, margin: 0, letterSpacing: 2 }}>
          TERRAIN MAPPING SIMULATION
        </h1>
        <div style={{ fontSize: 10, color: textDim, marginTop: 4, letterSpacing: 1 }}>
          MULTI-CAMERA ROTATING PLATFORM • STEREO & TEMPORAL DEPTH • SEMANTIC TERRAIN CLASSIFICATION
        </div>
      </div>

      <div style={{ display: "flex", gap: 12, flexWrap: "wrap", justifyContent: "center" }}>
        {/* LEFT: Controls */}
        <div style={{ width: 280, flexShrink: 0 }}>
          {/* Sim Controls */}
          <div style={{ background: panelBg, border: `1px solid ${borderColor}`, borderRadius: 8, padding: 12, marginBottom: 10 }}>
            <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 8, fontWeight: 600 }}>SIMULATION</div>
            <div style={{ display: "flex", gap: 6, marginBottom: 10 }}>
              <button onClick={() => setIsRotating(!isRotating)} style={{
                flex: 1, padding: "6px 0", background: isRotating ? "rgba(255,100,100,0.2)" : "rgba(100,255,100,0.2)",
                border: `1px solid ${isRotating ? "#FF6B6B" : "#6BFF9B"}`, borderRadius: 4, color: isRotating ? "#FF6B6B" : "#6BFF9B",
                cursor: "pointer", fontSize: 10, fontFamily: "inherit", fontWeight: 600
              }}>
                {isRotating ? "⏸ STOP" : "▶ START"}
              </button>
              <button onClick={resetSim} style={{
                flex: 1, padding: "6px 0", background: "rgba(255,215,0,0.1)",
                border: `1px solid ${borderColor}`, borderRadius: 4, color: accentGold,
                cursor: "pointer", fontSize: 10, fontFamily: "inherit"
              }}>↺ RESET</button>
            </div>

            <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 4, fontSize: 9, color: textDim }}>
              <div>Angle: <span style={{ color: textBright }}>{angleDeg}°</span></div>
              <div>Rotations: <span style={{ color: textBright }}>{rotations}</span></div>
              <div>Time: <span style={{ color: textBright }}>{simTime.toFixed(1)}s</span></div>
              <div>Coverage: <span style={{ color: "#00CC88" }}>{totalCoverage.toFixed(1)}%</span></div>
            </div>

            <div style={{ marginTop: 8 }}>
              <label style={{ fontSize: 9, color: textDim }}>Rotation Speed: {rotationSpeed}°/s</label>
              <input type="range" min={1} max={90} value={rotationSpeed} onChange={e => setRotationSpeed(Number(e.target.value))}
                style={{ width: "100%", height: 4 }} />
            </div>

            <div style={{ marginTop: 4 }}>
              <label style={{ fontSize: 9, color: textDim }}>Manual Angle: {angleDeg}°</label>
              <input type="range" min={0} max={628} value={Math.round(platformAngle * 100) % 628}
                onChange={e => { if (!isRotating) setPlatformAngle(Number(e.target.value) / 100); }}
                style={{ width: "100%", height: 4 }} disabled={isRotating} />
            </div>
          </div>

          {/* Tabs */}
          <div style={{ display: "flex", gap: 2, marginBottom: 2 }}>
            {["cameras", "stereo", "terrain", "view"].map(t => (
              <button key={t} onClick={() => setTab(t)} style={{
                flex: 1, padding: "5px 0", fontSize: 9, fontFamily: "inherit", fontWeight: tab === t ? 700 : 400,
                background: tab === t ? "rgba(255,215,0,0.15)" : panelBg,
                border: `1px solid ${tab === t ? accentGold : borderColor}`,
                borderBottom: tab === t ? "none" : `1px solid ${borderColor}`,
                borderRadius: "4px 4px 0 0", color: tab === t ? accentGold : textDim, cursor: "pointer",
                textTransform: "uppercase", letterSpacing: 1,
              }}>{t}</button>
            ))}
          </div>

          <div style={{ background: panelBg, border: `1px solid ${borderColor}`, borderRadius: "0 0 8px 8px", padding: 12 }}>
            {tab === "cameras" && (
              <>
                <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 8, fontWeight: 600 }}>CAMERA CONFIGURATION</div>
                <div style={{ fontSize: 9, color: textDim, marginBottom: 8 }}>Active Sides:</div>
                <div style={{ display: "flex", gap: 6, marginBottom: 10 }}>
                  {["A", "B", "C"].map(side => (
                    <button key={side} onClick={() => setActiveCamSides(prev => ({ ...prev, [side]: !prev[side] }))}
                      style={{
                        flex: 1, padding: "4px 0", fontSize: 10, fontFamily: "inherit",
                        background: activeCamSides[side] ? "rgba(255,215,0,0.15)" : "rgba(255,255,255,0.03)",
                        border: `1px solid ${activeCamSides[side] ? accentGold : "rgba(255,255,255,0.1)"}`,
                        borderRadius: 4, color: activeCamSides[side] ? accentGold : textDim, cursor: "pointer",
                      }}>Side {side} {activeCamSides[side] ? "✓" : "✗"}</button>
                  ))}
                </div>

                <div style={{ marginBottom: 6 }}>
                  <label style={{ fontSize: 9, color: textDim }}>FOV: {cameraFOV}°</label>
                  <input type="range" min={30} max={170} value={cameraFOV} onChange={e => setCameraFOV(Number(e.target.value))}
                    style={{ width: "100%" }} />
                </div>
                <div style={{ marginBottom: 6 }}>
                  <label style={{ fontSize: 9, color: textDim }}>Max Range: {cameraRange}m</label>
                  <input type="range" min={10} max={150} value={cameraRange} onChange={e => setCameraRange(Number(e.target.value))}
                    style={{ width: "100%" }} />
                </div>

                <div style={{ fontSize: 9, color: textDim, marginTop: 8, marginBottom: 4 }}>View Camera:</div>
                <div style={{ display: "flex", gap: 3, flexWrap: "wrap" }}>
                  {activeCameras.map(c => (
                    <button key={c.id} onClick={() => setSelectedCamView(c.id)} style={{
                      padding: "3px 8px", fontSize: 9, fontFamily: "inherit",
                      background: selectedCamView === c.id ? c.color + "30" : "transparent",
                      border: `1px solid ${selectedCamView === c.id ? c.color : "rgba(255,255,255,0.1)"}`,
                      borderRadius: 3, color: c.color, cursor: "pointer",
                    }}>{c.id}</button>
                  ))}
                </div>
              </>
            )}

            {tab === "stereo" && (
              <>
                <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 8, fontWeight: 600 }}>STEREO PARAMETERS</div>
                <div style={{ marginBottom: 6 }}>
                  <label style={{ fontSize: 9, color: textDim }}>Stereo Baseline: {stereoBaseline.toFixed(2)}m</label>
                  <input type="range" min={10} max={350} value={stereoBaseline * 100}
                    onChange={e => setStereoBaseline(Number(e.target.value) / 100)} style={{ width: "100%" }} />
                  <div style={{ fontSize: 8, color: textDim }}>Same-side camera separation</div>
                </div>
                <div style={{ marginBottom: 6 }}>
                  <label style={{ fontSize: 9, color: textDim }}>Temporal Baseline: {temporalBaseline.toFixed(2)}m</label>
                  <input type="range" min={5} max={300} value={temporalBaseline * 100}
                    onChange={e => setTemporalBaseline(Number(e.target.value) / 100)} style={{ width: "100%" }} />
                  <div style={{ fontSize: 8, color: textDim }}>Effective baseline from rotation</div>
                </div>
                <div style={{ marginBottom: 6 }}>
                  <label style={{ fontSize: 9, color: textDim }}>Focal Length: {focalLengthPx}px</label>
                  <input type="range" min={200} max={4000} value={focalLengthPx}
                    onChange={e => setFocalLengthPx(Number(e.target.value))} style={{ width: "100%" }} />
                </div>
                <div style={{ marginBottom: 8 }}>
                  <label style={{ fontSize: 9, color: textDim }}>Disparity Noise: {disparityNoise.toFixed(2)}px</label>
                  <input type="range" min={10} max={200} value={disparityNoise * 100}
                    onChange={e => setDisparityNoise(Number(e.target.value) / 100)} style={{ width: "100%" }} />
                </div>

                <div style={{ background: "rgba(255,215,0,0.05)", borderRadius: 4, padding: 8, fontSize: 9 }}>
                  <div style={{ color: accentGold, marginBottom: 4, fontWeight: 600 }}>1-INCH ACCURACY RANGE</div>
                  <div style={{ color: "#6BFF9B" }}>Stereo Pair: <b>{inchRange.toFixed(1)}m</b></div>
                  <div style={{ color: "#FF6B6B" }}>Temporal: <b>{tempInchRange.toFixed(1)}m</b></div>
                  <div style={{ color: textDim, marginTop: 4, fontSize: 8 }}>
                    Cross-platform (3m): <b>{Math.sqrt(0.0254 * focalLengthPx * 3.048 / disparityNoise).toFixed(1)}m</b>
                  </div>
                </div>
              </>
            )}

            {tab === "terrain" && (
              <>
                <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 8, fontWeight: 600 }}>TERRAIN</div>
                <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 4, marginBottom: 10 }}>
                  {Object.keys(TERRAIN_PRESETS).map(p => (
                    <button key={p} onClick={() => { setTerrainPreset(p); resetSim(); }} style={{
                      padding: "5px 0", fontSize: 9, fontFamily: "inherit", textTransform: "capitalize",
                      background: terrainPreset === p ? "rgba(255,215,0,0.15)" : "transparent",
                      border: `1px solid ${terrainPreset === p ? accentGold : "rgba(255,255,255,0.1)"}`,
                      borderRadius: 4, color: terrainPreset === p ? accentGold : textDim, cursor: "pointer",
                    }}>{p}</button>
                  ))}
                </div>

                <div style={{ marginBottom: 6 }}>
                  <label style={{ fontSize: 9, color: textDim }}>World Size: {worldSize}m × {worldSize}m</label>
                  <input type="range" min={40} max={300} value={worldSize} onChange={e => { setWorldSize(Number(e.target.value)); resetSim(); }}
                    style={{ width: "100%" }} />
                </div>

                <div style={{ fontSize: 9, color: textDim, marginTop: 8, marginBottom: 4 }}>Terrain Types:</div>
                <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 3 }}>
                  {TERRAIN_TYPES.map(t => (
                    <div key={t.id} style={{ display: "flex", alignItems: "center", gap: 4, fontSize: 8 }}>
                      <div style={{ width: 10, height: 10, borderRadius: 2, background: t.color, flexShrink: 0 }} />
                      <span style={{ color: textDim }}>{t.name}</span>
                    </div>
                  ))}
                </div>
              </>
            )}

            {tab === "view" && (
              <>
                <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 8, fontWeight: 600 }}>MAP VIEW MODE</div>
                {[
                  { id: "coverage", label: "Coverage Map", desc: "Observed vs unknown" },
                  { id: "terrain", label: "Terrain Classes", desc: "Ground truth terrain types" },
                  { id: "height", label: "Elevation", desc: "Heightmap visualization" },
                  { id: "depth_error", label: "Depth Error", desc: "Expected stereo error by range" },
                ].map(m => (
                  <button key={m.id} onClick={() => setViewMode(m.id)} style={{
                    display: "block", width: "100%", padding: "6px 8px", marginBottom: 4, textAlign: "left",
                    fontSize: 9, fontFamily: "inherit",
                    background: viewMode === m.id ? "rgba(255,215,0,0.12)" : "transparent",
                    border: `1px solid ${viewMode === m.id ? accentGold : "rgba(255,255,255,0.06)"}`,
                    borderRadius: 4, color: viewMode === m.id ? accentGold : textBright, cursor: "pointer",
                  }}>
                    <div style={{ fontWeight: 600 }}>{m.label}</div>
                    <div style={{ color: textDim, fontSize: 8 }}>{m.desc}</div>
                  </button>
                ))}

                <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginTop: 10, marginBottom: 6, fontWeight: 600 }}>DEPTH VIEW MODE</div>
                {["ground_truth", "estimated", "error"].map(m => (
                  <button key={m} onClick={() => setDepthViewMode(m)} style={{
                    padding: "4px 10px", marginRight: 4, marginBottom: 4,
                    fontSize: 9, fontFamily: "inherit", textTransform: "capitalize",
                    background: depthViewMode === m ? "rgba(255,215,0,0.12)" : "transparent",
                    border: `1px solid ${depthViewMode === m ? accentGold : "rgba(255,255,255,0.06)"}`,
                    borderRadius: 4, color: depthViewMode === m ? accentGold : textDim, cursor: "pointer",
                  }}>{m.replace("_", " ")}</button>
                ))}
              </>
            )}
          </div>
        </div>

        {/* CENTER: Main visualizations */}
        <div style={{ flex: 1, minWidth: 400, maxWidth: 820 }}>
          <div style={{ display: "flex", gap: 10, flexWrap: "wrap" }}>
            {/* Top-down map */}
            <div style={{ background: panelBg, border: `1px solid ${borderColor}`, borderRadius: 8, padding: 10 }}>
              <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 6, fontWeight: 600 }}>
                {viewMode === "coverage" ? "COVERAGE MAP" : viewMode === "terrain" ? "TERRAIN CLASSIFICATION" : viewMode === "height" ? "ELEVATION MAP" : "DEPTH ERROR MAP"}
              </div>
              <TopDownMap
                heightmap={heightmap} terrainGrid={terrainGrid} size={gridSize} worldSize={worldSize}
                platformAngle={platformAngle} cameras={activeCameras} config={config}
                observedCells={observedCells} viewMode={viewMode}
              />
            </div>

            {/* Right column */}
            <div style={{ flex: 1, minWidth: 320, display: "flex", flexDirection: "column", gap: 10 }}>
              {/* Camera view */}
              <div style={{ background: panelBg, border: `1px solid ${borderColor}`, borderRadius: 8, padding: 10 }}>
                <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 6, fontWeight: 600 }}>
                  CAMERA VIEW — {selectedCam?.id || "N/A"}
                </div>
                <CameraView
                  camera={selectedCam} heightmap={heightmap} terrainGrid={terrainGrid}
                  mapSize={gridSize} worldSize={worldSize} platformAngle={platformAngle}
                  label={selectedCam?.id || ""}
                />
              </div>

              {/* Depth view */}
              <div style={{ background: panelBg, border: `1px solid ${borderColor}`, borderRadius: 8, padding: 10 }}>
                <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 6, fontWeight: 600 }}>
                  DEPTH — {depthViewMode.replace("_", " ").toUpperCase()}
                </div>
                <DepthMapView
                  camera={selectedCam} heightmap={heightmap} mapSize={gridSize}
                  worldSize={worldSize} platformAngle={platformAngle} config={config}
                  mode={depthViewMode} label={selectedCam?.id || ""}
                />
              </div>
            </div>
          </div>

          {/* Bottom row: charts */}
          <div style={{ display: "flex", gap: 10, marginTop: 10, flexWrap: "wrap" }}>
            <div style={{ background: panelBg, border: `1px solid ${borderColor}`, borderRadius: 8, padding: 10, flex: 1, minWidth: 300 }}>
              <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 6, fontWeight: 600 }}>
                DEPTH ACCURACY vs RANGE
              </div>
              <AccuracyChart config={config} />
            </div>

            <div style={{ background: panelBg, border: `1px solid ${borderColor}`, borderRadius: 8, padding: 10, flex: 1, minWidth: 300 }}>
              <div style={{ fontSize: 10, color: accentGold, letterSpacing: 1, marginBottom: 6, fontWeight: 600 }}>
                COVERAGE vs ROTATION
              </div>
              <CoverageChart coverageHistory={coverageHistory} />

              {/* Stats summary */}
              <div style={{
                display: "grid", gridTemplateColumns: "1fr 1fr 1fr", gap: 6, marginTop: 8,
                padding: 8, background: "rgba(255,215,0,0.04)", borderRadius: 4,
              }}>
                <div style={{ textAlign: "center" }}>
                  <div style={{ fontSize: 16, fontWeight: 700, color: "#00CC88" }}>{totalCoverage.toFixed(1)}%</div>
                  <div style={{ fontSize: 8, color: textDim }}>COVERED</div>
                </div>
                <div style={{ textAlign: "center" }}>
                  <div style={{ fontSize: 16, fontWeight: 700, color: "#FFD700" }}>{activeCameras.length}</div>
                  <div style={{ fontSize: 8, color: textDim }}>CAMERAS</div>
                </div>
                <div style={{ textAlign: "center" }}>
                  <div style={{ fontSize: 16, fontWeight: 700, color: "#FF6B6B" }}>{(100 - totalCoverage).toFixed(1)}%</div>
                  <div style={{ fontSize: 8, color: textDim }}>UNKNOWN</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
