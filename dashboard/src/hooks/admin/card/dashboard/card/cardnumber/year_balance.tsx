import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";

export default function useYearBalanceCard(card_number: string,toast: any) {
    const {
        findYearlyBalanceByCard,
        yearBalance,
        loadingYearBalance,
        setLoadingYearBalance,
        setErrorYearBalance,
    } = useCardStore();

    const yearlyBalances = useMemo(() => {
        if (!yearBalance || !Array.isArray(yearBalance)) {
            return Array(5).fill(0);
        }

        const balanceMap = new Map<number, number>();
        yearBalance.forEach((balance) => {
            balanceMap.set(Number(balance.year), balance.total_balance);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        const result = years.map((year) => balanceMap.get(year) || 0);

        return result;
    }, [yearBalance]);

    const fetchYearlyData = useCallback(
        async (year: number) => {
            try {
                setLoadingYearBalance(true);
                setErrorYearBalance(null);

                await findYearlyBalanceByCard(toast, year, card_number);
            } catch (error) {
                setErrorYearBalance("Failed to fetch yearly balance");
            } finally {
                setLoadingYearBalance(false);
            }
        },
        [findYearlyBalanceByCard, setLoadingYearBalance, setErrorYearBalance],
    );

    const debouncedFetchYearlyData = debounce(fetchYearlyData, 1000);


    return {
        findYearlyBalanceByCard,
        yearBalance,
        loadingYearBalance,
        setLoadingYearBalance,
        setErrorYearBalance,
        yearlyBalances,
        fetchYearlyData,
        debouncedFetchYearlyData
    }
}