import * as React from 'react';
import {
  ChevronRight,
  CreditCard,
  DollarSign,
  GalleryVerticalEnd,
  LayoutDashboard,
  Store,
  User,
  WalletIcon,
  ArrowLeftRight,
  ArrowRightCircle,
  ArrowDownCircle,
  Trash,
} from 'lucide-react';

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
  SidebarRail,
} from '@/components/ui/sidebar';
import { Collapsible } from '@radix-ui/react-collapsible';
import { CollapsibleContent, CollapsibleTrigger } from './ui/collapsible';
import { Link } from 'react-router-dom';

const data = {
  navMain: [
    {
      title: 'Dashboard',
      url: '/dashboard',
      isActive: true,
      icon: LayoutDashboard,
      items: [
        {
          title: 'Saldos',
          url: '/dashboard/saldo',
          icon: DollarSign,
        },
        {
          title: 'Cards',
          url: '/dashboard/card',
          icon: CreditCard,
        },
        {
          title: 'Merchants',
          url: '/dashboard/merchant',
          icon: Store,
        },
        {
          title: 'Topups',
          url: '/dashboard/topup',
          icon: WalletIcon,
        },
        {
          title: 'Transaction',
          url: '/dashboard/transaction',
          isActive: false,
          icon: ArrowLeftRight,
        },
        {
          title: 'Transfers',
          url: '/dashboard/transfer',
          isActive: false,
          icon: ArrowRightCircle,
        },
        {
          title: 'Withdraws',
          url: '/dashboard/withdraw',
          isActive: false,
          icon: ArrowDownCircle,
        },
      ],
    },
    {
      title: 'Cards',
      url: '#',
      isActive: true,
      icon: CreditCard,
      items: [
        {
          title: 'Cards',
          url: '/cards',
          icon: CreditCard,
        },
        {
          title: 'Trashed Cards',
          url: '/cards/trashed',
          icon: Trash,
        },
      ],
    },
    {
      title: 'Merchants',
      url: '/merchants',
      isActive: true,
      icon: Store,
      items: [
        {
          title: 'Merchants',
          url: '/merchants',
          icon: Store,
        },
        {
          title: 'Trashed Merchants',
          url: '/merchants/trashed',
          icon: Trash,
        },
      ],
    },
    {
      title: 'Saldos',
      url: '/saldos',
      isActive: true,
      icon: DollarSign,
      items: [
        {
          title: 'Saldos',
          url: '/saldos',
          icon: DollarSign,
        },
        {
          title: 'Trashed Saldos',
          url: '/saldos/trashed',
          icon: Trash,
        },
      ],
    },
    {
      title: 'Topups',
      url: '/topups',
      isActive: true,
      icon: WalletIcon,
      items: [
        {
          title: 'Topups',
          url: '/topups',
          icon: WalletIcon,
        },
        {
          title: 'Trashed Topups',
          url: '/topups/trashed',
          icon: Trash,
        },
      ],
    },
    {
      title: 'Transaction',
      url: '/transactions',
      isActive: true,
      icon: ArrowLeftRight,
      items: [
        {
          title: 'Transactions',
          url: '/transactions',
          icon: ArrowLeftRight,
        },
        {
          title: 'Trashed Transactions',
          url: '/transactions/trashed',
          icon: Trash,
        },
      ],
    },
    {
      title: 'Transfers',
      url: '/transfers',
      isActive: true,
      icon: ArrowRightCircle,
      items: [
        {
          title: 'Transfers',
          url: '/transfers',
          icon: ArrowRightCircle,
        },
        {
          title: 'Trashed Transfers',
          url: '/transfers/trashed',
          icon: Trash,
        },
      ],
    },
    {
      title: 'Users',
      url: '/users',
      isActive: true,
      icon: User,
      items: [
        {
          title: 'Users',
          url: '/users',
          icon: User,
        },
        {
          title: 'Trashed Users',
          url: '/users/trashed',
          icon: Trash,
        },
      ],
    },
    {
      title: 'Roles',
      url: '/roles',
      isActive: true,
      icon: User,
      items: [
        {
          title: 'Roles',
          url: '/roles',
          icon: User,
        },
        {
          title: 'Trashed Roles',
          url: '/roles/trashed',
          icon: Trash,
        },
      ],
    },
    {
      title: 'Withdraws',
      url: '/withdraws',
      isActive: true,
      icon: ArrowDownCircle,
      items: [
        {
          title: 'Withdraws',
          url: '/withdraws',
          icon: ArrowDownCircle,
        },
        {
          title: 'Trashed Withdraws',
          url: '/withdraws/trashed',
          icon: Trash,
        },
      ],
    },
  ],
};

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" asChild>
              <a href="#">
                <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                  <GalleryVerticalEnd className="size-4" />
                </div>
                <div className="flex flex-col gap-0.5 leading-none">
                  <span className="font-semibold">Payment Gateway SanEdge</span>
                </div>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        {data.navMain.map((item) => (
          <SidebarGroup key={item.title}>
            <SidebarGroupLabel>{item.title}</SidebarGroupLabel>
            <SidebarMenu>
              {item.isActive ? (
                <Collapsible
                  asChild
                  defaultOpen={item.isActive}
                  className="group/collapsible"
                >
                  <SidebarMenuItem>
                    <CollapsibleTrigger asChild>
                      <SidebarMenuButton tooltip={item.title}>
                        {item.icon && <item.icon />}
                        <span>{item.title}</span>
                        <ChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
                      </SidebarMenuButton>
                    </CollapsibleTrigger>
                    <CollapsibleContent>
                      <SidebarMenuSub>
                        {item.items?.map((subItem: any) => (
                          <SidebarMenuSubItem key={subItem.title}>
                            <SidebarMenuSubButton
                              asChild
                              isActive={subItem.isActive}
                            >
                              <Link
                                to={subItem.url}
                                className="flex items-center space-x-2"
                              >
                                {subItem.icon && (
                                  <subItem.icon className="w-4 h-4" />
                                )}{' '}
                              
                                <span>{subItem.title}</span>
                              </Link>
                            </SidebarMenuSubButton>
                          </SidebarMenuSubItem>
                        ))}
                      </SidebarMenuSub>
                    </CollapsibleContent>
                  </SidebarMenuItem>
                </Collapsible>
              ) : (
                <SidebarMenuItem>
                  <SidebarMenuButton tooltip={item.title} asChild>
                    <Link to={item.url}>
                      {item.icon && <item.icon />}
                      <span>{item.title}</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              )}
            </SidebarMenu>
          </SidebarGroup>
        ))}
      </SidebarContent>
      <SidebarRail />
    </Sidebar>
  );
}
