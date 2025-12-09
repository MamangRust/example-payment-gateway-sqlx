import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useMonthBalanceCard(toast: any) {
    const {
        findMonthBalance,
        monthBalance,
        loadingMonthBalance,
        setLoadingMonthBalance,
        setErrorMonthBalance,
    } = useCardStore();

    const monthlyBalances = useMemo(() => {
        if (!monthBalance || !Array.isArray(monthBalance)) {
            return Array(12).fill(0);
        }

        const balances = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthBalance.forEach((balance) => {
            const monthIndex = new Date(`${balance.month} 1, ${year}`).getMonth();
            if (balance.total_balance !== undefined) {
                balances[monthIndex] = balance.total_balance;
            }
        });

        return balances;
    }, [monthBalance]);


    const fetchMonthlyData = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthBalance(true);
                setErrorMonthBalance(null);

                await findMonthBalance(toast, year);
            } catch (error) {
                console.error("Failed to fetch monthly balance:", error);
                setErrorMonthBalance("Failed to fetch monthly balance");
            } finally {
                setLoadingMonthBalance(false);
            }
        },
        [findMonthBalance, setLoadingMonthBalance, setErrorMonthBalance],
    );

    const debouncedFetchMonthlyData = debounce(fetchMonthlyData, 1000);

    return {
        findMonthBalance,
        monthBalance,
        loadingMonthBalance,
        setLoadingMonthBalance,
        setErrorMonthBalance,
        monthlyBalances,
        debouncedFetchMonthlyData,
    }
}