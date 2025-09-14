import { TableTransfer } from "@/components/admin/transfer";
import useListTransfer from "@/hooks/admin/transfer/ListTransfer";
import { useEffect, useMemo, useState } from "react";
import { useToast } from "@/hooks/use-toast";
import { MonthPicker, months } from "@/components/ui/monthpicker";
import { YearPicker } from "@/components/ui/yearpicker";
import { useMonthAmountTransfer, useMonthStatusFailedTransfer, useMonthStatusSuccessTransfer, useYearAmountTransfer, useYearStatusFailedTransfer, useYearStatusSuccessTransfer } from "@/hooks/admin/card/dashboard/transfer";
import TotalCardTransfer from "@/components/admin/transfer/dasboard/total";
import ChartDashboarTransfer from "@/components/admin/transfer/dasboard/chart";

export default function DashboardTransfers() {
  const initialYear = useMemo(() => {
    const currentDate = new Date();
    return currentDate.getFullYear();
  }, []);

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
    loadingGetTransfers,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
  } = useListTransfer();
  const { toast } = useToast();


  const {
    monthPercentageSuccessChange,
    loadingMonthStatusSuccess,
    currentMonthSuccess,
    previousMonthSuccess,
    debouncedFetchMonthlySuccess
  } = useMonthStatusSuccessTransfer(currentMonths, currentYear, previousMonth, previousYear, toast);

  const {
    debouncedFetchYearlySuccess,
    loadingYearStatusSuccess,
    previousYearSuccessData,
    currentYearSuccess,
    yearPercentageSuccessChange,
  } = useYearStatusSuccessTransfer(currentYear, toast);

  const {
    monthPercentageFailedChange,
    loadingMonthStatusFailed,
    currentMonthFailed,
    previousMonthFailed,
    debouncedFetchMonthlyFailed
  } = useMonthStatusFailedTransfer(currentMonths, currentYear, previousMonth, previousYear, toast);

  const {
    debouncedFetchYearlyFailed,
    loadingYearStatusFailed,
    previousYearFailedData,
    currentYearFailed,
    yearPercentageFailedChange,
  } = useYearStatusFailedTransfer(currentYear, toast);

  const {
    loadingMonthTransferAmount,
    monthlyAmount,
    debouncedFetchMonthlyAmount
  } = useMonthAmountTransfer(toast);

  const {
    loadingYearTransferAmount,
    yearlyAmount,
    debouncedFetchYearlyAmount
  } = useYearAmountTransfer(toast);



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

      debouncedFetchMonthlyAmount.cancel();
      debouncedFetchYearlyAmount.cancel();
    };
  }, [selectedYear, selectedMonth]);

  return (
    <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
      <div className="grid grid-cols-1 gap-4 md:grid-cols-4 mt-10">
        <TotalCardTransfer
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
        <div className="grid grid-cols-1 gap-2 md:grid-cols-2">
          <ChartDashboarTransfer
            loadingMonthTransferAmount={loadingMonthTransferAmount}
            loadingYearTransferAmount={loadingYearTransferAmount}
            monthlyAmount={monthlyAmount}
            yearlyAmount={yearlyAmount}
          />
        </div>
      </div>
      <TableTransfer
        search={search}
        setSearch={setSearch}
        isLoadingWithDelay={isLoadingWithDelay}
        loadingGetTransfers={loadingGetTransfers}
        table={table}
        pagination={pagination}
        handlePageChange={handlePageChange}
        handlePageSizeChange={handlePageSizeChange}
      />
    </div>
  );
}
