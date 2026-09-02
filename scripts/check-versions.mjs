/**
 * Ensure desktop release version strings stay aligned across manifests.
 * Extension version is tracked separately and not checked here.
 */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

function readJson(path) {
  return JSON.parse(readFileSync(join(root, path), "utf8"));
}

function readTomlVersion(path) {
  const text = readFileSync(join(root, path), "utf8");
  const match = text.match(/^version\s*=\s*"([^"]+)"/m);
  if (!match) throw new Error(`No version in ${path}`);
  return match[1];
}

const expected = readJson("package.json").version;
const targets = [
  { label: "apps/desktop/package.json", value: readJson("apps/desktop/package.json").version },
  {
    label: "apps/desktop/src-tauri/Cargo.toml",
    value: readTomlVersion("apps/desktop/src-tauri/Cargo.toml"),
  },
  {
    label: "apps/desktop/src-tauri/tauri.conf.json",
    value: readJson("apps/desktop/src-tauri/tauri.conf.json").version,
  },
];

const mismatches = targets.filter((t) => t.value !== expected);

if (mismatches.length) {
  console.error(`Expected desktop version ${expected}, but found:`);
  for (const item of mismatches) {
    console.error(`  - ${item.label}: ${item.value}`);
  }
  process.exit(1);
}

console.log(`version OK: ${expected} (root, desktop package, Cargo.toml, tauri.conf.json)`);
