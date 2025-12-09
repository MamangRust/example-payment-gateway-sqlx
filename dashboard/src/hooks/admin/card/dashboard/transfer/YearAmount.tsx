import { debounce } from "@/helpers/debounce";
import useTransferStore from "@/store/transfer/transfer";
import { useCallback, useMemo } from "react";

export default function useYearAmountTransfer(toast: any) {
    const {
        findYearTransferAmount,
        yearTransferAmount,
        loadingYearTransferAmount,
        setLoadingYearTransferAmount,
        setErrorYearTransferAmount,
    } = useTransferStore();

    const yearlyAmount = useMemo(() => {
        if (!yearTransferAmount || !Array.isArray(yearTransferAmount)) {
            return Array(5).fill(0);
        }

        const balanceMap = new Map<number, number>();
        yearTransferAmount.forEach((balance) => {
            balanceMap.set(Number(balance.year), balance.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => balanceMap.get(year) || 0);
    }, [yearTransferAmount]);

    const fetchYearlyAmount = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTransferAmount(true);
                setErrorYearTransferAmount(null);

                await findYearTransferAmount(toast, year);
            } catch (error) {
                setErrorYearTransferAmount("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTransferAmount(false);
            }
        },
        [findYearTransferAmount, setLoadingYearTransferAmount, setErrorYearTransferAmount],
    );

    const debouncedFetchYearlyAmount = debounce(fetchYearlyAmount, 300);

    return{
        findYearTransferAmount,
        yearTransferAmount,
        loadingYearTransferAmount,
        setLoadingYearTransferAmount,
        setErrorYearTransferAmount,
        yearlyAmount,
        fetchYearlyAmount,
        debouncedFetchYearlyAmount
    }
}