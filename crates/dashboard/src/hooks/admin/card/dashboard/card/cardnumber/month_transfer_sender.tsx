import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useMonthTransferSender(card_number: string,toast: any) {
    const {
        findMonthlyTransferSenderAmountByCard,
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

                await findMonthlyTransferSenderAmountByCard(toast, year, card_number);
            } catch (error) {
                console.error("Failed to fetch monthly transfers:", error);
                setErrorMonthTransferSender("Failed to fetch monthly transfers");
            } finally {
                setLoadingMonthTransferSender(false);
            }
        },
        [
            findMonthlyTransferSenderAmountByCard,
            setLoadingMonthTransferSender,
            setErrorMonthTransferSender,
        ],
    );

    const debouncedFetchMonthlyTransfersSender = debounce(
        fetchMonthlyTransfersSender,
        1000,
    );

    return{
        findMonthlyTransferSenderAmountByCard,
        monthTransferSender,
        loadingMonthTransferSender,
        setLoadingMonthTransferSender,
        setErrorMonthTransferSender,
        monthlyTransfersSender,
        fetchMonthlyTransfersSender,
        debouncedFetchMonthlyTransfersSender
    }
}