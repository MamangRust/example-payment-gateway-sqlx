import { debounce } from "@/helpers/debounce";
import useSaldoStore from "@/store/saldo/saldo";
import { useCallback, useMemo } from "react";


export default function useYearBalanceSaldo(toast: any) {
    const {
        yearBalance,
        loadingYearBalance,
        findYearBalance,
        setLoadingYearBalance,
        setErrorYearBalance,
    } = useSaldoStore();

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

        return years.map((year) => balanceMap.get(year) || 0);
    }, [yearBalance]);

    const fetchYearlyData = useCallback(
        async (year: number) => {
            try {
                setLoadingYearBalance(true);
                setErrorYearBalance(null);

                await findYearBalance(toast, year);
            } catch (error) {
                setErrorYearBalance("Failed to fetch yearly balance");
            } finally {
                setLoadingYearBalance(false);
            }
        },
        [findYearBalance, setLoadingYearBalance, setErrorYearBalance],
    );

    const debouncedFetchYearlyData = debounce(fetchYearlyData, 300);

    return{
        yearlyBalances,
        loadingYearBalance,
        fetchYearlyData,
        debouncedFetchYearlyData
    }
}