import { YearPicker } from "@/components/ui/yearpicker";
import { useEffect, useMemo, useState } from "react";
import { useToast } from "@/hooks/use-toast";
import { months, MonthPicker } from "@/components/ui/monthpicker";
import { useParams } from "react-router-dom";
import useListTransactionByMerchant from "@/hooks/admin/merchant/ListTransactionByMerchant";
import TableMerchantTransaction from "@/components/admin/merchant/table/transaction/table-merchant-transaction";
import { useMonthAmountMerchantDetail, useMonthPaymentMethodMerchantDetail, useMonthTotalAmountMerchantDetail, useYearAmountMerchantDetail, useYearPaymentMethodMerchantDetail, useYearTotalAmountMerchantDetail } from "@/hooks/admin/card/dashboard/merchant/detail";
import TotalCardMerchantDetail from "@/components/admin/merchant/dashboard/detail/total";
import ChartDashboarMerchantDetail from "@/components/admin/merchant/dashboard/detail/chart";

export default function MerchantDetail() {
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

  const merchant_id = params.merchant_id;

  const {
    table: transactionTable,
    search: transactionSearch,
    setSearch: setTransactionSearch,
    loadingGetTransactions,
    pagination: transactionPagination,
    handlePageChange: handleTransactionPageChange,
    handlePageSizeChange: handleTransactionPageSizeChange,
    isLoadingWithDelay: isTransactionLoadingWithDelay,
  } = useListTransactionByMerchant({ merchant_id: Number(merchant_id!) });

  const { toast } = useToast();

  const {
    currentMonthData,
    previousMonthData,
    loadingMonthTotalAmount,
    debouncedFetchMonthlyTotalBalance,
    currentMonthBalance,
    monthPercentageChange
  } = useMonthTotalAmountMerchantDetail(currentMonths, currentYear, previousMonth, previousYear, toast);

  const {
    currentYearData,
    previousYearData,
    loadingYearTotalAmount,
    currentYearBalance,
    debouncedFetchYearlyTotalBalance,
    yearPercentageChange,
  } = useYearTotalAmountMerchantDetail(currentYear, toast);

  const {
    debouncedFetchMonthlyPaymentMethod,
    loadingMonthPaymentMethod,
    monthlyPaymentMethod,
  } = useMonthPaymentMethodMerchantDetail(toast);

  const {
    debouncedFetchYearlyPaymentMethod,
    loadingYearPaymentMethod,
    yearlyPaymentMethod,
  } = useYearPaymentMethodMerchantDetail(toast);

  const {
    debouncedFetchMonthlyAmount,
    loadingMonthAmount,
    monthlyAmount,
  } = useMonthAmountMerchantDetail(toast);

  const {
    debouncedFetchYearlyAmount,
    loadingYearAmount,
    yearlyAmount,
  } = useYearAmountMerchantDetail(toast);




  const handleYearChange = (newYear: number) => {
    setSelectedYear(newYear);
  };
  const handleMonthChange = (month: { name: string; number: number }) => {
    setSelectedMonth(month);
  };


  useEffect(() => {
    const fetchData = async () => {
      await Promise.all([
        debouncedFetchMonthlyPaymentMethod(selectedYear, merchant_id),
        debouncedFetchYearlyPaymentMethod(selectedYear, merchant_id),
        debouncedFetchMonthlyAmount(selectedYear, merchant_id),
        debouncedFetchYearlyAmount(selectedYear, merchant_id),
        debouncedFetchMonthlyTotalBalance(
          selectedYear,
          selectedMonth.number,
          merchant_id,
        ),
        debouncedFetchYearlyTotalBalance(selectedYear, merchant_id),
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
        <TotalCardMerchantDetail
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
          <ChartDashboarMerchantDetail
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
