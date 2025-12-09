import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";

export default function useYearTransactionCard(toast: any) {
    const {
        findYearlyTransactionAmount,
        yearTransactionAmount,
        loadingYearTransaction,
        setLoadingYearTransaction,
        setErrorYearTransaction,
    } = useCardStore();

    const yearlyTransactions = useMemo(() => {
        if (!yearTransactionAmount || !Array.isArray(yearTransactionAmount)) {
            return Array(5).fill(0);
        }

        const transactionMap = new Map<number, number>();
        yearTransactionAmount.forEach((transaction) => {
            transactionMap.set(Number(transaction.year), transaction.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => transactionMap.get(year) || 0);
    }, [yearTransactionAmount]);

    const fetchYearlyTransactions = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTransaction(true);
                setErrorYearTransaction(null);

                await findYearlyTransactionAmount(toast, year);
            } catch (error) {
                console.error("Failed to fetch yearly transactions:", error);
                setErrorYearTransaction("Failed to fetch yearly transactions");
            } finally {
                setLoadingYearTransaction(false);
            }
        },
        [
            findYearlyTransactionAmount,
            setLoadingYearTransaction,
            setErrorYearTransaction,
        ],
    );

    const debouncedFetchYearlyTransactions = debounce(
        fetchYearlyTransactions,
        1000,
    );

    return{
        findYearlyTransactionAmount,
        yearTransactionAmount,
        loadingYearTransaction,
        setLoadingYearTransaction,
        setErrorYearTransaction,
        yearlyTransactions,
        fetchYearlyTransactions,
        debouncedFetchYearlyTransactions
    }

}