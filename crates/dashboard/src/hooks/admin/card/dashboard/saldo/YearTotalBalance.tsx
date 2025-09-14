import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import useSaldoStore from "@/store/saldo/saldo";
import { useCallback } from "react";


export default function useYearTotalBalance(currentYear: number,toast: any) {
    const {
        yearTotalBalance,
        loadingYearTotalBalance,
        findYearTotalBalance,
        setLoadingYearTotalBalance,
        setErrorYearTotalBalance,
    } = useSaldoStore();

    const currentYearData =
        yearTotalBalance && Array.isArray(yearTotalBalance)
            ? yearTotalBalance.find(
                (balance) => balance.year === currentYear.toString(),
            )
            : null;

    const previousYearData =
        yearTotalBalance && Array.isArray(yearTotalBalance)
            ? yearTotalBalance.find(
                (balance) => balance.year === (currentYear - 1).toString(),
            )
            : null;

    const currentYearBalance = currentYearData?.total_balance || 0;
    const previousYearBalance = previousYearData?.total_balance || 0;

    const yearPercentageChange = calculatePercentageChange(
        currentYearBalance,
        previousYearBalance,
    );

    const fetchYearlyTotalBalance = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTotalBalance(true);
                setErrorYearTotalBalance(null);

                await findYearTotalBalance(toast, year);
            } catch (error) {
                setErrorYearTotalBalance("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTotalBalance(false);
            }
        },
        [findYearTotalBalance, setLoadingYearTotalBalance, setErrorYearTotalBalance],
    );

    const debouncedFetchYearlyTotalBalance = debounce(
        fetchYearlyTotalBalance,
            300,
        );

    return {
        debouncedFetchYearlyTotalBalance,
        loadingYearTotalBalance,
        currentYearData,
        previousYearData,
        currentYearBalance,
        previousYearBalance,
        yearPercentageChange,
        fetchYearlyTotalBalance
    }

}