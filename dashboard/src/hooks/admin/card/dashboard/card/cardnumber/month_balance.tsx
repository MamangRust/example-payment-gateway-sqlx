import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useMonthBalanceCard(card_number: string,toast: any) {
    const {
        findMonthlyBalanceByCard,
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
    
            await findMonthlyBalanceByCard(toast, year, card_number as string);
          } catch (error) {
            console.error("Failed to fetch monthly balance:", error);
            setErrorMonthBalance("Failed to fetch monthly balance");
          } finally {
            setLoadingMonthBalance(false);
          }
        },
        [findMonthlyBalanceByCard, setLoadingMonthBalance, setErrorMonthBalance],
      );

    const debouncedFetchMonthlyData = debounce(fetchMonthlyData, 1000);

    return {
        fetchMonthlyData,
        monthBalance,
        loadingMonthBalance,
        setLoadingMonthBalance,
        setErrorMonthBalance,
        monthlyBalances,
        debouncedFetchMonthlyData,
    }
}