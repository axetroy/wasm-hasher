/**
 * Usage:
 *
 * NODE_AUTH_TOKEN=npm_xxxxx node publish.js
 */

const fs = require("fs");
const path = require("path");
const spawn = require("child_process").spawn;

const ignoreCrates = ["lib"];
const crateFolder = path.join(__dirname, "crate");

const packages = [
  __dirname,
  ...fs
    .readdirSync(crateFolder)
    .filter((v) => !ignoreCrates.includes(v))
    .map((v) => path.join(crateFolder, v)),
].map((v) => path.join(v, "out"));

async function main() {
  for (const cwd of packages) {
    await new Promise((resolve, reject) => {
      const ps = spawn("npm", ["publish", "--access", "public"], {
        cwd: cwd,
        stdio: "inherit",
        env: process.env,
      });

      ps.on("close", (code) => {
        code === 0
          ? resolve()
          : reject(new Error(`Process exist with ${code}`));
      });
    });
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
