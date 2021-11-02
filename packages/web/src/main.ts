import App from './App.svelte'
import init from 'vite-wasm-functions'

const load = async () => {
  await init()

  const app = new App({
    target: document.getElementById('app')
  })
}

load()
