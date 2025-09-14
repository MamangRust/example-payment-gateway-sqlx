import { YearPicker } from "@/components/ui/yearpicker";
import { useEffect, useMemo, useState } from "react";
import { useToast } from "@/hooks/use-toast";
import { months, MonthPicker } from "@/components/ui/monthpicker";
import { useParams } from "react-router-dom";
import TableMerchantTransaction from "@/components/admin/merchant/table/transaction/table-merchant-transaction";
import useListTransactionByApiKey from "@/hooks/admin/merchant/ListTransactionByApikey";
import { useMonthAmountMerchantApiKey, useMonthPaymentMethodMerchantApiKey, useMonthTotalAmountMerchantApiKey, useYearAmountMerchantApiKey, useYearPaymentMethodMerchantApiKey, useYearTotalAmountMerchantApiKey } from "@/hooks/admin/card/dashboard/merchant/api_key";
import TotalCardMerchantApiKey from "@/components/admin/merchant/dashboard/apikey/total";
import ChartDashboarMerchantApiKey from "@/components/admin/merchant/dashboard/apikey/chart";

export default function MerchantDetailApiKey() {
  const initialYear = useMemo(() => {
    const currentDate = new Date();
    return currentDate.getFullYear();
  }, []);
  const params = useParams();

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

  const api_key = params.api_key;

  const {
    table: transactionTable,
    search: transactionSearch,
    setSearch: setTransactionSearch,
    loadingGetTransactions,
    pagination: transactionPagination,
    handlePageChange: handleTransactionPageChange,
    handlePageSizeChange: handleTransactionPageSizeChange,
    isLoadingWithDelay: isTransactionLoadingWithDelay,
  } = useListTransactionByApiKey({ api_key: api_key! });

  const { toast } = useToast();

  const {
    loadingMonthTotalAmount,
    currentMonthData,
    previousMonthData,
    debouncedFetchMonthlyTotalBalance,
    currentMonthBalance,
    monthPercentageChange
  } = useMonthTotalAmountMerchantApiKey(currentMonths, currentYear, previousMonth, previousYear, toast);

  const {
    loadingYearTotalAmount,
    currentYearData,
    previousYearData,
    currentYearBalance,
    debouncedFetchYearlyTotalBalance,
    yearPercentageChange,
  } = useYearTotalAmountMerchantApiKey(currentYear, toast);

  const {
    debouncedFetchMonthlyPaymentMethod,
    loadingMonthPaymentMethod,
    monthlyPaymentMethod,
  } = useMonthPaymentMethodMerchantApiKey(toast);

  const {
    debouncedFetchYearlyPaymentMethod,
    loadingYearPaymentMethod,
    yearlyPaymentMethod,
  } = useYearPaymentMethodMerchantApiKey(toast);

  const {
    debouncedFetchMonthlyAmount,
    loadingMonthAmount,
    monthlyAmount,
  } = useMonthAmountMerchantApiKey(toast);

  const {
    debouncedFetchYearlyAmount,
    loadingYearAmount,
    yearlyAmount,
  } = useYearAmountMerchantApiKey(toast);


  const handleYearChange = (newYear: number) => {
    setSelectedYear(newYear);
  };
  const handleMonthChange = (month: { name: string; number: number }) => {
    setSelectedMonth(month);
  };



  useEffect(() => {
    const fetchData = async () => {
      await Promise.all([
        debouncedFetchMonthlyPaymentMethod(selectedYear, api_key),
        debouncedFetchYearlyPaymentMethod(selectedYear, api_key),
        debouncedFetchMonthlyAmount(selectedYear, api_key),
        debouncedFetchYearlyAmount(selectedYear, api_key),
        debouncedFetchMonthlyTotalBalance(
          selectedYear,
          selectedMonth.number,
          api_key,
        ),
        debouncedFetchYearlyTotalBalance(selectedYear, api_key),
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
        <TotalCardMerchantApiKey
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
          <ChartDashboarMerchantApiKey
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
