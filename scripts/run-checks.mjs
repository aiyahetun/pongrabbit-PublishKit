/**
 * Unified self-check pipeline for local dev and CI.
 *
 * Usage:
 *   npm run check          # full gate (default)
 *   npm run check:quick    # skip production frontend bundle
 *   node scripts/run-checks.mjs --only i18n,versions,rust
 */
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const args = process.argv.slice(2);
const quick = args.includes("--quick");
const onlyArg = args.find((a) => a.startsWith("--only="));
const only = onlyArg ? new Set(onlyArg.slice("--only=".length).split(",")) : null;

/** @type {{ id: string, name: string, command: string, args: string[], cwd?: string, quick?: boolean }[]} */
const STEPS = [
  {
    id: "i18n",
    name: "i18n key parity (zh-CN ↔ en)",
    command: "npm",
    args: ["run", "check:i18n"],
  },
  {
    id: "versions",
    name: "desktop version alignment",
    command: "node",
    args: ["scripts/check-versions.mjs"],
  },
  {
    id: "types",
    name: "frontend TypeScript (vue-tsc)",
    command: "npm",
    args: ["run", "typecheck", "--workspace", "@publishkit/desktop"],
    quick: true,
  },
  {
    id: "frontend",
    name: "frontend production build (vue-tsc + vite)",
    command: "npm",
    args: ["run", "build", "--workspace", "@publishkit/desktop"],
  },
  {
    id: "rust",
    name: "Rust unit & integration tests (cargo test)",
    command: "cargo",
    args: ["test", "--manifest-path", "apps/desktop/src-tauri/Cargo.toml"],
  },
];

function shouldRun(step) {
  if (only && !only.has(step.id)) return false;
  if (quick && step.id === "frontend") return false;
  return true;
}

function runStep(step) {
  console.log(`\n▶ ${step.name}`);
  const started = Date.now();
  const cmd = [step.command, ...step.args].join(" ");
  const result = spawnSync(cmd, {
    cwd: step.cwd ?? root,
    stdio: "inherit",
    shell: true,
    env: process.env,
  });
  const seconds = ((Date.now() - started) / 1000).toFixed(1);
  if (result.status !== 0) {
    console.error(`\n✗ FAILED: ${step.name} (${seconds}s)`);
    return false;
  }
  console.log(`✓ passed (${seconds}s)`);
  return true;
}

console.log(quick ? "PublishKit self-check (quick)" : "PublishKit self-check (full)");
console.log(`Root: ${root}`);

const active = STEPS.filter(shouldRun);
if (!active.length) {
  console.error("No check steps selected.");
  process.exit(1);
}

const failed = [];
for (const step of active) {
  if (!runStep(step)) failed.push(step.name);
}

console.log("\n--- summary ---");
if (failed.length) {
  console.error(`FAILED (${failed.length}/${active.length}):`);
  for (const name of failed) console.error(`  - ${name}`);
  process.exit(1);
}

console.log(`All ${active.length} checks passed.`);
