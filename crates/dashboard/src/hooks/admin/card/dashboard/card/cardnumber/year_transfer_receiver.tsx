import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useYearTransferReceiver(card_number: string,toast: any) {
    const {
        findYearlyTransferReceiverAmountByCard,
        yearTransferReceiver,
        loadingYearTransferReceiver,
        setErrorYearTransferReceiver,
        setLoadingYearTransferReceiver,
    } = useCardStore();

    const yearlyTransfersReceiver = useMemo(() => {
        if (!yearTransferReceiver || !Array.isArray(yearTransferReceiver)) {
            return Array(5).fill(0);
        }

        const transferMap = new Map<number, number>();
        yearTransferReceiver.forEach((transfer) => {
            transferMap.set(Number(transfer.year), transfer.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => transferMap.get(year) || 0);
    }, [yearTransferReceiver]);

    const fetchYearlyTransfersReceiver = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTransferReceiver(true);
                setErrorYearTransferReceiver(null);

                await findYearlyTransferReceiverAmountByCard(toast, year, card_number);
            } catch (error) {
                console.error("Failed to fetch yearly transfers:", error);
                setErrorYearTransferReceiver("Failed to fetch yearly transfers");
            } finally {
                setLoadingYearTransferReceiver(false);
            }
        },
        [
            findYearlyTransferReceiverAmountByCard,
            setLoadingYearTransferReceiver,
            setErrorYearTransferReceiver,
        ],
    );

    const debouncedFetchYearlyTransfersReceiver = debounce(
        fetchYearlyTransfersReceiver,
        1000,
    );

    return{
        findYearlyTransferReceiverAmountByCard,
        yearTransferReceiver,
        loadingYearTransferReceiver,
        setErrorYearTransferReceiver,
        setLoadingYearTransferReceiver,
        yearlyTransfersReceiver,
        fetchYearlyTransfersReceiver,
        debouncedFetchYearlyTransfersReceiver
    }
}