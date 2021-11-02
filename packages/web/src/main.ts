import App from './App.svelte'
import init from 'vite-wasm-functions'

const load = async () => {
  const startTime = performance.now()
  await init()
  const endTime = performance.now()
  console.log(`Call to wasm init took ${endTime - startTime} milliseconds`)


  const app = new App({
    target: document.getElementById('app')
  })
}

load()
