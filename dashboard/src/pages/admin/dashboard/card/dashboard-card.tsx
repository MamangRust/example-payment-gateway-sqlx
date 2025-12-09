import { useEffect, useMemo, useState } from "react";
import TableCard from "@/components/admin/card/table/table-card";
import useListCard from "@/hooks/admin/card/ListCard";
import { YearPicker } from "@/components/ui/yearpicker";
import { useToast } from "@/hooks/use-toast";
import CardSkeleton from "@/components/ui/cardSkeleton";
import { useAnalyticsDashboardCard, useMonthBalanceCard, useMonthTopupCard, useMonthTransactionCard, useMonthTransferReceiver, useMonthTransferSender, useMonthWithdrawCard, useYearBalanceCard, useYearTopupCard, useYearTransactionCard, useYearTransferReceiver, useYearTransferSender, useYearWithdrawCard } from "@/hooks/admin/card/dashboard/card";
import ChartDashboardCard from "@/components/admin/card/dashboard/chart";
import TotalCardDashboard from "@/components/admin/card/dashboard/total";

export default function DashboardCard() {
  const initialYear = useMemo(() => {
    const currentDate = new Date();
    return currentDate.getFullYear();
  }, []);

  const [selectedYear, setSelectedYear] = useState<number>(initialYear);

  const {
    table,
    search,
    setSearch,
    loadingGetCards,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
  } = useListCard();

  const { toast } = useToast();

  const {
    dashboard,
    loadingDashboard,
    debouncedFetchDashboardCard
  } = useAnalyticsDashboardCard(toast)

  const {
    loadingMonthBalance,
    monthlyBalances,
    debouncedFetchMonthlyData,
  } = useMonthBalanceCard(toast);

  const {
    loadingYearBalance,
    yearlyBalances,
    debouncedFetchYearlyData
  } = useYearBalanceCard(toast);

  const {
    loadingMonthWithdrawAmount,
    monthlyWithdraws,
    debouncedFetchMonthlyWithdraws
  } = useMonthWithdrawCard(toast);

  const {
    loadingYearWithdrawAmount,
    yearlyWithdraws,
    debouncedFetchYearlyWithdraws
  } = useYearWithdrawCard(toast);

  const {
    loadingMonthTopupAmount,
    monthlyTopups,
    debouncedFetchMonthlyTopups
  } = useMonthTopupCard(toast);

  const {
    loadingYearTopupAmount,
    yearlyTopups,
    debouncedFetchYearlyTopups
  } = useYearTopupCard(toast);

  const {
    loadingMonthTransaction,
    monthlyTransactions,
    debouncedFetchMonthlyTransactions
  } = useMonthTransactionCard(toast);

  const {
    loadingYearTransaction,
    yearlyTransactions,
    debouncedFetchYearlyTransactions
  } = useYearTransactionCard(toast);

  const {
    loadingMonthTransferSender,
    monthlyTransfersSender,
    debouncedFetchMonthlyTransfersSender
  } = useMonthTransferSender(toast);

  const {
    loadingYearTransferSender,
    yearlyTransfersSender,
    debouncedFetchYearlyTransfersSender
  } = useYearTransferSender(toast);

  const {
    loadingMonthTransferReceiver,
    monthlyTransfersReceiver,
    debouncedFetchMonthlyTransfersReceiver
  } = useMonthTransferReceiver(toast);

  const {
    loadingYearTransferReceiver,
    yearlyTransfersReceiver,
    debouncedFetchYearlyTransfersReceiver
  } = useYearTransferReceiver(toast);


  const handleYearChange = (newYear: number) => {
    setSelectedYear(newYear);
  };


  useEffect(() => {
    const fetchData = async () => {
      await Promise.all([
        debouncedFetchDashboardCard(),
        debouncedFetchMonthlyData(selectedYear),
        debouncedFetchYearlyData(selectedYear),
        debouncedFetchMonthlyWithdraws(selectedYear),
        debouncedFetchYearlyWithdraws(selectedYear),
        debouncedFetchMonthlyTopups(selectedYear),
        debouncedFetchYearlyTopups(selectedYear),
        debouncedFetchMonthlyTransactions(selectedYear),
        debouncedFetchYearlyTransactions(selectedYear),
        debouncedFetchMonthlyTransfersSender(selectedYear),
        debouncedFetchYearlyTransfersSender(selectedYear),
        debouncedFetchMonthlyTransfersReceiver(selectedYear),
        debouncedFetchYearlyTransfersReceiver(selectedYear),
      ]);
    };

    fetchData();

    return () => {
      debouncedFetchDashboardCard.cancel();
      debouncedFetchMonthlyData.cancel();
      debouncedFetchYearlyData.cancel();
      debouncedFetchMonthlyWithdraws.cancel();
      debouncedFetchYearlyWithdraws.cancel();
      debouncedFetchMonthlyTopups.cancel();
      debouncedFetchYearlyTopups.cancel();
      debouncedFetchMonthlyTransactions.cancel();
      debouncedFetchYearlyTransactions.cancel();
      debouncedFetchMonthlyTransfersSender.cancel();
      debouncedFetchYearlyTransfersSender.cancel();
      debouncedFetchMonthlyTransfersReceiver.cancel();
      debouncedFetchYearlyTransfersReceiver.cancel();
    };
  }, [selectedYear]);

  return (
    <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
      <div className="grid grid-cols-1 gap-4 md:grid-cols-3 mt-10">
        {loadingDashboard ? (
          <>
            <CardSkeleton />
            <CardSkeleton />
            <CardSkeleton />
            <CardSkeleton />
            <CardSkeleton />
          </>
        ) : (
          <>
          <TotalCardDashboard dashboardCardNumber={dashboard!}  />
          </>
        )}
      </div>
      <div className="flex-1 flex flex-col min-h-0 space-y-8 mt-4">
        <div className="flex justify-between">
          <div>
            <YearPicker
              selectedYear={selectedYear}
              onYearChange={(date) => handleYearChange(date)}
            />
          </div>
        </div>
        <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
          <ChartDashboardCard
            loadingMonthBalance={loadingMonthBalance}
            loadingYearBalance={loadingYearBalance}
            loadingMonthWithdrawAmount={loadingMonthWithdrawAmount}
            loadingYearWithdrawAmount={loadingYearWithdrawAmount}
            loadingMonthTopupAmount={loadingMonthTopupAmount}
            loadingYearTopupAmount={loadingYearTopupAmount}
            loadingMonthTransaction={loadingMonthTransaction}
            loadingYearTransaction={loadingYearTransaction}
            loadingMonthTransferSender={loadingMonthTransferSender}
            loadingYearTransferSender={loadingYearTransferSender}
            loadingMonthTransferReceiver={loadingMonthTransferReceiver}
            loadingYearTransferReceiver={loadingYearTransferReceiver}
            monthlyBalances={monthlyBalances}
            yearlyBalances={yearlyBalances}
            monthlyWithdraws={monthlyWithdraws}
            yearlyWithdraws={yearlyWithdraws}
            monthlyTopups={monthlyTopups}
            yearlyTopups={yearlyTopups}
            monthlyTransactions={monthlyTransactions}
            yearlyTransactions={yearlyTransactions}
            monthlyTransfersSender={monthlyTransfersSender}
            yearlyTransfersSender={yearlyTransfersSender}
            monthlyTransfersReceiver={monthlyTransfersReceiver}
            yearlyTransfersReceiver={yearlyTransfersReceiver}
          />
        </div>
      </div>
      <TableCard
        search={search}
        setSearch={setSearch}
        isLoadingWithDelay={isLoadingWithDelay}
        loadingGetCards={loadingGetCards}
        table={table}
        pagination={pagination}
        handlePageChange={handlePageChange}
        handlePageSizeChange={handlePageSizeChange}
      />
    </div>
  );
}
