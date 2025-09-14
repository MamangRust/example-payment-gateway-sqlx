import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useMonthTransferSender(toast: any) {
    const {
        findMonthlyTransferSenderAmount,
        monthTransferSender,
        loadingMonthTransferSender,
        setLoadingMonthTransferSender,
        setErrorMonthTransferSender,
    } = useCardStore();

    const monthlyTransfersSender = useMemo(() => {
        if (!monthTransferSender || !Array.isArray(monthTransferSender)) {
            return Array(12).fill(0);
        }

        const transfers = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthTransferSender.forEach((transfer) => {
            const monthIndex = new Date(`${transfer.month} 1, ${year}`).getMonth();
            if (transfer.total_amount !== undefined) {
                transfers[monthIndex] = transfer.total_amount;
            }
        });

        return transfers;
    }, [monthTransferSender]);


    const fetchMonthlyTransfersSender = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthTransferSender(true);
                setErrorMonthTransferSender(null);

                await findMonthlyTransferSenderAmount(toast, year);
            } catch (error) {
                console.error("Failed to fetch monthly transfers:", error);
                setErrorMonthTransferSender("Failed to fetch monthly transfers");
            } finally {
                setLoadingMonthTransferSender(false);
            }
        },
        [
            findMonthlyTransferSenderAmount,
            setLoadingMonthTransferSender,
            setErrorMonthTransferSender,
        ],
    );

    const debouncedFetchMonthlyTransfersSender = debounce(
        fetchMonthlyTransfersSender,
        1000,
    );

    return{
        findMonthlyTransferSenderAmount,
        monthTransferSender,
        loadingMonthTransferSender,
        setLoadingMonthTransferSender,
        setErrorMonthTransferSender,
        monthlyTransfersSender,
        fetchMonthlyTransfersSender,
        debouncedFetchMonthlyTransfersSender
    }
}