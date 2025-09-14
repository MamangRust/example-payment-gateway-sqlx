import { useEffect, useMemo, useState } from "react";
import { YearPicker } from "@/components/ui/yearpicker";
import { useToast } from "@/hooks/use-toast";
import CardSkeleton from "@/components/ui/cardSkeleton";
import { useParams } from "react-router-dom";
import { useAnalyticsDashboardCardNumber, useMonthBalanceCardNumber, useMonthTopupCardNumber, useMonthTransactionCardNumber, useMonthTransferReceiverCardNumber, useMonthTransferSenderCardNumber, useMonthWithdrawCardNumber, useYearBalanceCardNumber, useYearTopupCardNumber, useYearTransactionCardNumber, useYearTransferReceiverCardNumber, useYearTransferSenderCardNumber, useYearWithdrawCardNumber } from "@/hooks/admin/card/dashboard/card";
import { TotalCardDashboardByCardNumber } from "@/components/admin/card/dashboard/card_number";
import ChartDashboardByCardNumber from "@/components/admin/card/dashboard/card_number/chart";

export default function DashboardCardByCardNumber() {
  const { card_number } = useParams<{ card_number?: string }>();

  const initialYear = useMemo(() => {
    const currentDate = new Date();
    return currentDate.getFullYear();
  }, []);

  const [selectedYear, setSelectedYear] = useState<number>(initialYear);

  const { toast } = useToast();

  const {
    dashboardCardNumber,
    loadingDashboardCardNumber,
    debouncedFetchDashboardCard
  } = useAnalyticsDashboardCardNumber(card_number as string, toast)

  const {
    loadingMonthBalance,
    monthlyBalances,
    debouncedFetchMonthlyData,
  } = useMonthBalanceCardNumber(card_number as string, toast);

  const {
    loadingYearBalance,
    yearlyBalances,
    debouncedFetchYearlyData
  } = useYearBalanceCardNumber(card_number as string, toast);

  const {
    loadingMonthWithdrawAmount,
    monthlyWithdraws,
    debouncedFetchMonthlyWithdraws
  } = useMonthWithdrawCardNumber(card_number as string, toast);

  const {
    loadingYearWithdrawAmount,
    yearlyWithdraws,
    debouncedFetchYearlyWithdraws
  } = useYearWithdrawCardNumber(card_number as string, toast);

  const {
    loadingMonthTopupAmount,
    monthlyTopups,
    debouncedFetchMonthlyTopups
  } = useMonthTopupCardNumber(card_number as string, toast);

  const {
    loadingYearTopupAmount,
    yearlyTopups,
    debouncedFetchYearlyTopups
  } = useYearTopupCardNumber(card_number as string, toast);

  const {
    loadingMonthTransaction,
    monthlyTransactions,
    debouncedFetchMonthlyTransactions
  } = useMonthTransactionCardNumber(card_number as string, toast);

  const {
    loadingYearTransaction,
    yearlyTransactions,
    debouncedFetchYearlyTransactions
  } = useYearTransactionCardNumber(card_number as string, toast);

  const {
    loadingMonthTransferSender,
    monthlyTransfersSender,
    debouncedFetchMonthlyTransfersSender
  } = useMonthTransferSenderCardNumber(card_number as string, toast);

  const {
    loadingYearTransferSender,
    yearlyTransfersSender,
    debouncedFetchYearlyTransfersSender
  } = useYearTransferSenderCardNumber(card_number as string, toast);

  const {
    loadingMonthTransferReceiver,
    monthlyTransfersReceiver,
    debouncedFetchMonthlyTransfersReceiver
  } = useMonthTransferReceiverCardNumber(card_number as string, toast);

  const {
    loadingYearTransferReceiver,
    yearlyTransfersReceiver,
    debouncedFetchYearlyTransfersReceiver
  } = useYearTransferReceiverCardNumber(card_number as string, toast);


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
        {loadingDashboardCardNumber ? (
          <>
            <CardSkeleton />
            <CardSkeleton />
            <CardSkeleton />
            <CardSkeleton />
            <CardSkeleton />
            <CardSkeleton />
          </>
        ) : (
          <>
            <TotalCardDashboardByCardNumber dashboardCardNumber={dashboardCardNumber!} />
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
          <ChartDashboardByCardNumber
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
    </div>
  );
}
