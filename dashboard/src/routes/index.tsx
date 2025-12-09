import LayoutAdmin from "@/layouts/admin";
import ErrorPage from "@/pages/error";
import { createBrowserRouter, Navigate } from "react-router-dom";
import AuthLayout from "@/layouts/auth";
import LoginPage from "@/pages/auth/login";
import CardDashboard from "@/pages/admin/dashboard/card/dashboard-card";
import DashboardSaldo from "@/pages/admin/dashboard/saldo/dashboard-saldo";
import DashboardMerchant from "@/pages/admin/dashboard/merchant/dashboard-merchant";
import DashboardTopups from "@/pages/admin/dashboard/topup/dashboard-topup";
import DashboardTransactions from "@/pages/admin/dashboard/transaction/dashboard-transaction";
import DashboardTransfers from "@/pages/admin/dashboard/transfer/dashboard-transfer";
import DashboardWithdraws from "@/pages/admin/dashboard/withdraw/dashboard-withdraw";
import ProfilePage from "@/pages/admin/user-profile";
import SaldoPage from "@/pages/admin/saldo/saldo";
import CardPage from "@/pages/admin/card/card";
import MerchantPage from "@/pages/admin/merchant/merchant";
import TopupPage from "@/pages/admin/topup/topup";
import TransactionPage from "@/pages/admin/transaction/transaction";
import WithdrawPage from "@/pages/admin/withdraw/withdraw";
import UserPage from "@/pages/admin/user/user";
import TransferPage from "@/pages/admin/transfer/transfer";
import RegisterPage from "@/pages/auth/register";
import CardTrashedPage from "@/pages/admin/card/cardTrashed";
import MerchantTrashedPage from "@/pages/admin/merchant/merchantTrashed";
import SaldoTrashedPage from "@/pages/admin/saldo/saldoTrashed";
import TopupTrashedPage from "@/pages/admin/topup/topupTrashed";
import TransactionTrashedPage from "@/pages/admin/transaction/transactionTrashed";
import TransferTrashedPage from "@/pages/admin/transfer/tansferTrashed";
import UserTrashedPage from "@/pages/admin/user/userTrashed";
import WithdrawTrashedPage from "@/pages/admin/withdraw/withdrawTrashed";
import DashboardCardByCardNumber from "@/pages/admin/dashboard/card/dashboard-card-bycardnumber";
import RolePage from "@/pages/admin/role/role";
import RoleTrashedPage from "@/pages/admin/role/roleTrashed";
import CardDetail from "@/pages/admin/card/cardDetail";
import MerchantDetail from "@/pages/admin/merchant/merchantDetail";
import MerchantDetailApiKey from "@/pages/admin/merchant/merchantApiKey";
import TopupDetail from "@/pages/admin/topup/topupDetail";
import TransactionDetail from "@/pages/admin/transaction/transactionDetail";
import TransferDetail from "@/pages/admin/transfer/transferDetail";
import WithdrawDetail from "@/pages/admin/withdraw/withdrawDetail";

const router = createBrowserRouter([
  {
    path: "/dashboard",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "card",
        element: <CardDashboard />,
        errorElement: <ErrorPage />,
      },
      {
        path: "merchant",
        element: <DashboardMerchant />,
        errorElement: <ErrorPage />,
      },
      {
        path: "saldo",
        element: <DashboardSaldo />,
        errorElement: <ErrorPage />,
      },
      {
        path: "topup",
        element: <DashboardTopups />,
        errorElement: <ErrorPage />,
      },
      {
        path: "transaction",
        element: <DashboardTransactions />,
        errorElement: <ErrorPage />,
      },
      {
        path: "transfer",
        element: <DashboardTransfers />,
        errorElement: <ErrorPage />,
      },
      {
        path: "withdraw",
        element: <DashboardWithdraws />,
        errorElement: <ErrorPage />,
      },
    ],
  },
  {
    path: "/dashboard/card/:card_number",
    element: <LayoutAdmin />,
    children: [
      {
        index: true,
        element: <DashboardCardByCardNumber />,
      },
    ],
    errorElement: <ErrorPage />,
  },

  {
    path: "/cards",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <CardPage />,
      },
      {
        path: "detail/:card_number",
        element: <CardDetail />,
      },
      {
        path: "trashed",
        element: <CardTrashedPage />,
      },
    ],
  },
  {
    path: "/merchants",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <MerchantPage />,
      },
      {
        path: "detail/:merchant_id",
        element: <MerchantDetail />,
      },
      {
        path: "detail-key/:api_key",
        element: <MerchantDetailApiKey />,
      },
      {
        path: "trashed",
        element: <MerchantTrashedPage />,
      },
    ],
  },
  {
    path: "/saldos",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <SaldoPage />,
      },
      {
        path: "trashed",
        element: <SaldoTrashedPage />,
      },
    ],
  },
  {
    path: "/topups",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <TopupPage />,
      },
      {
        path: "detail/:card_number",
        element: <TopupDetail />,
      },
      {
        path: "trashed",
        element: <TopupTrashedPage />,
      },
    ],
  },
  {
    path: "/transactions",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <TransactionPage />,
      },
      {
        path: "detail/:card_number",
        element: <TransactionDetail />,
      },
      {
        path: "trashed",
        element: <TransactionTrashedPage />,
      },
    ],
  },
  {
    path: "/transfers",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <TransferPage />,
      },
      {
        path: "detail/:card_number",
        element: <TransferDetail />,
      },
      {
        path: "trashed",
        element: <TransferTrashedPage />,
      },
    ],
  },
  {
    path: "/users",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <UserPage />,
        errorElement: <ErrorPage />,
      },
      {
        path: "trashed",
        element: <UserTrashedPage />,
      },
      {
        path: "*",
        element: <ErrorPage />,
      },
    ],
  },

  {
    path: "/roles",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <RolePage />,
        errorElement: <ErrorPage />,
      },
      {
        path: "trashed",
        element: <RoleTrashedPage />,
      },
      {
        path: "*",
        element: <ErrorPage />,
      },
    ],
  },
  {
    path: "/withdraws",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <WithdrawPage />,
      },
      {
        path: "detail/:card_number",
        element: <WithdrawDetail />,
      },
      {
        path: "trashed",
        element: <WithdrawTrashedPage />,
      },
    ],
  },
  {
    path: "/profile",
    element: <LayoutAdmin />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <ProfilePage />,
      },
    ],
  },
  {
    path: "/",
    element: <Navigate to={"/auth/login"} />,
  },
  {
    path: "/auth",
    element: <AuthLayout />,
    children: [
      {
        path: "register",
        element: <RegisterPage />,
      },
      {
        path: "login",
        element: <LoginPage />,
      },
    ],
  },
]);

export default router;
