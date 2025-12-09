import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useYearWithdrawCard(card_number: string,toast: any) {
    const {
        findYearlyWithdrawAmountByCard,
        yearWithdrawAmount,
        loadingYearWithdrawAmount,
        setLoadingYearWithdrawAmount,
        setErrorYearWithdrawAmount,
    } = useCardStore()

    const yearlyWithdraws = useMemo(() => {
        if (!yearWithdrawAmount || !Array.isArray(yearWithdrawAmount)) {
            return Array(5).fill(0);
        }

        const withdrawMap = new Map<number, number>();
        yearWithdrawAmount.forEach((withdraw) => {
            withdrawMap.set(Number(withdraw.year), withdraw.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => withdrawMap.get(year) || 0);
    }, [yearWithdrawAmount]);

    const fetchYearlyWithdraws = useCallback(
        async (year: number) => {
            try {
                setLoadingYearWithdrawAmount(true);
                setErrorYearWithdrawAmount(null);

                await findYearlyWithdrawAmountByCard(toast, year, card_number);
            } catch (error) {
                console.error("Failed to fetch yearly withdraws:", error);
                setErrorYearWithdrawAmount("Failed to fetch yearly withdraws");
            } finally {
                setLoadingYearWithdrawAmount(false);
            }
        },
        [
            findYearlyWithdrawAmountByCard,
            setLoadingYearWithdrawAmount,
            setErrorYearWithdrawAmount,
        ],
    );

    const debouncedFetchYearlyWithdraws = debounce(
        fetchYearlyWithdraws,
        1000,
    );

    return {
        findYearlyWithdrawAmountByCard,
        yearWithdrawAmount,
        loadingYearWithdrawAmount,
        setLoadingYearWithdrawAmount,
        setErrorYearWithdrawAmount,
        yearlyWithdraws,
        fetchYearlyWithdraws,
        debouncedFetchYearlyWithdraws
    }

}