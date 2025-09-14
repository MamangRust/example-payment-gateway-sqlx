import { debounce } from "@/helpers/debounce";
import useTransferStore from "@/store/transfer/transfer";
import { useCallback, useMemo } from "react";

export default function useMonthAmountTransferSender(toast: any) {
    const {
        findMonthTransferAmountBySender,
        monthTransferAmountSender,
        loadingMonthTransferAmountSender,
        setLoadingMonthTransferAmountSender,
        setErrorMonthTransferAmountSender,
    } = useTransferStore();

    const monthlyAmountSender = useMemo(() => {
        if (!monthTransferAmountSender || !Array.isArray(monthTransferAmountSender)) {
            return Array(12).fill(0);
        }

        const balances = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthTransferAmountSender.forEach((balance) => {
            const monthIndex = new Date(`${balance.month} 1, ${year}`).getMonth();
            if (balance.total_amount !== undefined) {
                balances[monthIndex] = balance.total_amount;
            }
        });

        return balances;
    }, [monthTransferAmountSender]);

    const fetchMonthlyAmount = useCallback(
        async (year: number,card_number: string) => {
            try {
                setLoadingMonthTransferAmountSender(true);
                setErrorMonthTransferAmountSender(null);

                await findMonthTransferAmountBySender(toast, year, card_number);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthTransferAmountSender("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthTransferAmountSender(false);
            }
        },
        [
            findMonthTransferAmountBySender,
            setLoadingMonthTransferAmountSender,
            setErrorMonthTransferAmountSender,
        ],
    );

    const debouncedFetchMonthlyAmountSender = debounce(fetchMonthlyAmount, 300);

    return{
        findMonthTransferAmountBySender,
        monthTransferAmountSender,
        loadingMonthTransferAmountSender,
        setLoadingMonthTransferAmountSender,
        setErrorMonthTransferAmountSender,
        monthlyAmountSender,
        fetchMonthlyAmount,
        debouncedFetchMonthlyAmountSender
    }
}