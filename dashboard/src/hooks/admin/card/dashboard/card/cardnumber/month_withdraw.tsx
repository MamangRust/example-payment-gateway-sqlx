import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useMonthWithdrawCard(card_number: string,toast: any) {
    const {
        findMonthlyWithdrawAmountByCard,
        monthWithdrawAmount,
        loadingMonthWithdrawAmount,
        setLoadingMonthWithdrawAmount,
        setErrorMonthWithdrawAmount,
    } = useCardStore();

    const monthlyWithdraws = useMemo(() => {
        if (!monthWithdrawAmount || !Array.isArray(monthWithdrawAmount)) {
            return Array(12).fill(0);
        }

        const withdraws = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthWithdrawAmount.forEach((withdraw) => {
            const monthIndex = new Date(`${withdraw.month} 1, ${year}`).getMonth();
            if (withdraw.total_amount !== undefined) {
                withdraws[monthIndex] = withdraw.total_amount;
            }
        });

        return withdraws;
    }, [monthWithdrawAmount]);

    const fetchMonthlyWithdraws = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthWithdrawAmount(true);
                setErrorMonthWithdrawAmount(null);

                await findMonthlyWithdrawAmountByCard(toast, year, card_number);
            } catch (error) {
                console.error("Failed to fetch monthly withdraws:", error);
                setErrorMonthWithdrawAmount("Failed to fetch monthly withdraws");
            } finally {
                setLoadingMonthWithdrawAmount(false);
            }
        },
        [
            findMonthlyWithdrawAmountByCard,
            setLoadingMonthWithdrawAmount,
            setErrorMonthWithdrawAmount,
        ],
    );

    const debouncedFetchMonthlyWithdraws = debounce(fetchMonthlyWithdraws, 1000);

    return {
        findMonthlyWithdrawAmountByCard,
        monthWithdrawAmount,
        loadingMonthWithdrawAmount,
        setLoadingMonthWithdrawAmount,
        setErrorMonthWithdrawAmount,
        monthlyWithdraws,
        fetchMonthlyWithdraws,
        debouncedFetchMonthlyWithdraws
    }
}