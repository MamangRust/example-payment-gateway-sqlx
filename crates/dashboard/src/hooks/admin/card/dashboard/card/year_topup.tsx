import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useYearTopupCard(toast: any) {
    const {
        findYearTopupAmount,
        yearTopupAmount,
        loadingYearTopupAmount,
        setLoadingYearTopupAmount,
        setErrorYearTopupAmount,
    } = useCardStore();

    const yearlyTopups = useMemo(() => {
        if (!yearTopupAmount || !Array.isArray(yearTopupAmount)) {
            return Array(5).fill(0);
        }

        const topupMap = new Map<number, number>();
        yearTopupAmount.forEach((topup) => {
            topupMap.set(Number(topup.year), topup.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => topupMap.get(year) || 0);
    }, [yearTopupAmount]);


    const fetchYearlyTopups = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTopupAmount(true);
                setErrorYearTopupAmount(null);

                await findYearTopupAmount(toast, year);
            } catch (error) {
                console.error("Failed to fetch yearly topups:", error);
                setErrorYearTopupAmount("Failed to fetch yearly topups");
            } finally {
                setLoadingYearTopupAmount(false);
            }
        },
        [findYearTopupAmount, setLoadingYearTopupAmount, setErrorYearTopupAmount],
    );

    const debouncedFetchYearlyTopups = debounce(fetchYearlyTopups, 1000);

    return {
        findYearTopupAmount,
        yearTopupAmount,
        loadingYearTopupAmount,
        setLoadingYearTopupAmount,
        setErrorYearTopupAmount,
        yearlyTopups,
        fetchYearlyTopups,
        debouncedFetchYearlyTopups
    }
}