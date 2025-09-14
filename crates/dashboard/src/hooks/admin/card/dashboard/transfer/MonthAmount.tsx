import { debounce } from "@/helpers/debounce";
import useTransferStore from "@/store/transfer/transfer";
import { useCallback, useMemo } from "react";

export default function useMonthAmountTransfer(toast: any) {
    const {
        findMonthTransferAmount,
        monthTransferAmount,
        loadingMonthTransferAmount,
        setLoadingMonthTransferAmount,
        setErrorMonthTransferAmount,
    } = useTransferStore();

    const monthlyAmount = useMemo(() => {
        if (!monthTransferAmount || !Array.isArray(monthTransferAmount)) {
            return Array(12).fill(0);
        }

        const balances = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthTransferAmount.forEach((balance) => {
            const monthIndex = new Date(`${balance.month} 1, ${year}`).getMonth();
            if (balance.total_amount !== undefined) {
                balances[monthIndex] = balance.total_amount;
            }
        });

        return balances;
    }, [monthTransferAmount]);

    const fetchMonthlyAmount = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthTransferAmount(true);
                setErrorMonthTransferAmount(null);

                await findMonthTransferAmount(toast, year);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthTransferAmount("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthTransferAmount(false);
            }
        },
        [
            findMonthTransferAmount,
            setLoadingMonthTransferAmount,
            setErrorMonthTransferAmount,
        ],
    );

    const debouncedFetchMonthlyAmount = debounce(fetchMonthlyAmount, 300);

    return{
        findMonthTransferAmount,
        monthTransferAmount,
        loadingMonthTransferAmount,
        setLoadingMonthTransferAmount,
        setErrorMonthTransferAmount,
        monthlyAmount,
        fetchMonthlyAmount,
        debouncedFetchMonthlyAmount
    }
}