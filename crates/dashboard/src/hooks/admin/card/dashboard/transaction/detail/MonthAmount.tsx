import { debounce } from "@/helpers/debounce";
import useTransactionStore from "@/store/transaction/transaction";
import { useCallback, useMemo } from "react";

export default function useMonthAmountTransactionDetail(toast: any) {
    const {
        findMonthTransactionAmountCard,
        monthTransactionAmount,
        loadingMonthTransactionAmount,
        setLoadingMonthTransactionAmount,
        setErrorMonthTransactionAmount,
    } = useTransactionStore();

    const monthlyAmount = useMemo(() => {
        if (!monthTransactionAmount || !Array.isArray(monthTransactionAmount)) {
            return Array(12).fill(0);
        }

        const balances = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthTransactionAmount.forEach((balance) => {
            const monthIndex = new Date(`${balance.month} 1, ${year}`).getMonth();
            if (balance.total_amount !== undefined) {
                balances[monthIndex] = balance.total_amount;
            }
        });

        return balances;
    }, [monthTransactionAmount]);

    const fetchMonthlyAmount = useCallback(
        async (year: number,card_number: string) => {
            try {
                setLoadingMonthTransactionAmount(true);
                setErrorMonthTransactionAmount(null);

                await findMonthTransactionAmountCard(toast, year, card_number);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthTransactionAmount("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthTransactionAmount(false);
            }
        },
        [
            findMonthTransactionAmountCard,
            setLoadingMonthTransactionAmount,
            setErrorMonthTransactionAmount,
        ],
    );

    const debouncedFetchMonthlyAmount = debounce(fetchMonthlyAmount, 300);

    return{
        findMonthTransactionAmountCard,
        monthTransactionAmount,
        loadingMonthTransactionAmount,
        setLoadingMonthTransactionAmount,
        setErrorMonthTransactionAmount,
        monthlyAmount,
        fetchMonthlyAmount,
        debouncedFetchMonthlyAmount
    }
}