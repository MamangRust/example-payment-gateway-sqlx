import { TableSaldo } from "@/components/admin/saldo";
import useListSaldo from "@/hooks/admin/saldo/ListSaldo";
import { useEffect, useMemo, useState } from "react";
import { useToast } from "@/hooks/use-toast";
import { YearPicker } from "@/components/ui/yearpicker";
import { months, MonthPicker } from "@/components/ui/monthpicker";
import { useMonthBalanceSaldo, useMonthTotalBalanceSaldo, useYearBalanceSaldo, useYearTotalBalance } from "@/hooks/admin/card/dashboard/saldo";
import ChartDashboarSaldo from "@/components/admin/saldo/dashboard/chart";
import TotalCardSaldo from "@/components/admin/saldo/dashboard/total";

export default function DashboardSaldo() {
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
    table,
    search,
    setSearch,
    loadingGetSaldos,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
  } = useListSaldo();


  const { toast } = useToast();

  const handleYearChange = (newYear: number) => {
    setSelectedYear(newYear);
  };
  const handleMonthChange = (month: { name: string; number: number }) => {
    setSelectedMonth(month);
  };

  const currentDate = new Date();
  const currentYear = currentDate.getFullYear();
  const currentMonths = currentDate.getMonth();

  const previousMonth = currentMonths === 0 ? 11 : currentMonths - 1;
  const previousYear = currentMonths === 0 ? currentYear - 1 : currentYear;


  const {
    loadingMonthBalance,
    monthlyBalances,
    debouncedFetchMonthlyData
  } = useMonthBalanceSaldo(toast);

  const {
    yearlyBalances,
    loadingYearBalance,
    debouncedFetchYearlyData
  } = useYearBalanceSaldo(toast);

  const {
    monthPercentageChange,
    loadingMonthTotalBalance,
    currentMonthData,
    previousMonthData,
    currentMonthBalance,
    debouncedFetchMonthlyTotalBalance
  } = useMonthTotalBalanceSaldo(currentMonths, currentYear, previousMonth, previousYear, toast)

  const {
    loadingYearTotalBalance,
    currentYearData,
    previousYearData,
    currentYearBalance,
    yearPercentageChange,
    debouncedFetchYearlyTotalBalance
  } = useYearTotalBalance(currentYear, toast);


  useEffect(() => {
    const fetchData = async () => {
      await Promise.all([
        debouncedFetchMonthlyData(selectedYear),
        debouncedFetchYearlyData(selectedYear),
        debouncedFetchMonthlyTotalBalance(selectedYear, selectedMonth.number),
        debouncedFetchYearlyTotalBalance(selectedYear),
      ]);
    };

    fetchData();

    return () => {
      debouncedFetchMonthlyData.cancel();
      debouncedFetchYearlyData.cancel();
      debouncedFetchMonthlyTotalBalance.cancel();
      debouncedFetchYearlyTotalBalance.cancel();
    };
  }, [selectedYear, selectedMonth]);

  return (
    <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
      <div className="grid grid-cols-1 gap-4 md:grid-cols-2 mt-10">
        <TotalCardSaldo
          loadingMonthTotalBalance={loadingMonthTotalBalance}
          currentMonthData={currentMonthData!}
          currentMonthBalance={currentMonthBalance}
          previousMonthData={previousMonthData!}
          monthPercentageChange={monthPercentageChange}
          loadingYearTotalBalance={loadingYearTotalBalance}
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
        <div className="grid grid-cols-1 gap-2 md:grid-cols-2">
          <ChartDashboarSaldo
            loadingMonthBalance={loadingMonthBalance}
            loadingYearBalance={loadingYearBalance}
            monthlyBalances={monthlyBalances}
            yearlyBalances={yearlyBalances}
          />
        </div>
      </div>

      <TableSaldo
        search={search}
        setSearch={setSearch}
        isLoadingWithDelay={isLoadingWithDelay}
        loadingGetSaldos={loadingGetSaldos}
        table={table}
        pagination={pagination}
        handlePageChange={handlePageChange}
        handlePageSizeChange={handlePageSizeChange}
      />
    </div>
  );
}
