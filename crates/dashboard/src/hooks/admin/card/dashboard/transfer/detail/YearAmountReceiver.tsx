import { debounce } from "@/helpers/debounce";
import useTransferStore from "@/store/transfer/transfer";
import { useCallback, useMemo } from "react";

export default function useYearAmountTransferReceiver(toast: any) {
    const {
        findYearTransferAmountByReceiver,
        yearTransferAmountReceiver,
        loadingYearTransferAmountReceiver,
        setLoadingYearTransferAmountReceiver,
        setErrorYearTransferAmountReceiver
    } = useTransferStore();

    const yearlyAmountReceiver = useMemo(() => {
        if (!yearTransferAmountReceiver || !Array.isArray(yearTransferAmountReceiver)) {
            return Array(5).fill(0);
        }

        const balanceMap = new Map<number, number>();
        yearTransferAmountReceiver.forEach((balance) => {
            balanceMap.set(Number(balance.year), balance.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => balanceMap.get(year) || 0);
    }, [yearTransferAmountReceiver]);

    const fetchYearlyAmount = useCallback(
        async (year: number,card_number: string) => {
            try {
                setLoadingYearTransferAmountReceiver(true);
                setErrorYearTransferAmountReceiver(null);

                await findYearTransferAmountByReceiver(toast, year,card_number);
            } catch (error) {
                setErrorYearTransferAmountReceiver("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTransferAmountReceiver(false);
            }
        },
        [findYearTransferAmountByReceiver, setLoadingYearTransferAmountReceiver, setErrorYearTransferAmountReceiver],
    );

    const debouncedFetchYearlyAmountReceiver = debounce(fetchYearlyAmount, 300);

    return{
        findYearTransferAmountByReceiver,
        yearTransferAmountReceiver,
        loadingYearTransferAmountReceiver,
        setLoadingYearTransferAmountReceiver,
        setErrorYearTransferAmountReceiver,
        yearlyAmountReceiver,
        fetchYearlyAmount,
        debouncedFetchYearlyAmountReceiver
    }
}