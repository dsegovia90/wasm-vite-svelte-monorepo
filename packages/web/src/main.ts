import App from './App.svelte'
import init, { greet } from 'vite-wasm-functions'

init().then(() => {
  greet('Daniel')
})

const app = new App({
  target: document.getElementById('app')
})

export default app
