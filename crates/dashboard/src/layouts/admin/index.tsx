import { Header } from '@/components/admin/header';
import ModalAdmin from '@/components/admin/modal';
import { AppSidebar } from '@/components/app-sidebar';
import SplashScreen from '@/components/splash';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';
import AuthProvider from '@/provider/AuthProvider';
import { useEffect, useState } from 'react';
import { Outlet, useNavigation } from 'react-router-dom';

function LayoutAdmin() {
  const [isLoading, setIsLoading] = useState(true);
  const navigation = useNavigation();

  useEffect(() => {
    if (navigation.state === 'loading') {
      setIsLoading(true);
    } else {
      const timer = setTimeout(() => setIsLoading(false), 1500);

      return () => clearTimeout(timer);
    }
  }, [navigation.state]);

  return (
    <AuthProvider>
      {isLoading ? (
        <SplashScreen />
      ) : (
        <SidebarProvider>
          <AppSidebar />
          <SidebarInset>
            <Header />
            <br />
            <ModalAdmin />
            <Outlet />
          </SidebarInset>
        </SidebarProvider>
      )}
    </AuthProvider>
  );
}

export default LayoutAdmin;
