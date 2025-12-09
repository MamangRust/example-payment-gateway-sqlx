import { debounce } from "@/helpers/debounce";
import useTransactionStore from "@/store/transaction/transaction";
import { useCallback, useMemo } from "react";

export default function useYearAmountTransaction(toast: any) {
    const {
        findYearTransactionAmount,
        yearTransactionAmount,
        loadingYearTransactionAmount,
        setLoadingYearTransactionAmount,
        setErrorYearTransactionAmount,
    } = useTransactionStore();

    const yearlyAmount = useMemo(() => {
        if (!yearTransactionAmount || !Array.isArray(yearTransactionAmount)) {
            return Array(5).fill(0);
        }

        const balanceMap = new Map<number, number>();
        yearTransactionAmount.forEach((balance) => {
            balanceMap.set(Number(balance.year), balance.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => balanceMap.get(year) || 0);
    }, [yearTransactionAmount]);

    const fetchYearlyAmount = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTransactionAmount(true);
                setErrorYearTransactionAmount(null);

                await findYearTransactionAmount(toast, year);
            } catch (error) {
                setErrorYearTransactionAmount("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTransactionAmount(false);
            }
        },
        [findYearTransactionAmount, setLoadingYearTransactionAmount, setErrorYearTransactionAmount],
    );

    const debouncedFetchYearlyAmount = debounce(fetchYearlyAmount, 300);

    return{
        findYearTransactionAmount,
        yearTransactionAmount,
        loadingYearTransactionAmount,
        setLoadingYearTransactionAmount,
        setErrorYearTransactionAmount,
        yearlyAmount,
        fetchYearlyAmount,
        debouncedFetchYearlyAmount
    }
}