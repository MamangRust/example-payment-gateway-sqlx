import { debounce } from "@/helpers/debounce";
import useTransferStore from "@/store/transfer/transfer";
import { useCallback, useMemo } from "react";

export default function useYearAmountTransferSender(toast: any) {
    const {
        findYearTransferAmountBySender,
        yearTransferAmountSender,
        loadingYearTransferAmountSender,
        setLoadingYearTransferAmountSender,
        setErrorYearTransferAmountSender,
    } = useTransferStore();

    const yearlyAmountSender = useMemo(() => {
        if (!yearTransferAmountSender || !Array.isArray(yearTransferAmountSender)) {
            return Array(5).fill(0);
        }

        const balanceMap = new Map<number, number>();
        yearTransferAmountSender.forEach((balance) => {
            balanceMap.set(Number(balance.year), balance.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => balanceMap.get(year) || 0);
    }, [yearTransferAmountSender]);

    const fetchYearlyAmount = useCallback(
        async (year: number,card_number: string) => {
            try {
                setLoadingYearTransferAmountSender(true);
                setErrorYearTransferAmountSender(null);

                await findYearTransferAmountBySender(toast, year, card_number);
            } catch (error) {
                setErrorYearTransferAmountSender("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTransferAmountSender(false);
            }
        },
        [findYearTransferAmountBySender, setLoadingYearTransferAmountSender, setErrorYearTransferAmountSender],
    );

    const debouncedFetchYearlyAmountSender = debounce(fetchYearlyAmount, 300);

    return{
        findYearTransferAmountBySender,
        yearTransferAmountSender,
        loadingYearTransferAmountSender,
        setLoadingYearTransferAmountSender,
        setErrorYearTransferAmountSender,
        yearlyAmountSender,
        fetchYearlyAmount,
        debouncedFetchYearlyAmountSender
    }
}