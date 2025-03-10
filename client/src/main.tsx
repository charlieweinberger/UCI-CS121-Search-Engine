import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
// import App from './App.tsx'
import App2 from './App.tsx' // TODO remove when done testing

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    {/* <App /> */}
    <App2 /> // TODO remove when done testing
  </StrictMode>,
)
