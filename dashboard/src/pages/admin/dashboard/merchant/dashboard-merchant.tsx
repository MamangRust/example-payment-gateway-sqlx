import { YearPicker } from "@/components/ui/yearpicker";
import useListMerchant from "@/hooks/admin/merchant/ListMerchant";
import { TableMerchant } from "@/components/admin/merchant";
import { useEffect, useMemo, useState } from "react";
import { useToast } from "@/hooks/use-toast";
import { months, MonthPicker } from "@/components/ui/monthpicker";
import useListTransactionMerchant from "@/hooks/admin/merchant/ListTransactionMerchant";
import TableMerchantTransaction from "@/components/admin/merchant/table/transaction/table-merchant-transaction";
import { useMonthAmountMerchant, useMonthPaymentMethodMerchant, useMonthTotalAmountMerchant, useYearAmountMerchant, useYearPaymentMethodMerchant, useYearTotalAmountMerchant } from "@/hooks/admin/card/dashboard/merchant";
import TotalCardMerchant from "@/components/admin/merchant/dashboard/total";
import ChartDashboarMerchant from "@/components/admin/merchant/dashboard/chart";

export default function DashboardMerchant() {
  const initialYear = useMemo(() => {
    const currentDate = new Date();
    return currentDate.getFullYear();
  }, []);
  const currentMonthIndex = new Date().getMonth();
  const currentMonth = months[currentMonthIndex];

  const [selectedMonth, setSelectedMonth] = useState<{
    name: string;
    number: number;
  }>({
    name: currentMonth.value,
    number: currentMonth.number,
  });

  const [selectedYear, setSelectedYear] = useState<number>(initialYear);

  const {
    table: merchantTable,
    search: merchantSearch,
    setSearch: setMerchantSearch,
    loadingGetMerchants,
    pagination: merchantPagination,
    handlePageChange: handleMerchantPageChange,
    handlePageSizeChange: handleMerchantPageSizeChange,
    isLoadingWithDelay: isMerchantLoadingWithDelay,
  } = useListMerchant();

  const {
    table: transactionTable,
    search: transactionSearch,
    setSearch: setTransactionSearch,
    loadingGetTransactions,
    pagination: transactionPagination,
    handlePageChange: handleTransactionPageChange,
    handlePageSizeChange: handleTransactionPageSizeChange,
    isLoadingWithDelay: isTransactionLoadingWithDelay,
  } = useListTransactionMerchant();
  const { toast } = useToast();

  const currentDate = new Date();
  const currentYear = currentDate.getFullYear();
  const currentMonths = currentDate.getMonth();

  const previousMonth = currentMonths === 0 ? 11 : currentMonths - 1;
  const previousYear = currentMonths === 0 ? currentYear - 1 : currentYear;

  const {
    currentMonthData,
    loadingMonthTotalAmount,
    previousMonthData,
    debouncedFetchMonthlyTotalBalance,
    currentMonthBalance,
    monthPercentageChange
  } = useMonthTotalAmountMerchant(currentMonths, currentYear, previousMonth, previousYear, toast);

  const {
    currentYearData,
    previousYearData,
    currentYearBalance,
    debouncedFetchYearlyTotalBalance,
    yearPercentageChange,
    loadingYearTotalAmount,
  } = useYearTotalAmountMerchant(currentYear, toast);

  const {
    debouncedFetchMonthlyPaymentMethod,
    loadingMonthPaymentMethod,
    monthlyPaymentMethod,
  } = useMonthPaymentMethodMerchant(toast);

  const {
    debouncedFetchYearlyPaymentMethod,
    loadingYearPaymentMethod,
    yearlyPaymentMethod,
  } = useYearPaymentMethodMerchant(toast);

  const {
    debouncedFetchMonthlyAmount,
    loadingMonthAmount,
    monthlyAmount,
  } = useMonthAmountMerchant(toast);

  const {
    debouncedFetchYearlyAmount,
    loadingYearAmount,
    yearlyAmount,
  } = useYearAmountMerchant(toast);


  const handleYearChange = (newYear: number) => {
    setSelectedYear(newYear);
  };
  const handleMonthChange = (month: { name: string; number: number }) => {
    setSelectedMonth(month);
  };


  useEffect(() => {
    const fetchData = async () => {
      await Promise.all([
        debouncedFetchMonthlyPaymentMethod(selectedYear),
        debouncedFetchYearlyPaymentMethod(selectedYear),
        debouncedFetchMonthlyAmount(selectedYear),
        debouncedFetchYearlyAmount(selectedYear),
        debouncedFetchMonthlyTotalBalance(selectedYear, selectedMonth.number),
        debouncedFetchYearlyTotalBalance(selectedYear),
      ]);
    };

    fetchData();

    return () => {
      debouncedFetchMonthlyPaymentMethod.cancel();
      debouncedFetchYearlyPaymentMethod.cancel();
      debouncedFetchMonthlyAmount.cancel();
      debouncedFetchYearlyAmount.cancel();
      debouncedFetchMonthlyTotalBalance.cancel();
      debouncedFetchYearlyTotalBalance.cancel();
    };
  }, [selectedYear, selectedMonth]);

  return (
    <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
      <div className="grid grid-cols-1 gap-4 md:grid-cols-2 mt-10">
        <TotalCardMerchant
          loadingMonthTotalAmount={loadingMonthTotalAmount}
          currentMonthData={currentMonthData!}
          currentMonthBalance={currentMonthBalance}
          previousMonthData={previousMonthData!}
          monthPercentageChange={monthPercentageChange}
          loadingYearTotalAmount={loadingYearTotalAmount}
          currentYearData={currentYearData!}
          currentYearBalance={currentYearBalance}
          previousYearData={previousYearData!}
          yearPercentageChange={yearPercentageChange}
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
          <ChartDashboarMerchant
            loadingMonthAmount={loadingMonthAmount}
            loadingMonthPaymentMethod={loadingMonthPaymentMethod}
            loadingYearAmount={loadingYearAmount}
            loadingYearPaymentMethod={loadingYearPaymentMethod}
            monthlyAmount={monthlyAmount}
            monthlyPaymentMethod={monthlyPaymentMethod}
            yearlyAmount={yearlyAmount}
            yearlyPaymentMethod={yearlyPaymentMethod}
          />
        </div>
      </div>

      <TableMerchant
        search={merchantSearch}
        setSearch={setMerchantSearch}
        isLoadingWithDelay={isMerchantLoadingWithDelay}
        loadingGetMerchants={loadingGetMerchants}
        table={merchantTable}
        pagination={merchantPagination}
        handlePageChange={handleMerchantPageChange}
        handlePageSizeChange={handleMerchantPageSizeChange}
      />

      <TableMerchantTransaction
        search={transactionSearch}
        setSearch={setTransactionSearch}
        isLoadingWithDelay={isTransactionLoadingWithDelay}
        loadingGetTransactions={loadingGetTransactions}
        table={transactionTable}
        pagination={transactionPagination}
        handlePageChange={handleTransactionPageChange}
        handlePageSizeChange={handleTransactionPageSizeChange}
      />
    </div>
  );
}
