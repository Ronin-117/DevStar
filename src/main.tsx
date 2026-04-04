import React, { useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';

function Root() {
  useEffect(() => {
    const w = window as unknown as { __TAURI__?: { window: { getCurrent: () => { label: string } } } };
    if (w.__TAURI__?.window?.getCurrent()?.label === 'active') {
      document.documentElement.classList.add('active-window');
      document.body.style.background = '#ffffff';
      document.getElementById('root')!.style.background = '#ffffff';
    }
  }, []);
  return <App />;
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Root />
  </React.StrictMode>,
);
