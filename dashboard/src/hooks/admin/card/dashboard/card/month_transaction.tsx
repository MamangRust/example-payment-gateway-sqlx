import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useMonthTransactionCard(toast: any) {
    const {
        findMonthlyTransactionAmount,
        monthTransactionAmount,
        loadingMonthTransaction,
        setLoadingMonthTransaction,
        setErrorMonthTransaction,
    } = useCardStore();

    const monthlyTransactions = useMemo(() => {
        if (!monthTransactionAmount || !Array.isArray(monthTransactionAmount)) {
            return Array(12).fill(0);
        }

        const transactions = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthTransactionAmount.forEach((transaction) => {
            const monthIndex = new Date(`${transaction.month} 1, ${year}`).getMonth();
            if (transaction.total_amount !== undefined) {
                transactions[monthIndex] = transaction.total_amount;
            }
        });

        return transactions;
    }, [monthTransactionAmount]);

    const fetchMonthlyTransactions = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthTransaction(true);
                setErrorMonthTransaction(null);

                await findMonthlyTransactionAmount(toast, year);
            } catch (error) {
                console.error("Failed to fetch monthly transactions:", error);
                setErrorMonthTransaction("Failed to fetch monthly transactions");
            } finally {
                setLoadingMonthTransaction(false);
            }
        },
        [
            findMonthlyTransactionAmount,
            setLoadingMonthTransaction,
            setErrorMonthTransaction,
        ],
    );

    const debouncedFetchMonthlyTransactions = debounce(
        fetchMonthlyTransactions,
        1000,
    );

    return{
        findMonthlyTransactionAmount,
        monthTransactionAmount,
        loadingMonthTransaction,
        setLoadingMonthTransaction,
        setErrorMonthTransaction,
        monthlyTransactions,
        fetchMonthlyTransactions,
        debouncedFetchMonthlyTransactions
    }
}