import { TableTransaction } from "@/components/admin/transaction";
import { useEffect, useMemo, useState } from "react";
import { useToast } from "@/hooks/use-toast";
import { MonthPicker, months } from "@/components/ui/monthpicker";
import { YearPicker } from "@/components/ui/yearpicker";
import { useParams } from "react-router-dom";
import useListTransactionCardNumber from "@/hooks/admin/transaction/ListTransactionCardNumber";
import { useMonthAmountTransactionDetail, useMonthPaymentMethodTransactionDetail, useMonthStatusFailedTransactionDetail, useMonthStatusSuccessTransactionDetail, useYearAmountTransactionDetail, useYearPaymentMethodTransactionDetail, useYearStatusFailedTransactionDetail, useYearStatusSuccessTransactionDetail } from "@/hooks/admin/card/dashboard/transaction/detail";
import TotalCardTransactionDetail from "@/components/admin/transaction/dasboard/detail/total";
import ChartDashboarTransactionDetail from "@/components/admin/transaction/dasboard/detail/chart";

export default function TransactionDetail() {
  const { toast } = useToast();
  const initialYear = useMemo(() => {
    const currentDate = new Date();
    return currentDate.getFullYear();
  }, []);

  const params = useParams();

  const card_number = params.card_number;

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
    loadingGetCardNumberTransaction,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
  } = useListTransactionCardNumber({ card_number: card_number! });

  const {
    monthPercentageSuccessChange,
    loadingMonthStatusSuccess,
    currentMonthSuccess,
    previousMonthSuccess,
    debouncedFetchMonthlySuccess
  } = useMonthStatusSuccessTransactionDetail(currentMonths, currentYear, previousMonth, previousYear, toast);

  const {
    debouncedFetchYearlySuccess,
    loadingYearStatusSuccess,
    previousYearSuccessData,
    currentYearSuccess,
    yearPercentageSuccessChange,
  } = useYearStatusSuccessTransactionDetail(currentYear, toast);

  const {
    monthPercentageFailedChange,
    loadingMonthStatusFailed,
    currentMonthFailed,
    previousMonthFailed,
    debouncedFetchMonthlyFailed
  } = useMonthStatusFailedTransactionDetail(currentMonths, currentYear, previousMonth, previousYear, toast);

  const {
    debouncedFetchYearlyFailed,
    loadingYearStatusFailed,
    previousYearFailedData,
    currentYearFailed,
    yearPercentageFailedChange,
  } = useYearStatusFailedTransactionDetail(currentYear, toast);

  const {
    loadingMonthTransactionAmount,
    monthlyAmount,
    debouncedFetchMonthlyAmount
  } = useMonthAmountTransactionDetail(toast);

  const {
    loadingYearTransactionAmount,
    yearlyAmount,
    debouncedFetchYearlyAmount
  } = useYearAmountTransactionDetail(toast);

  const {
    loadingMonthTransactionMethod,
    monthlyPaymentMethod,
    debouncedFetchMonthlyPaymentMethod
  } = useMonthPaymentMethodTransactionDetail(toast);

  const {
    loadingYearTransactionMethod,
    yearlyPaymentMethod,
    debouncedFetchYearlyPaymentMethod
  } = useYearPaymentMethodTransactionDetail(toast);


  const handleYearChange = (newYear: number) => {
    setSelectedYear(newYear);
  };

  const handleMonthChange = (month: { name: string; number: number }) => {
    setSelectedMonth(month);
  };


  useEffect(() => {
    const fetchData = async () => {
      await Promise.all([
        debouncedFetchMonthlySuccess(
          selectedYear,
          selectedMonth.number,
          card_number,
        ),
        debouncedFetchYearlySuccess(selectedYear, card_number),

        debouncedFetchMonthlyFailed(
          selectedYear,
          selectedMonth.number,
          card_number,
        ),
        debouncedFetchYearlyFailed(selectedYear, card_number),

        debouncedFetchMonthlyPaymentMethod(selectedYear, card_number),
        debouncedFetchYearlyPaymentMethod(selectedYear, card_number),
        debouncedFetchMonthlyAmount(selectedYear, card_number),
        debouncedFetchYearlyAmount(selectedYear, card_number),
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
        <TotalCardTransactionDetail
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
          <ChartDashboarTransactionDetail
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
        loadingGetTransactions={loadingGetCardNumberTransaction}
        table={table}
        pagination={pagination}
        handlePageChange={handlePageChange}
        handlePageSizeChange={handlePageSizeChange}
      />
    </div>
  );
}
