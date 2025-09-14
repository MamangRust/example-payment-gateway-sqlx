import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import useSaldoStore from "@/store/saldo/saldo";
import { useCallback } from "react";


export default function useMonthTotalBalanceSaldo(currentMonths: number, currentYear: number, previousMonth: number, previousYear: number, toast: any) {
    const {
        monthTotalBalance,
        loadingMonthTotalBalance,
        findMonthTotalBalance,
        setLoadingMonthTotalBalance,
        setErrorMonthTotalBalance,
    } = useSaldoStore();



    const currentMonthData =
        monthTotalBalance && Array.isArray(monthTotalBalance)
            ? monthTotalBalance.find(
                (balance) =>
                    parseInt(balance.month) === currentMonths + 1 &&
                    parseInt(balance.year) === currentYear,
            )
            : null;

    const previousMonthData =
        monthTotalBalance && Array.isArray(monthTotalBalance)
            ? monthTotalBalance.find(
                (balance) =>
                    parseInt(balance.month) === previousMonth + 1 &&
                    parseInt(balance.year) === previousYear,
            )
            : null;

    const currentMonthBalance = currentMonthData?.total_balance || 0;
    const previousMonthBalance = previousMonthData?.total_balance || 0;

    const monthPercentageChange = calculatePercentageChange(
        currentMonthBalance,
        previousMonthBalance,
      );


    const fetchMonthlyTotalbalance = useCallback(
        async (year: number, month: number) => {
            try {
                setLoadingMonthTotalBalance(true);
                setErrorMonthTotalBalance(null);

                await findMonthTotalBalance(toast, year, month);
            } catch (error) {
                setErrorMonthTotalBalance("Failed to fetch monthly balance");
            } finally {
                setLoadingMonthTotalBalance(false);
            }
        },
        [findMonthTotalBalance, setErrorMonthTotalBalance, setLoadingMonthTotalBalance],
    );

    const debouncedFetchMonthlyTotalBalance = debounce(
        fetchMonthlyTotalbalance,
        300,
    );

    return {
        monthPercentageChange,
        loadingMonthTotalBalance,
        currentMonthData,
        previousMonthData,
        currentMonthBalance,
        previousMonthBalance,
        fetchMonthlyTotalbalance,
        debouncedFetchMonthlyTotalBalance
    }

}