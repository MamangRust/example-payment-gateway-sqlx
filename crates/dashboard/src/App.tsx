import { RouterProvider } from 'react-router-dom';
import { ThemeProvider } from './components/admin/theme-provider';
import router from './routes';
import { Toaster } from './components/ui/toaster';

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <RouterProvider router={router} />
      <Toaster />
    </ThemeProvider>
  );
}

export default App;
