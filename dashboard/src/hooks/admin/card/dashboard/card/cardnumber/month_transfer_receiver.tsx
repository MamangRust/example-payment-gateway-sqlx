import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useMonthTransferReceiver(card_number: string,toast: any) {
    const {
        findMonthlyTransferReceiverAmountByCard,
        monthTransferReceiver,
        loadingMonthTransferReceiver,
        setLoadingMonthTransferReceiver,
        setErrorMonthTransferReceiver,
    } = useCardStore();

    const monthlyTransfersReceiver = useMemo(() => {
        if (!monthTransferReceiver || !Array.isArray(monthTransferReceiver)) {
            return Array(12).fill(0);
        }

        const transfers = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthTransferReceiver.forEach((transfer) => {
            const monthIndex = new Date(`${transfer.month} 1, ${year}`).getMonth();
            if (transfer.total_amount !== undefined) {
                transfers[monthIndex] = transfer.total_amount;
            }
        });

        return transfers;
    }, [monthTransferReceiver]);

    const fetchMonthlyTransfersReceiver = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthTransferReceiver(true);
                setErrorMonthTransferReceiver(null);

                await findMonthlyTransferReceiverAmountByCard(toast, year,card_number);
            } catch (error) {
                console.error("Failed to fetch monthly transfers:", error);
                setErrorMonthTransferReceiver("Failed to fetch monthly transfers");
            } finally {
                setLoadingMonthTransferReceiver(false);
            }
        },
        [
            findMonthlyTransferReceiverAmountByCard,
            setLoadingMonthTransferReceiver,
            setErrorMonthTransferReceiver,
        ],
    );

    const debouncedFetchMonthlyTransfersReceiver = debounce(
        fetchMonthlyTransfersReceiver,
        1000,
    );

    return {
        findMonthlyTransferReceiverAmountByCard,
        monthTransferReceiver,
        loadingMonthTransferReceiver,
        setLoadingMonthTransferReceiver,
        setErrorMonthTransferReceiver,
        monthlyTransfersReceiver,
        fetchMonthlyTransfersReceiver,
        debouncedFetchMonthlyTransfersReceiver
    }
}