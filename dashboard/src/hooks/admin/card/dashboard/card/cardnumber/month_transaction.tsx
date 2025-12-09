import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useMonthTransactionCard(card_number: string, toast: any) {
    const {
        findMonthlyTransactionAmountByCard,
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

                await findMonthlyTransactionAmountByCard(
                    toast,
                    year,
                    card_number as string,
                );
            } catch (error) {
                console.error("Failed to fetch monthly transactions:", error);
                setErrorMonthTransaction("Failed to fetch monthly transactions");
            } finally {
                setLoadingMonthTransaction(false);
            }
        },
        [
            findMonthlyTransactionAmountByCard,
            setLoadingMonthTransaction,
            setErrorMonthTransaction,
        ],
    );

    const debouncedFetchMonthlyTransactions = debounce(
        fetchMonthlyTransactions,
        1000,
    );

    return {
        findMonthlyTransactionAmountByCard,
        monthTransactionAmount,
        loadingMonthTransaction,
        setLoadingMonthTransaction,
        setErrorMonthTransaction,
        monthlyTransactions,
        fetchMonthlyTransactions,
        debouncedFetchMonthlyTransactions
    }
}