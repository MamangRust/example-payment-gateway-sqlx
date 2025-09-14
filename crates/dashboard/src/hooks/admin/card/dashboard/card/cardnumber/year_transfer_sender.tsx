import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";

export default function useYearTransferSender(card_number: string,toast: any) {
    const {
        findYearlyTransferSenderAmountByCard,
        yearTransferSender,
        loadingYearTransferSender,
        setLoadingYearTransferSender,
        setErrorYearTransferSender,
    } = useCardStore();

    const yearlyTransfersSender = useMemo(() => {
        if (!yearTransferSender || !Array.isArray(yearTransferSender)) {
            return Array(5).fill(0);
        }

        const transferMap = new Map<number, number>();
        yearTransferSender.forEach((transfer) => {
            transferMap.set(Number(transfer.year), transfer.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => transferMap.get(year) || 0);
    }, [yearTransferSender]);

    const fetchYearlyTransfersSender = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTransferSender(true);
                setErrorYearTransferSender(null);

                await findYearlyTransferSenderAmountByCard(toast, year, card_number);
            } catch (error) {
                console.error("Failed to fetch yearly transfers:", error);
                setErrorYearTransferSender("Failed to fetch yearly transfers");
            } finally {
                setLoadingYearTransferSender(false);
            }
        },
        [
            findYearlyTransferSenderAmountByCard,
            setLoadingYearTransferSender,
            setErrorYearTransferSender,
        ],
    );

    const debouncedFetchYearlyTransfersSender = debounce(
        fetchYearlyTransfersSender,
        1000,
    );

    return {
        findYearlyTransferSenderAmountByCard,
        yearTransferSender,
        loadingYearTransferSender,
        setLoadingYearTransferSender,
        setErrorYearTransferSender,
        yearlyTransfersSender,
        fetchYearlyTransfersSender,
        debouncedFetchYearlyTransfersSender
    }
}