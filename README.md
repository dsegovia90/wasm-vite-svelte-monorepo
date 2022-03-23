# WASM (with Rust) + Vite + Svelte Monorepo
## Quick Start
### Unix

Assuming a fresh install (no node, no rust)

1. Clone the repo.
2. `cd` into repo
3. Install [nvm](https://github.com/nvm-sh/nvm#installing-and-updating), close and reopen terminal after install.
4. Run
```bash
nvm install --lts
```
5. To verify install run
```bash
node -v
// should output something similar to
v16.13.0
```
6. This monorepo uses [yarn workspaces](https://yarnpkg.com/features/workspaces) under the hood. Install `yarn` by running
```bash
npm i -g yarn
```
7. Install [rustup + rust](https://www.rust-lang.org/tools/install)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
// follow installation prompts, close and reopen terminal after install.
```
8. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
9. If in Ubuntu or similar, you need to install a CC linker (you might already have it if you've run sudo apt-get update before), run
```bash
sudo apt-get update
sudo apt install build-essential

```
10. Yarn needs rust to be built at least once so it can cross reference dependencies in the monorepo. Run in the `packages/rust` directory
```bash
cd packages/rust
# build WebAssembly and Javascript wrappers using wasm-pack
# don't panic, this might take a few seconds
wasm-pack build --target web
# Then initialize yarn in the rust directory
yarn
# go back to the monorepo root directory
cd ../..
```
11. Install node dependencies, run
```bash
yarn
```
12. Install [cargo watch](https://crates.io/crates/cargo-watch)
```bash
cargo install cargo-watch
```
13. To start the development server, run
```
yarn dev
```
14. Enjoy! Got some feedback? Open an issue, or better yet, a PR. If you like this template, please star this repo.

## What's next
This needs to work with a regular CI (ie: vercel/netlify/github pages). Will create a guide for this if there is enough demand for it.

If this gets enough attention, and there demand for it, I will create a template for sveltekit, and potentially for vue and react.
