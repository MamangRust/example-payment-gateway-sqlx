import { debounce } from "@/helpers/debounce";
import useTransferStore from "@/store/transfer/transfer";
import { useCallback, useMemo } from "react";

export default function useMonthAmountTransferReceiver(toast: any) {
    const {
        findMonthTransferAmountByReceiver,
        monthTransferAmountReceiver,
        loadingMonthTransferAmountReceiver,
        setLoadingMonthTransferAmountReceiver,
        setErrorMonthTransferAmountReceiver,
    } = useTransferStore();

    const monthlyAmountReceiver = useMemo(() => {
        if (!monthTransferAmountReceiver || !Array.isArray(monthTransferAmountReceiver)) {
            return Array(12).fill(0);
        }

        const balances = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthTransferAmountReceiver.forEach((balance) => {
            const monthIndex = new Date(`${balance.month} 1, ${year}`).getMonth();
            if (balance.total_amount !== undefined) {
                balances[monthIndex] = balance.total_amount;
            }
        });

        return balances;
    }, [monthTransferAmountReceiver]);

    const fetchMonthlyAmount = useCallback(
        async (year: number,card_number: string) => {
            try {
                setLoadingMonthTransferAmountReceiver(true);
                setErrorMonthTransferAmountReceiver(null);

                await findMonthTransferAmountByReceiver(toast, year,card_number);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthTransferAmountReceiver("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthTransferAmountReceiver(false);
            }
        },
        [
            findMonthTransferAmountByReceiver,
            setLoadingMonthTransferAmountReceiver,
            setErrorMonthTransferAmountReceiver,
        ],
    );

    const debouncedFetchMonthlyAmountReceiver = debounce(fetchMonthlyAmount, 300);

    return{
        findMonthTransferAmountByReceiver,
        monthTransferAmountReceiver,
        loadingMonthTransferAmountReceiver,
        setLoadingMonthTransferAmountReceiver,
        setErrorMonthTransferAmountReceiver,
        monthlyAmountReceiver,
        fetchMonthlyAmount,
        debouncedFetchMonthlyAmountReceiver
    }
}