import { useLocation } from 'react-router-dom';
import { Input } from '@/components/ui/input';
import { Search } from 'lucide-react';
import { ModeToggle } from './mode-toggle';
import { SidebarTrigger } from '../ui/sidebar';
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbPage,
} from '@/components/ui/breadcrumb';
import { Separator } from '../ui/separator';
import { CommandMenu } from './command';
import { AvatarDropdown } from './avatar-dropdown';
import { useState } from 'react';
import { NotificationMenu } from './notification-dropdown';

export function Header() {
  const location = useLocation();
  const pathname = location.pathname;
  const [isSearchOpen, setIsSearchOpen] = useState(false);

  const pageTitles: Record<string, string> = {
    '/dashboard': 'Dashboard',
    '/dashboard/card': 'Dashboard Card',
    '/dashboard/saldo': 'Dashboard Saldo',
    '/dashboard/merchant': 'Dashboard Merchant',
    '/dashboard/topup': 'Dashboard Topup',
    '/dashboard/transaction': 'Dashboard Transaction',
    '/dashboard/transfer': 'Dashboard Transfer',
    '/dashboard/withdraw': 'Dashboard Withdraw',

    '/users': 'User',
    '/user/detail': 'User Details',
    '/cards': 'Card',
    '/cards/detail': 'Card Details',
    '/saldos': 'Saldo',
    '/saldos/detail': 'Saldo Details',
    '/merchants': 'Merchant',
    '/merchants/detail': 'Merchant Details',
    '/topups': 'Topup',
    '/topups/detail': 'Topup Details',
    '/transactions': 'Transaction',
    '/transactions/detail': 'Transaction Details',
    '/transfers': 'Transfer',
    '/transfers/detail': 'Transfer Details',
    '/withdraws': 'Withdraw',
    '/withdraws/detail': 'Withdraw Details',

    '/profile': 'Profile',
  };

  let currentTitle = 'Page';
  if (pageTitles[pathname]) {
    currentTitle = pageTitles[pathname];
  } else if (pathname.startsWith('/user/detail')) {
    currentTitle = 'User Details';
  } else if (pathname.startsWith('/transactions/detail')) {
    currentTitle = 'Transaction Details';
  }

  return (
    <header className="flex h-16 shrink-0 items-center justify-between gap-2 border-b px-4">
      <div className="flex items-center gap-2">
        <SidebarTrigger />
        <Separator orientation="vertical" className="h-6" />
        <Breadcrumb>
          <BreadcrumbList>
            <BreadcrumbItem>
              <BreadcrumbPage>{currentTitle}</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
      </div>

      <div className="flex items-center gap-4">
        <div className="relative">
          <Input
            type="text"
            placeholder="Search..."
            className="w-48 cursor-pointer"
            readOnly
            onClick={() => setIsSearchOpen(true)}
          />
          <Search className="absolute right-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
        </div>

        <CommandMenu open={isSearchOpen} setOpen={setIsSearchOpen} />
        <ModeToggle />
        <NotificationMenu />
        <AvatarDropdown />
      </div>
    </header>
  );
}
