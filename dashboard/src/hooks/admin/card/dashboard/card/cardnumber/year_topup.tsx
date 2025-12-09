import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useYearTopupCard(card_number: string,toast: any) {
    const {
        findYearlyTopupAmountByCard,
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

                await findYearlyTopupAmountByCard(toast, year, card_number);
            } catch (error) {
                console.error("Failed to fetch yearly topups:", error);
                setErrorYearTopupAmount("Failed to fetch yearly topups");
            } finally {
                setLoadingYearTopupAmount(false);
            }
        },
        [findYearlyTopupAmountByCard, setLoadingYearTopupAmount, setErrorYearTopupAmount],
    );

    const debouncedFetchYearlyTopups = debounce(fetchYearlyTopups, 1000);

    return {
        findYearlyTopupAmountByCard,
        yearTopupAmount,
        loadingYearTopupAmount,
        setLoadingYearTopupAmount,
        setErrorYearTopupAmount,
        yearlyTopups,
        fetchYearlyTopups,
        debouncedFetchYearlyTopups
    }
}