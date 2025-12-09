import { TableTransaction } from "@/components/admin/transaction";
import useListTransaction from "@/hooks/admin/transaction/ListTransaction";
import { useEffect, useMemo, useState } from "react";
import { useToast } from "@/hooks/use-toast";
import { MonthPicker, months } from "@/components/ui/monthpicker";
import { YearPicker } from "@/components/ui/yearpicker";
import { useMonthAmountTransaction, useMonthPaymentMethodTransaction, useMonthStatusFailedTransaction, useMonthStatusSuccessTransaction, useYearAmountTransaction, useYearPaymentMethodTransaction, useYearStatusFailedTransaction, useYearStatusSuccessTransaction } from "@/hooks/admin/card/dashboard/transaction";
import TotalCardTransaction from "@/components/admin/transaction/dasboard/total";
import ChartDashboarTransaction from "@/components/admin/transaction/dasboard/chart";

export default function DashboardTransactions() {
  const initialYear = useMemo(() => {
    const currentDate = new Date();
    return currentDate.getFullYear();
  }, []);

  const { toast } = useToast();

  const currentMonthIndex = new Date().getMonth();
  const currentMonth = months[currentMonthIndex];
  const currentDate = new Date();
  const currentYear = currentDate.getFullYear();
  const currentMonths = currentDate.getMonth();

  const previousMonth = currentMonths === 0 ? 11 : currentMonths - 1;
  const previousYear = currentMonths === 0 ? currentYear - 1 : currentYear;


  const [selectedMonth, setSelectedMonth] = useState<{
    name: string;
    number: number;
  }>({
    name: currentMonth.value,
    number: currentMonth.number,
  });
  const [selectedYear, setSelectedYear] = useState<number>(initialYear);

  const {
    table,
    search,
    setSearch,
    loadingGetTransactions,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
  } = useListTransaction();

  const {
    monthPercentageSuccessChange,
    loadingMonthStatusSuccess,
    currentMonthSuccess,
    previousMonthSuccess,
    debouncedFetchMonthlySuccess
  } = useMonthStatusSuccessTransaction(currentMonths, currentYear, previousMonth, previousYear, toast);

  const {
    debouncedFetchYearlySuccess,
    loadingYearStatusSuccess,
    previousYearSuccessData,
    currentYearSuccess,
    yearPercentageSuccessChange,
  } = useYearStatusSuccessTransaction(currentYear, toast);

  const {
    monthPercentageFailedChange,
    loadingMonthStatusFailed,
    currentMonthFailed,
    previousMonthFailed,
    debouncedFetchMonthlyFailed
  } = useMonthStatusFailedTransaction(currentMonths, currentYear, previousMonth, previousYear, toast);

  const {
    debouncedFetchYearlyFailed,
    loadingYearStatusFailed,
    previousYearFailedData,
    currentYearFailed,
    yearPercentageFailedChange,
  } = useYearStatusFailedTransaction(currentYear, toast);

  const {
    loadingMonthTransactionAmount,
    monthlyAmount,
    debouncedFetchMonthlyAmount
  } = useMonthAmountTransaction(toast);

  const {
    loadingYearTransactionAmount,
    yearlyAmount,
    debouncedFetchYearlyAmount
  } = useYearAmountTransaction(toast);

  const {
    loadingMonthTransactionMethod,
    monthlyPaymentMethod,
    debouncedFetchMonthlyPaymentMethod
  } = useMonthPaymentMethodTransaction(toast);

  const {
    loadingYearTransactionMethod,
    yearlyPaymentMethod,
    debouncedFetchYearlyPaymentMethod
  } = useYearPaymentMethodTransaction(toast);


  const handleYearChange = (newYear: number) => {
    setSelectedYear(newYear);
  };

  const handleMonthChange = (month: { name: string; number: number }) => {
    setSelectedMonth(month);
  };


  useEffect(() => {
    const fetchData = async () => {
      await Promise.all([
        debouncedFetchMonthlySuccess(selectedYear, selectedMonth.number),
        debouncedFetchYearlySuccess(selectedYear),

        debouncedFetchMonthlyFailed(selectedYear, selectedMonth.number),
        debouncedFetchYearlyFailed(selectedYear),

        debouncedFetchMonthlyPaymentMethod(selectedYear),
        debouncedFetchYearlyPaymentMethod(selectedYear),
        debouncedFetchMonthlyAmount(selectedYear),
        debouncedFetchYearlyAmount(selectedYear),
      ]);
    };

    fetchData();

    return () => {
      debouncedFetchMonthlyFailed.cancel();
      debouncedFetchYearlyFailed.cancel();

      debouncedFetchMonthlySuccess.cancel();
      debouncedFetchYearlySuccess.cancel();

      debouncedFetchMonthlyPaymentMethod.cancel();
      debouncedFetchYearlyPaymentMethod.cancel();
      debouncedFetchMonthlyAmount.cancel();
      debouncedFetchYearlyAmount.cancel();
    };
  }, [selectedYear, selectedMonth]);

  return (
    <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
      <div className="grid grid-cols-1 gap-4 md:grid-cols-4 mt-10">
        <TotalCardTransaction
          loadingMonthStatusSuccess={loadingMonthStatusSuccess}
          currentMonthSuccess={currentMonthSuccess}
          previousMonthSuccess={previousMonthSuccess}
          monthPercentageSuccessChange={monthPercentageSuccessChange}
          loadingYearStatusSuccess={loadingYearStatusSuccess}
          currentYearSuccess={currentYearSuccess}
          previousYearSuccessData={previousYearSuccessData!}
          yearPercentageSuccessChange={yearPercentageSuccessChange}
          loadingMonthStatusFailed={loadingMonthStatusFailed}
          currentMonthFailed={currentMonthFailed}
          previousMonthFailed={previousMonthFailed}
          monthPercentageFailedChange={monthPercentageFailedChange}
          loadingYearStatusFailed={loadingYearStatusFailed}
          currentYearFailed={currentYearFailed}
          previousYearFailedData={previousYearFailedData!}
          yearPercentageFailedChange={yearPercentageFailedChange}
        />
      </div>

      <div className="flex-1 flex flex-col min-h-0 space-y-8 mt-4">
        <div className="flex justify-between">
          <div>
            <MonthPicker onMonthChange={handleMonthChange} />
          </div>

          <div>
            <YearPicker
              selectedYear={selectedYear}
              onYearChange={(date) => handleYearChange(date)}
            />
          </div>
        </div>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          <ChartDashboarTransaction
            loadingMonthTransactionAmount={loadingMonthTransactionAmount}
            loadingMonthTransactionMethod={loadingMonthTransactionMethod}
            loadingYearTransactionAmount={loadingYearTransactionAmount}
            loadingYearTransactionMethod={loadingYearTransactionMethod}
            monthlyAmount={monthlyAmount}
            monthlyPaymentMethod={monthlyPaymentMethod}
            yearlyAmount={yearlyAmount}
            yearlyPaymentMethod={yearlyPaymentMethod}
          />
        </div>
      </div>

      <TableTransaction
        search={search}
        setSearch={setSearch}
        isLoadingWithDelay={isLoadingWithDelay}
        loadingGetTransactions={loadingGetTransactions}
        table={table}
        pagination={pagination}
        handlePageChange={handlePageChange}
        handlePageSizeChange={handlePageSizeChange}
      />
    </div>
  );
}
